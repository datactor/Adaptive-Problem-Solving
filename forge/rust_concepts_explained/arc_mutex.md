# Arc<Mutex< T >>

## Mutex

### Mutex Rules and Usage
mutual exclusion의 줄임말로, 주어진 시간에 오직 하나의 스레드만 데이터 접근을 허용한다.
Mutex 내부에 접근하기 위해서 스레드는 먼저 Mutex lock을 얻기를 요청함으로써 신호를 보내야 한다.
락은 누군가가 배타적으로 Mutex 내부 값에 접근하는지를 추적하는 뮤텍스의 데이터 구조이다.
- mutex 사용 규칙
  1. mutex가 감싸고 있는 값을 사용하기 전에 반드시 lock을 얻는 시도를 해야 한다.
  2. mutex가 감싸고 있는 값의 사용이 끝났다면, 다른 스레드들이 lock을 얻을 수 있도록 반드시 unlock해야 한다.

lock을 얻고 난 후에는, Mutex는 RefCell과 같이 내부 가변성을 가진다. mutable reference 처럼 다룰 수 있으며,
lock을 얻으면 LockResult<MutexGuard<'_, T>>라는 Result 타입을 얻는데, 풀어서 얻은 MutexGuard 타입은
스마트 포인터이다. RefCell과 비슷하게 스코프 밖으로 벗어나거나 move될 때, 자동으로 lock을 해제하는
drop method가 구현되어 있다. 다른 점은 메모리 할당이나 할당해제를 하지 않고 drop시 오로지 unlock만 수행한다는 것이다.  
mutex가 감싸는 값을 가져오는 일은 비동기나 다중 스레드의 공유 메모리에 접근하는 상황이기 때문에, 애초에 개별적으로 값을 할당하지 않으며,
스레드 간에 공유된다. 그러므로 매번 drop할 때마다 할당해제할 개별적인 값이 없으며, 공유된 값은 모든 스레드가
참조를 해제할 때까지 공유 메모리에 남아 있는다.

```rust
impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        let result = self.mutex.unlock();
        match result {
            Ok(()) => {}
            Err(poison) => {
                let panicking = thread::panicking();
                self.poison.store(true, Release);
                if !panicking {
                    panic!("mutex poisoned: {}", poison.description());
                }
            }
        }
    }
}
```

### Atomic unlocking with Futex
Mutex type은 운영체제에서 제공하는 저수준 동기화 primitive(inner field)를 사용해
스레드간 상호 배제를 달성한다.  
여기서 우리가 실제로 다루는 Mutex의 inner type은 sys::Mutex로 이루어져 있고,
이 sys::Mutex struct는 내부에 futex(linux)라는 필드를 가지고 있는데, AtomicU32를 통해 lock의 득실을
atomic number로 나타낸다. 즉 lock을 수행할 때, 프로그램이 아닌 운영체제 단위에서 atomic ordering을 수행해

0: unlock,  
1: locked no other threads waiting,  
2: locked and other threads waiting(contended)  
세가지 상태 변화를 통해 완전 실패와 완전 성공을 보장한다.

여기서 futex를 사용하는 이유는 `compare_exchange`(compare_and_swap) 연산을 통해 0, 1, 2의 상태를 atomic하게 변경시키기 위함이다.
```rust
#[inline]
pub fn try_lock(&self) -> bool {
    self.futex.compare_exchange(0, 1, Acquire, Relaxed).is_ok()
}
```
위 부분에서 0은 current state, 1은 새롭게 바꿀 state, `Acquire` Ordering은 첫번째 인자가 0임을 확인 했을 때 동작하는 Ordering이다.
이는 뮤텍스를 잠그는 연산이 뮤텍스에 접근하는 연산보다 먼저 일어나야 함을 보장한다.  
Relaxed는 첫번째 인자가 0이 아닐 경우(즉, 뮤텍스가 이미 잠겨있다면) `Relaxed` Ordering을 통해 실패를 처리한다(실패 했을 경우 값의 변경은 없기 때문에
메모리 순서는 보장하지 않아도 된다.).

