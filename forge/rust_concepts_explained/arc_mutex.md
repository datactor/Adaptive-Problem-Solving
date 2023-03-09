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
unlock, locked no other threads waiting, locked and other threads waiting 세가지 상태 변화를
완전 실패와 완전 성공을 보장한다.
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

## Arc
Rc와 같이 값의 소유자 수를 추적하는 참조 횟수를 추적하여 다중 소유권을 허용하는 스마트 포인터이다.
차이점은 멀티 스레드 context에서 사용할 수 있는 atomic ref count pointer 라는 것이다.  
Rc는 내부의 RcBox struct에서 일반 스칼라 연산을 사용하여 증가 및 감소하는 ref counting한다.
atomic 연산이 필요하지 않은 단일 스레드 context에서만 사용될 수 있음을 의미한다.  
반면 Arc는 내부 ArcInner struct에서 atomic 연산을 사용하여 증가 및 감소하는 AtomicUsize type의 값에
저장된다. atomic 연산의 기본 순서는 Relaxed이다. 즉, 읽기 쓰기 순서가 보장되지 않지만 필요한 경우 변경할 수 있다.
이는 순서를 강제하지 않기 때문에 하드웨어가 선택한 순서대로 읽기 쓰기 작업을 수행하게 하여 성능상의 이점을 제공할
수 있지만, 버그의 여지가 있다.

counting의 완전 실패와 완전 카운팅을 보장하기 때문에 매 카운팅 순간 모두의 정확성을 보장하지는 않는다.
카운팅 도중 경합이 일어나면 실패할 것이기 때문이다.
그렇지만 서로 다른 스레드 간에 공유 메모리에 대한 엑세스를 synchronize 함으로써 결국에는 참조 카운트가 정확해지도록 보장된다.
여기서 sync의 핵심은 AtomicOrdering에 있다. AtomicOrdering은 OS단위 아토믹 연산의 읽기 쓰기 순서로,
OS단위로 아토믹 연산을 수행하기 때문에 sync가 보장된다. 그렇기 때문에 Arc는 lock조차도 필요 없어진다.

Arc는 Rust와 같은 가비지 수집 언어에서 메모리 누수로 이어질 수 있는 순환 참조를 감지하고 처리하도록 설계되었다.
Arc에서 사용하는 참조 카운팅 메커니즘은 프로그래머가 명시적으로 문제를 처리하지 않더라도 이러한 주기를 감지하고
메모리를 올바르게 할당 해제할 수 있다. 이것이 Arc가 여러 스레드 간에 데이터를 공유하는 안전하고 신뢰할 수 있는
방법으로 간주되는 이유 중 하나이다.

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