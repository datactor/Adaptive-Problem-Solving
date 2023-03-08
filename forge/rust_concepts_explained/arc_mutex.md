# Arc<Mutex< T >>

## Mutex
mutual exclusion의 줄임말로, 주어진 시간에 오직 하나의 스레드만 데이터 접근을 허용한다.
Mutex 내부에 접근하기 위해서 스레드는 먼저 Mutex lock을 얻기를 요청함으로써 신호를 보내야 한다.
락은 누군가가 배타적으로 Mutex 내부 값에 접근하는지를 추적하는 뮤텍스의 데이터 구조이다.

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

여기에는 아토믹에 더불어 sync를 통한 정확성 보장도 포함되는데, 우리가 정확성 보장을 위한 sync를 구현하지 않아도
운영체제 단위에서 수행하는 저수준 동기화 프리미티브인 `futex(fast userspace mutex)`를 불러오기 때문에
운영체제가 동기화를 보장해준다. 우리는 그저 syn::Mutex를 사용해 lock을 획득하고 해제하면 futex가 동기화 처리를 해준다.  

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
futex와 같은 운영체제 내부의 프리미티브는 한 번에 하나의 스레드만 잠금을 보유할 수 있도록 하여 아토믹을 구현하는 동시에,
데이터 경합을 방지하고 스레드 동기화를 통해 프로그램의 정확성을 보장한다.

## Arc
Arc는 counting을 아토믹 하게 수행해, 완전 실패와 완전 카운팅을 보장하는 것.
정확한 카운팅을 보장하지는 않는다. 카운팅 도중 경합이 일어나면 실패할 것이기 때문이다.
그렇지만 서로 다른 스레드 간에 공유 메모리에 대한 엑세스를 synchronize 함으로써
결국에는 참조 카운트가 정확하도록 보장한다.

Arc는 Rust와 같은 가비지 수집 언어에서 메모리 누수로 이어질 수 있는 순환 참조를 감지하고 처리하도록 설계되었습니다. Arc에서 사용하는 참조 카운팅 메커니즘은 프로그래머가 명시적으로 문제를 처리하지 않더라도 이러한 주기를 감지하고 메모리를 올바르게 할당 해제할 수 있습니다. 이것이 Arc가 여러 스레드 간에 데이터를 공유하는 안전하고 신뢰할 수 있는 방법으로 간주되는 이유 중 하나입니다.

Arc는 순환 참조를 자동으로 방지하지 않습니다. 프로그래머는 사이클을 수동으로 중단하거나 약한 카운트를 사용하여 여전히 적절하게 처리해야 합니다. 그러나 참조 주기가 있고 참조 횟수가 부정확하더라도 Arc는 메모리 누수 없이 메모리가 올바르게 할당 해제되도록 보장합니다. 이는 Arc가 강력한 카운팅과 약한 카운팅의 조합과 동기화 메커니즘을 사용하여 참조 카운트가 결국 정확하고 메모리가 올바르게 할당 해제되도록 하기 때문입니다.

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