```rust
use crate::sync::atomic::{
    AtomicU32,
    Ordering::{Acquire, Relaxed, Release},
};
use crate::sys::futex::{futex_wait, futex_wake};

pub struct SysMutex {
    /// 0: unlocked
    /// 1: locked, no other threads waiting
    /// 2: locked, and other threads waiting (contended)
    futex: AtomicU32,
}

impl SysMutex {
    #[inline]
    pub const fn new() -> Self {
        Self { futex: AtomicU32::new(0) }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "Mutex")]
pub struct Mutex<T: ?Sized> {
    inner: sys::Mutex, // SysMutex
    poison: poison::Flag,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn lock(&self) -> Result<MutexGuard<T>, PoisonError<MutexGuard<T>>> {
        let mut lock = self.mutex.lock().unwrap();

        // Create a new MutexGuard
        let guard = MutexGuard {
            mutex: &self.mutex,
            data: unsafe { &mut *self.data.get() },
        };

        // Return the guard wrapped in a Result
        Ok(guard)
    }
}
```

### Low-Level Synchronization with Futex
위에서 언급한 아토믹에 더불어 sync를 통한 정확성 보장도 포함되는데, 우리가 정확성 보장을 위한 sync를 구현하지 않아도
운영체제 단위에서 수행하는 저수준 동기화 프리미티브인 `futex(fast userspace mutex)`를 불러오기 때문에
운영체제가 동기화를 보장해준다. 우리는 그저 sync::Mutex를 사용해 lock을 획득하고 해제하면 futex가 동기화 처리를 해준다.  

아래는 futex crate에서 구현한 spinlock이다(futex_wait, futex_wake 메소드 내부에서 libc::syscall을 통해 futex를 불러온다).  
* spinlock? wait 명령이 오면 lock을 획득할때까지 스레드를 계속 유지하는 loop를 돌려 다른 스레드와 경합을 방지하는 동기화 프리미티브 중 하나이다.

```rust
use crate::sys::futex::{futex_wait, futex_wake};

impl Mutex {
    // snippet
    
    #[cold]
    fn lock_contended(&self) {
        // Spin first to speed things up if the lock is released quickly.
        let mut state = self.spin();
    
        // If it's unlocked now, attempt to take the lock
        // without marking it as contended.
        if state == 0 {
            match self.futex.compare_exchange(0, 1, Acquire, Relaxed) {
                Ok(_) => return, // Locked!
                Err(s) => state = s,
            }
        }
    
        loop {
            // Put the lock in contended state.
            // We avoid an unnecessary write if it as already set to 2,
            // to be friendlier for the caches.
            if state != 2 && self.futex.swap(2, Acquire) == 0 {
                // We changed it from 0 to 2, so we just successfully locked it.
                return;
            }
    
            // Wait for the futex to change state, assuming it is still 2.
            futex_wait(&self.futex, 2, None);
    
            // Spin again after waking up.
            state = self.spin();
        }
    }
}
```
즉, 기다리는 스레드는 spinlock으로 컨텍스트를 유지시키고 사용하지 않는 스레드는 절전시켜 synchronization을 보장한다.

futex와 같은 운영체제 내부의 프리미티브는 한 번에 하나의 스레드만 잠금을 보유할 수 있도록 하여 아토믹을 구현하는 동시에,
데이터 경합을 방지하고 스레드 동기화를 통해 프로그램의 정확성을 보장한다.

```rust
if state != 2 && self.futex.swap(2, Acquire) == 0 {
    // We changed it from 0 to 2, so we just successfully locked it.
    return;
}
```
위의 코드는 futex를 spinlock을 최적화 하기 위해 구현된 breaker로
state != 2; state가 다른 스레드에 의해 대기중인 상태가 아니며,
self.futex.swap(2, Acquire) == 0; Acquire Ordering으로 futex의 값을 2로 swap 했을때, 이전의 값이 0일 경우,
락을 획득할 수 있다는 것을 의미하며 spin loop를 중단하고 락을 획득한다.
즉 이전의 futex의 값이 0일때, futex를 2로 바꾸지만, 예외적으로 중단하고 락을 획득한다.
이는 효율성을 높이고, 불필요한 spin-wait를 피하는 예외적인 최적화이다.

## Arc
Rc와 같이 값의 소유자 수를 추적하는 참조 횟수를 추적하여 다중 소유권을 허용하는 스마트 포인터이다.
strong counting, weak counting, 할당해제 규칙 등 대부분의 메카니즘은 Rc와 같다.
그러나 Arc는 멀티 스레드 context에서 사용할 수 있도록 atomic ref count pointer로 설계되었다.

Rc는 내부의 RcBox struct에서 일반 스칼라 연산을 사용하여 증가 및 감소하는 reference를 count한다.
즉, atomic 연산이 필요하지 않은 단일 스레드 context에서만 사용될 수 있음을 의미한다.  
반면 Arc는 내부 ArcInner struct에서 atomic 연산을 사용하여 증가 및 감소하는 AtomicUsize type의 값에
저장된다. atomic 연산의 기본 순서는 Relaxed이다. 즉, 읽기 쓰기 순서가 보장되지 않지만 필요한 경우 변경할 수 있다.
이는 순서를 강제하지 않기 때문에 하드웨어가 선택한 순서대로 읽기 쓰기 작업을 수행하게 하여 성능상의 이점을 제공할
수 있지만, 버그의 여지가 있다.

counting의 완전 실패와 완전 카운팅을 보장하기 때문에 매 카운팅 순간 모두의 정확성을 보장하지는 않는다.
카운팅 도중 경합이 일어나면 실패할 것이기 때문이다(또한 `Relaxed` Oredering으로 수행하기 때문. 여기서 strict한 Ordering을 선택한다면 오버헤드가 너무 커진다.).
그렇지만 서로 다른 스레드 간에 공유 메모리에 대한 엑세스를 synchronize 함으로써 결국에는 참조 카운트가 정확해지도록 보장된다.
여기서 sync의 핵심은 AtomicOrdering에 있다. AtomicOrdering은 공유 메모리에서 다른 스레드에 표시되어야 하는 순서를 지정하는 것으로,
OS레벨에서 아토믹 연산을 수행하기 때문에 sync가 보장된다. 그렇기 때문에 Arc는 lock조차도 필요 없어진다.  
즉, 동시에 여러 아토믹 연산이 수행되더라도, Atomic Ordering 덕분에 각 개별연산은 결국에는 반드시 반영된다.

```rust
// This is repr(C) to future-proof against possible field-reordering, which
// would interfere with otherwise safe [into|from]_raw() of transmutable
// inner types.
#[repr(C)]
struct ArcInner<T: ?Sized> {
    strong: atomic::AtomicUsize,

    // the value usize::MAX acts as a sentinel for temporarily "locking" the
    // ability to upgrade weak pointers or downgrade strong ones; this is used
    // to avoid races in `make_mut` and `get_mut`.
    weak: atomic::AtomicUsize,

    data: T,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Clone for Arc<T> {
  #[inline]
  fn clone(&self) -> Arc<T> {
    // Using a relaxed ordering is alright here, as knowledge of the
    // original reference prevents other threads from erroneously deleting
    // the object.
    //
    // As explained in the [Boost documentation][1], Increasing the
    // reference counter can always be done with memory_order_relaxed: New
    // references to an object can only be formed from an existing
    // reference, and passing an existing reference from one thread to
    // another must already provide any required synchronization.
    //
    // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
    let old_size = self.inner().strong.fetch_add(1, Relaxed);
  
    // However we need to guard against massive refcounts in case someone is `mem::forget`ing
    // Arcs. If we don't do this the count can overflow and users will use-after free. This
    // branch will never be taken in any realistic program. We abort because such a program is
    // incredibly degenerate, and we don't care to support it.
    //
    // This check is not 100% water-proof: we error when the refcount grows beyond `isize::MAX`.
    // But we do that check *after* having done the increment, so there is a chance here that
    // the worst already happened and we actually do overflow the `usize` counter. However, that
    // requires the counter to grow from `isize::MAX` to `usize::MAX` between the increment
    // above and the `abort` below, which seems exceedingly unlikely.
    if old_size > MAX_REFCOUNT {
      abort();
    }
  
    unsafe { Self::from_inner(self.ptr) }
  }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // Decrement the strong reference count
        let strong = Arc::<T>::strong_count(&self.ptr);
        if strong == 1 {
            // This is the last strong reference, deallocate the allocation
            unsafe {
                // Get a pointer to the ArcInner<T> struct
                let inner = self.ptr.as_ptr();

                // Decrement the reference count
                let count = ArcInner::<T>::dec_strong_count(inner);

                // If there are no more strong references or weak references,
                // deallocate the allocation
                if count.strong == 0 {
                    drop(Arc::from_raw(inner));
                }
            }
        } else {
            // There are still other strong references, so just decrement the count
            Arc::<T>::dec_strong_count(&self.ptr);
        }
    }
}
```
* AtomicOrdering
  1. Relaxed: 순서를 보장하지 않음. data race 가능성 있음.
  2. Acquire: 현재 스레드가 현재 작업 이전에 다른 스레드가 수행한 모든 메모리 작업을 관찰할 수 있는지 확인.
     즉, 다른 스레드 작업을 관찰하고 획득 하라는 것(다른 스레드가 '작업 중'일때 획득하지 말라는 것)  
     이는 스레드 간에 데이터를 동기화하여 현재 스레드가 가장 최신 버전의 데이터를 갖도록 하는 데 유용하다.
  3. Release: 현재 스레드가 수행한 작업 중일 때, 작업을 마친 이후에 다른 스레드에 의해 관찰되도록 함.
     (현재 스레드가 '작업 중'일 때 다른 스레드에서 관찰하여 획득할 수 없음) 이는 스레드 간에 데이터를 동기화해
     현재 스레드가 데이터 수정을 완료한 후 다른 스레드가 일관된 데이터 보기를 볼 수 있도록 하는데 유용하다.
  4. AcqRel: Acquire + Release 모두 제공
  5. SeqCst: 모든 메모리 작업은 모든 스레드에서 동일한 순서로 관찰한다.
     즉 모든 스레드에서, 스레드마다 작업이 수행한 이후 다음 스레드가 관찰되도록 하기 + 다른 스레드 작업을 관찰하고 획득하기
     이렇게 하면 Queue에 들어있는 것처럼, 스레드마다 순서가 정해져있는 것처럼 작동돼 순차 일관성이 보장된다.

* Relaxed로 설정된 이유 ?  
  AtomicOrdering에 대해 엄격한 순서 지정을 사용하면 모든 스레드가 동일한
  이벤트 시퀀스를 관찰하는 방식으로 메모리 작업이 정렬되도록 한다.
  즉, 각 스레드는 다른 스레드가 수행한 순서대로 모든 메모리 작업을 관찰한다.
  이는 프로그램의 올바른 동작을 보장하기 위해 보존해야 하는 작업 간에 종속성이 있을 때 필요하다.  

  반면 Relaxed 순서를 사용하면 메모리 작업의 순서가 보장되지 않는다.
  이는 스레드가 다른 스레드에서 수행한 순서를 지킬 필요없이, 다른 순서로 메모리 작업을 관찰할 수 있음을 의미한다.
  Relaxed 순서는 작업 간에 종속성이 없거나 다른 방법으로 종속성을 보장할 수 있는 경우에 유용하다.  

  여기서 참조 횟수를 늘리고 새 참조를 만드는 메모리 작업인 Arc의 Clone 구현의 경우
  원래 참조를 알고 있기 때문에 다른 방법으로 종속성을 보장할 수 있는 경우이다.
  따라서 보존해야 하는 메모리 작업의 특정 순서에 대한 요구 사항이 없기 때문에
  Relaxed 순서를 사용하는 것이 효율적이다.

  이 상황에서 엄격한 순서를 사용하면 프로그램의 올바른 동작에 필요하지 않더라도 모든 스레드가 동일하게 볼 수 있는
  일련의 메모리 작업을 수행한다. 이로 인해 성능이 저하되고 불필요한 동기화 오버헤드가 발생할 수 있다.

Arc에 대한 Clone 구현은 원래 참조에 대한 사전 정보가 다른 스레드가 개체를 잘못 삭제하는 것을
방지하기 때문에 엄격한 Ordering이 필요없어, 'Relaxed' Ordering을 사용한다.
Drop 구현은 Rc와 같이 더 이상 strong count나 weak count가 없으면
강한 참조 수를 0으로 만들고 할당을 해제한다. 여전히 strong count가 남아 있는 경우 개수를 줄인다.
강한 참조 카운트와 약한 참조 카운트는 ArcInner struct 저장되며
usize::MAX는 make_mut 및 get_mut에서 경합을 피하기 위해 약한 포인터를 업그레이드하거나
강한 포인터를 다운그레이드하는 기능을 일시적으로 "locking하는" Sentinel 역할을 한다.  
예를 들어서, 일반적인 경우에서는 UAF가 발생하지 않지만 atomic이 매순간 정확하지는 않다.
다른 스레드에서 강한 참조를 점유하고 있는 상황에서 순간적인 오류로 실제는 spare 강한 참조가 없지만, Arc에서
spare 강한 참조가 남아 있으며, 더 이상 사용되지 않는다고 잘못 가정하고 weak으로 downgrade 한다면,
이미 점유하고 있는 참조에서 UAF가 발생할 수 있다. 이 경우를 대비해 Sentinel을 세운다.

대규모 ref count 및 use-after-free 오류를 방지하기 위해 Arc는 중단 기능을 사용하여 최대 refcount를
확인한다. ref count가 너무 크면 프로그램이 중단된다.

Arc는 여러 스레드 간에 데이터를 공유하는 안전하고 신뢰할 수 있는 방법으로 간주된다.
다중 스레드 컨텍스트에서 mutable 값에 대한 안전하고 효율적인 공유 액세스를 제공하기 위해
Mutex 및 RwLock과 같은 다른 동기화 프리미티브와 함께 자주 사용된다. 이러한 동기화 프리미티브는
복잡한 데이터 구조를 만드는 데 사용할 수 있다.

전반적으로 Arc는 다중 스레드 컨텍스트에서 메모리를 관리하고 변경 가능한 데이터에 대한
안전하고 효율적인 공유 액세스를 보장하는 데 유용한 도구이다.

## Limitations of Atomic
당연하지만 atomic 연산은 단일 메모리 공간(e.g. 멀티코어 또는 멀티스레드 시스템 내부의 공유 메모리) 내에서만 보장된다.
여러 노드 간에는 각 노드는 아토믹으로 수행되더라도, 그러한 접근 자체가 독립된 환경에서 수행되기 때문에 애초에 떨어져 있는 구조이다(아토믹 자체가 성립이 안됨).
그렇기 때문에 분산 시스템에서의 일관성을 보장하려면 보통 복잡한 동기화 프로토콜이 필요하며, 이는 아토믹 연산 이상의 레벨에서 작동한다.
예를 들어 두 노드가 동일한 데이터에 대해 동시에 변경을 시도할 때 일관성을 보장하기 위해 Lock을 사용하거나, Paxos 또는 Raft와 같은 분산 합의 알고리즘을 사용할 수 있다.

따라서, 멀티노드 환경에서 Atomic 연산은 직접적으로는 유효하지 않으나, 분산 시스템을 동기화하는 데 사용되는 더 높은 수준의
프로토콜에서는 중요한 역할을 수행할 수 있다.