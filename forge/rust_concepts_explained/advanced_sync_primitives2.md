# Unlocking the Crossbeam
Previous article: [advanced_sync_primitives1](https://github.com/datactor/Rustic-data-solving/blob/main/forge/rust_concepts_explained/advanced_sync_primitives1.md),

이전 article에서는 동기 데이터를 보호하는 근본적인 타입들인 Cell 라이브러리의 Cell, RefCell, UnsafeCell과,
여러 동기 primitives들로 구성된 sync 라이브러리의 Atomic types, Semaphore(custom), Condvar, Barrier, mpsc, Once, Mutex, RwLock와 같은
Rust의 고급 동기 primitives들에 알아보았다.

여기서는 효율적이고 안전한 동시성 프리미티브를 제공하는 Rust 라이브러리인 Crossbeam를 unlocking한다.

스레드 간에 데이터를 전송하도록 설계된 Crossbeam 채널, 공유 데이터에 대한 lock-free operation을 제공하는 Crossbeam atomic type,
계산 병렬화를 위한 강력한 메커니즘인 Crossbeam 및 Rayon을 사용한 work stealing에 대해 알아보자.

이전 기사에서 다룬 고급 동기 프리미티브와 이 기사의 다음 주제에 대한 지식을 통해 Rust의 동시성 기능을 확실하게 이해하고
효율적이고 안전한 동시 프로그램을 작성할 수 있을 것이다(있어야 한다 반드시).

## 4. Introduction to crossbeam
Crossbeam 라이브러리는 효율적이고 안전한 동시성 프리미티브를 제공하는 Rust의 external crate이다.
동시 프로그래밍을 더 쉽고 안정적으로 만들도록 설계되었다.

Crossbeam 사용의 주요 이점 중 하나는 atomic operation만을 사용하여 Lock-Free 및 Wait-Free 동기화 프리미티브를 제공한다.
즉 Mutex 및 Semaphore와 같은 기존 동기화 프리미티브보다 훨씬 효율적일 수 있다는 것이다.
기존의 Mutex는 lock과 spin-lock 두가지의 오버헤드가 존재하며, semaphore의 구현은 atomic primitives만으로 구현한다고 하더라도
대부분의 구현에서는 acquire 메서드를 구현할 때 다음과 같은 이유로 인해, spin-lock 형태로 loop를 구현한다.
1. 신호가 전송되지 않았음에도 불구하고 스레드가 OS에 의해 깨어날 수 있는 "spurious wake-up"라는 가능한 경쟁 조건을 방지하는 데 사용된다.
   - spurious wake-up(의사 각성)?  
     프로그램에서 wake-up 조건이 달성되지 않았음에도, wake-up되는 경우.  
     예를 들어 스레드는 메시지가 채널에 도착하기를 기다리고 있다고 가정해보자.
     메시지가 도착할 때까지 스레드를 절전 모드로 전환하기 위해 내부적으로 futex_wait()을 호출하는 채널의 recv() 메서드를 호출한다.
     메시지가 채널에 도착하면 채널은 대기 중인 스레드를 깨우는 신호를 보낸다.  
     여기서 메시지가 OS에는 도착하고, 러스트 프로그램 내의 채널에 도착하기 직전이라고 생각해보자. 
     이 경우 OS에서는 '정상적으로' 대기 중인 스레드를 깨운다. 그렇지만 Rust 프로그램의 채널에는 아직 message가 도착하지 않았다.
     프로그램 내에서는 조건이 충족되지 않았음에도 스레드가 깨어났기 때문에 spurious wake-up이라고 불린다.
     위의 경우 외에도 다양한 상황에서 spurious wake-up이 발생할 수 있다.
     Rust의 동기 프리미티브는 스레드 스케줄링 및 동기화를 처리하기 위해 OS에 의존한다. 결과적으로 이러한 프리미티브의 동작은 OS의 동작에 영향을 받을 수 있다.
   예상하지 못한 spurious wake-up이 발생할 경우를 대비하여 loop분기를 통해 wake-up 조건을 지속적으로 확인하고 조건이 맞지 않으면
   continue를 통해 재시도한다. 이를 통해 스레드는 스레드를 획득하려고 시도하기 전에 실제로 조건이 맞을 때까지 대기하여 프로그램에서 잘못된 동작을 방지한다.

2. backoff strategy
   스레드가 기다리지 않고 지속적으로 세마포어 획득을 시도하면 과도한 경합 및 성능 저하가 발생할 수 있다.
   loop분기를 사용하여 스레드가 재시도하기 전에 일정 시간 동안 대기할 수 있으므로 경합이 줄어들고 성능을 향상시킬 가능성이 있다.

3. 다른 스레드에서 변수에 대한 동시 업데이트 가능성을 처리하기 위해서이다.
   loop분기를 사용하면 acquire 메서드가 그 동안 다른 스레드에 의해 변수가 업데이트된 경우 acquire 메서드가 원자적 작업을 재시도하여
   acquire 작업이 정확하고 안전하게 실행되도록 할 수 있다.

loop는 많은 동시 프로그래밍 시나리오에서 공통적인 패턴이며 경쟁 조건 및 기타 동기화 문제를 방지하는 데 도움이 된다.
그렇지만 loop를 사용하는 방식은 성능 측면에서 trade-offs가 있다.

crossbeam은 lock-free algorithms를 통한 상호 배제, wait-free를 통한 acquire operation을 구현할 수 있다.
이는 Atomic operation 및 memory ordering guarantees를 사용하여 달성된다.

Crossbeam의 또 다른 이점은 다양한 type의 동시 프리미티브에 대한 통합 API를 제공하여 필요에 따라 서로 다른 프리미티브 간에 쉽게 전환할 수 있다는 것이다.
이는 복잡한 동시 데이터 구조 또는 애플리케이션을 구축할 때 특히 유용할 수 있다.
기술적 이점 외에도 Crossbeam은 적극적인 개발 및 커뮤니티 지원으로 인해 Rust의 동시성 환경에서 중요한 부분이 되었다.
프로덕션 시스템에서 널리 사용되며 많은 Rust 개발자가 동시 프로그래밍을 위한 기본 라이브러리(external crate 이지만 tokio와 같이 많은 user들의 기본적인 option)로 채택했다.

전반적으로 Crossbeam 라이브러리는 동시 프로그래밍을 위한 안전하고 효율적이며 사용하기 쉬운 프리미티브 세트를 제공하기 때문에
Rust의 동시성 환경에 중요한 추가 기능이며 강력하고 확장 가능한 동시 시스템을 구축하기 위한 좋은 선택이다.

## 5. Crossbeam channels
### Explanation of Crossbeam channels
crossbeam channel은 스레드가 동시 프로그램에서 서로 통신할 수 있도록 하는 데이터 전송 메커니즘 type이다.
Rust의 표준 mpsc channel에 대한 대안으로 Crossbeam crate에서 제공한다.
crossbeam channel은 효율적이고 안전하도록 설계되어 atomic operation만 사용하여 스레드 간에 lock 및 wait가 없는 동기화를 제공한다.
또한 유연성이 있어 사용자가 특정 요구 사항에 맞게 다양한 메시지 전달 전략과 buffer types 중에서 선택할 수 있다.

Crossbeam 채널의 주요 기능 및 장점은 다음과 같다.

- Multiple message passing strategies: 사용자는 buffered 또는 unbuffered channel뿐만 아니라 blocking 또는 non-blocking semantics를 사용하여
  메시지를 보내고 받을 수 있다.
- Customizable buffer types: 사용자는 blocking 및 제한되지 않은 queue, lock-free ring buffer를 포함하여
  특정 워크로드에 최적화하기 위해 다양한 버퍼 types 중에서 선택할 수 있다.
- Lock-free and wait-free: Crossbeam channel은 atomic operation을 사용하여 lock 또는 spin-lock 없이 스레드 간에 효율적이고 안전한 동기화를 제공한다.
- Familiar API: Crossbeam channel API는 Rust의 std mpsc channel과 유사하므로 Rust 개발자가 쉽게 채택하고 사용할 수 있다.
- Flexibility: Crossbeam channel은 Rust의 std 라이브러리 및 external crate뿐만 아니라 다른 Crossbeam primitives와도 잘 작동하도록 설계되었다.

전반적으로 Crossbeam 채널은 동시 Rust 프로그램을 구축하기 위한 강력하고 유연한 도구이다.
스레드가 서로 통신할 수 있는 안전하고 효율적인 방법을 제공하는 동시에 다양한 사용 사례에 적합한 메시지 전달 및 버퍼 types에 대한 다양한 옵션을 제공한다.

### The internal implementation of the Crossbeam channel
Crossbeam channel의 내부 구현은 매우 복잡하지만 가장 중요한 부분에 대한 간략한 개요는 다음과 같다.  
아래는 Crossbeam channel의 구현 중 일부이다.
```rust
/// A slot in a block.
struct Slot<T> {
    /// The message.
    msg: UnsafeCell<MaybeUninit<T>>,

    /// The state of the slot.
    state: AtomicUsize,
}

pub fn bounded<T>(cap: usize) -> (Sender<T>, Receiver<T>) {
    if cap == 0 {
        let (s, r) = counter::new(flavors::zero::Channel::new());
        let s = Sender {
            flavor: SenderFlavor::Zero(s),
        };
        let r = Receiver {
            flavor: ReceiverFlavor::Zero(r),
        };
        (s, r)
    } else {
        let (s, r) = counter::new(flavors::array::Channel::with_capacity(cap));
        let s = Sender {
            flavor: SenderFlavor::Array(s),
        };
        let r = Receiver {
            flavor: ReceiverFlavor::Array(r),
        };
        (s, r)
    }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let (s, r) = counter::new(flavors::list::Channel::new());
    let s = Sender {
        flavor: SenderFlavor::List(s),
    };
    let r = Receiver {
        flavor: ReceiverFlavor::List(r),
    };
    (s, r)
}

/// Reference counter internals.
struct Counter<C> {
    /// The number of senders associated with the channel.
    senders: AtomicUsize,

    /// The number of receivers associated with the channel.
    receivers: AtomicUsize,

    /// Set to `true` if the last sender or the last receiver reference deallocates the channel.
    destroy: AtomicBool,

    /// The internal channel.
    chan: C,
}

/// Wraps a channel into the reference counter.
pub(crate) fn new<C>(chan: C) -> (Sender<C>, Receiver<C>) {
    let counter = Box::into_raw(Box::new(Counter {
        senders: AtomicUsize::new(1),
        receivers: AtomicUsize::new(1),
        destroy: AtomicBool::new(false),
        chan,
    }));
    let s = Sender { counter };
    let r = Receiver { counter };
    (s, r)
}

pub(crate) struct Sender<C> {
    counter: *mut Counter<C>,
}

/// The sending side.
impl<C> Sender<C> {
    /// Returns the internal `Counter`.
    fn counter(&self) -> &Counter<C> {
        unsafe { &*self.counter }
    }

    /// Acquires another sender reference.
    pub(crate) fn acquire(&self) -> Sender<C> {
        let count = self.counter().senders.fetch_add(1, Ordering::Relaxed);

        // Cloning senders and calling `mem::forget` on the clones could potentially overflow the
        // counter. It's very difficult to recover sensibly from such degenerate scenarios so we
        // just abort when the count becomes very large.
        if count > isize::MAX as usize {
            process::abort();
        }

        Sender {
            counter: self.counter,
        }
    }

    /// Releases the sender reference.
    ///
    /// Function `disconnect` will be called if this is the last sender reference.
    pub(crate) unsafe fn release<F: FnOnce(&C) -> bool>(&self, disconnect: F) {
        if self.counter().senders.fetch_sub(1, Ordering::AcqRel) == 1 {
            disconnect(&self.counter().chan);

            if self.counter().destroy.swap(true, Ordering::AcqRel) {
                drop(Box::from_raw(self.counter));
            }
        }
    }
}

/// The receiving side.
pub(crate) struct Receiver<C> {
    counter: *mut Counter<C>,
}

impl<C> Receiver<C> {
    /// Returns the internal `Counter`.
    fn counter(&self) -> &Counter<C> {
        unsafe { &*self.counter }
    }

    /// Acquires another receiver reference.
    pub(crate) fn acquire(&self) -> Receiver<C> {
        let count = self.counter().receivers.fetch_add(1, Ordering::Relaxed);

        // Cloning receivers and calling `mem::forget` on the clones could potentially overflow the
        // counter. It's very difficult to recover sensibly from such degenerate scenarios so we
        // just abort when the count becomes very large.
        if count > isize::MAX as usize {
            process::abort();
        }

        Receiver {
            counter: self.counter,
        }
    }

    /// Releases the receiver reference.
    ///
    /// Function `disconnect` will be called if this is the last receiver reference.
    pub(crate) unsafe fn release<F: FnOnce(&C) -> bool>(&self, disconnect: F) {
        if self.counter().receivers.fetch_sub(1, Ordering::AcqRel) == 1 {
            disconnect(&self.counter().chan);

            if self.counter().destroy.swap(true, Ordering::AcqRel) {
                drop(Box::from_raw(self.counter));
            }
        }
    }
}

pub(crate) struct Channel<T> {
    /// The head of the channel.
    head: CachePadded<Position<T>>,

    /// The tail of the channel.
    tail: CachePadded<Position<T>>,

    /// Receivers waiting while the channel is empty and not disconnected.
    receivers: SyncWaker,

    /// Indicates that dropping a `Channel<T>` may drop messages of type `T`.
    _marker: PhantomData<T>,
}

impl<T> Channel<T> {
    /// Attempts to reserve a slot for sending a message.
    fn start_send(&self, token: &mut Token) -> bool {
        let backoff = Backoff::new();
        let mut tail = self.tail.index.load(Ordering::Acquire);
        let mut block = self.tail.block.load(Ordering::Acquire);
        let mut next_block = None;

        loop {
            // Check if the channel is disconnected.
            if tail & MARK_BIT != 0 {
                token.list.block = ptr::null();
                return true;
            }

            // Calculate the offset of the index into the block.
            let offset = (tail >> SHIFT) % LAP;

            // If we reached the end of the block, wait until the next one is installed.
            if offset == BLOCK_CAP {
                backoff.snooze();
                tail = self.tail.index.load(Ordering::Acquire);
                block = self.tail.block.load(Ordering::Acquire);
                continue;
            }

            // If we're going to have to install the next block, allocate it in advance in order to
            // make the wait for other threads as short as possible.
            if offset + 1 == BLOCK_CAP && next_block.is_none() {
                next_block = Some(Box::new(Block::<T>::new()));
            }

            // If this is the first message to be sent into the channel, we need to allocate the
            // first block and install it.
            if block.is_null() {
                let new = Box::into_raw(Box::new(Block::<T>::new()));

                if self
                    .tail
                    .block
                    .compare_exchange(block, new, Ordering::Release, Ordering::Relaxed)
                    .is_ok()
                {
                    self.head.block.store(new, Ordering::Release);
                    block = new;
                } else {
                    next_block = unsafe { Some(Box::from_raw(new)) };
                    tail = self.tail.index.load(Ordering::Acquire);
                    block = self.tail.block.load(Ordering::Acquire);
                    continue;
                }
            }

            let new_tail = tail + (1 << SHIFT);

            // Try advancing the tail forward.
            match self.tail.index.compare_exchange_weak(
                tail,
                new_tail,
                Ordering::SeqCst,
                Ordering::Acquire,
            ) {
                Ok(_) => unsafe {
                    // If we've reached the end of the block, install the next one.
                    if offset + 1 == BLOCK_CAP {
                        let next_block = Box::into_raw(next_block.unwrap());
                        self.tail.block.store(next_block, Ordering::Release);
                        self.tail.index.fetch_add(1 << SHIFT, Ordering::Release);
                        (*block).next.store(next_block, Ordering::Release);
                    }

                    token.list.block = block as *const u8;
                    token.list.offset = offset;
                    return true;
                },
                Err(t) => {
                    tail = t;
                    block = self.tail.block.load(Ordering::Acquire);
                    backoff.spin();
                }
            }
        }
    }

    /// Writes a message into the channel.
    pub(crate) unsafe fn write(&self, token: &mut Token, msg: T) -> Result<(), T> {
        // If there is no slot, the channel is disconnected.
        if token.list.block.is_null() {
            return Err(msg);
        }

        // Write the message into the slot.
        let block = token.list.block.cast::<Block<T>>();
        let offset = token.list.offset;
        let slot = (*block).slots.get_unchecked(offset);
        slot.msg.get().write(MaybeUninit::new(msg));
        slot.state.fetch_or(WRITE, Ordering::Release);

        // Wake a sleeping receiver.
        self.receivers.notify();
        Ok(())
    }

    /// Sends a message into the channel.
    pub(crate) fn send(
        &self,
        msg: T,
        _deadline: Option<Instant>,
    ) -> Result<(), SendTimeoutError<T>> {
        let token = &mut Token::default();
        assert!(self.start_send(token));
        unsafe {
            self.write(token, msg)
                .map_err(SendTimeoutError::Disconnected)
        }
    }

    /// Attempts to reserve a slot for receiving a message.
    fn start_recv(&self, token: &mut Token) -> bool {
        let backoff = Backoff::new();
        let mut head = self.head.index.load(Ordering::Acquire);
        let mut block = self.head.block.load(Ordering::Acquire);

        loop {
            // Calculate the offset of the index into the block.
            let offset = (head >> SHIFT) % LAP;

            // If we reached the end of the block, wait until the next one is installed.
            if offset == BLOCK_CAP {
                backoff.snooze();
                head = self.head.index.load(Ordering::Acquire);
                block = self.head.block.load(Ordering::Acquire);
                continue;
            }

            let mut new_head = head + (1 << SHIFT);

            if new_head & MARK_BIT == 0 {
                atomic::fence(Ordering::SeqCst);
                let tail = self.tail.index.load(Ordering::Relaxed);

                // If the tail equals the head, that means the channel is empty.
                if head >> SHIFT == tail >> SHIFT {
                    // If the channel is disconnected...
                    if tail & MARK_BIT != 0 {
                        // ...then receive an error.
                        token.list.block = ptr::null();
                        return true;
                    } else {
                        // Otherwise, the receive operation is not ready.
                        return false;
                    }
                }

                // If head and tail are not in the same block, set `MARK_BIT` in head.
                if (head >> SHIFT) / LAP != (tail >> SHIFT) / LAP {
                    new_head |= MARK_BIT;
                }
            }

            // The block can be null here only if the first message is being sent into the channel.
            // In that case, just wait until it gets initialized.
            if block.is_null() {
                backoff.snooze();
                head = self.head.index.load(Ordering::Acquire);
                block = self.head.block.load(Ordering::Acquire);
                continue;
            }

            // Try moving the head index forward.
            match self.head.index.compare_exchange_weak(
                head,
                new_head,
                Ordering::SeqCst,
                Ordering::Acquire,
            ) {
                Ok(_) => unsafe {
                    // If we've reached the end of the block, move to the next one.
                    if offset + 1 == BLOCK_CAP {
                        let next = (*block).wait_next();
                        let mut next_index = (new_head & !MARK_BIT).wrapping_add(1 << SHIFT);
                        if !(*next).next.load(Ordering::Relaxed).is_null() {
                            next_index |= MARK_BIT;
                        }

                        self.head.block.store(next, Ordering::Release);
                        self.head.index.store(next_index, Ordering::Release);
                    }

                    token.list.block = block as *const u8;
                    token.list.offset = offset;
                    return true;
                },
                Err(h) => {
                    head = h;
                    block = self.head.block.load(Ordering::Acquire);
                    backoff.spin();
                }
            }
        }
    }

    /// Receives a message from the channel.
    pub(crate) fn recv(&self, deadline: Option<Instant>) -> Result<T, RecvTimeoutError> {
        let token = &mut Token::default();
        loop {
            // Try receiving a message several times.
            let backoff = Backoff::new();
            loop {
                if self.start_recv(token) {
                    unsafe {
                        return self.read(token).map_err(|_| RecvTimeoutError::Disconnected);
                    }
                }

                if backoff.is_completed() {
                    break;
                } else {
                    backoff.snooze();
                }
            }

            if let Some(d) = deadline {
                if Instant::now() >= d {
                    return Err(RecvTimeoutError::Timeout);
                }
            }

            // Prepare for blocking until a sender wakes us up.
            Context::with(|cx| {
                let oper = Operation::hook(token);
                self.receivers.register(oper, cx);

                // Has the channel become ready just now?
                if !self.is_empty() || self.is_disconnected() {
                    let _ = cx.try_select(Selected::Aborted);
                }

                // Block the current thread.
                let sel = cx.wait_until(deadline);

                match sel {
                    Selected::Waiting => unreachable!(),
                    Selected::Aborted | Selected::Disconnected => {
                        self.receivers.unregister(oper).unwrap();
                        // If the channel was disconnected, we still have to check for remaining
                        // messages.
                    }
                    Selected::Operation(_) => {}
                }
            });
        }
    }
}
```

1. 채널(bounded or unbounded)은 Sender와 Receiver로 구성되며 각각 별도의 lock-free queue을 사용하여 구현된다.
   여기서 queue는 Slot 또는 Buffer로 구현된다. 채널 버퍼의 각 queue는 lock-free queue 데이터 구조의 노드에 해당한다. 
   queue의 노드는 채널을 통해 송수신되는 데이터에 대한 정보와 queue의 현재 상태(예: empty or full 여부)를 유지한다.
   이 데이터 구조를 사용하면 여러 스레드가 lock이나 기타 동기화 프리미티브 없이 동시에 데이터를 보내고 받을 수 있으므로
   동시성이 높은 애플리케이션에서 더 나은 성능과 확장성을 얻을 수 있다.
2. Sender가 메시지를 보낼 때 메시지를 자신의 대기열로 푸시한다.
   즉, send() 메소드는 start_send() 메소드를 호출하여 채널에 메시지를 보낼 수 있는지 먼저 확인한 다음
   write() 메소드를 사용하여 메시지를 채널 버퍼(slot)에 쓴다. 메시지가 성공적으로 작성되면 send() 메서드는 Ok(())를 반환하고,
   그렇지 않으면 전송에 실패한 메시지와 함께 SendTimeoutError를 반환한다. 이것은 작업이 스레드로부터 안전한지 확인하기 위해 원자적으로 수행된다.
3. receiver가 recv() 메서드를 통해서 메시지를 받으면 먼저 자신의 queue를 확인한다. 메시지가 있으면 가장 오래된 메시지를 팝하여 반환한다.  
   - crossbeam_deque의 경우, 메시지가 없으면 sender의 queue에서 메시지를 훔치려고 시도한다. 성공적으로 메시지를 도용하면 해당 메시지를 반환한다.
   그렇지 않은 경우 메시지 전송을 기다린다.
4. Sender와 Receiver는 또한 공유 대기열(채널 버퍼 또는 슬롯)을 사용하여 채널 상태에 대한 정보를 교환해야 할 때 서로 통신한다.


### Comparison with Rust's mpsc channels
Crossbeam 채널과 Rust의 mpsc(다중 생산자, 단일 소비자) 채널은 모두 Rust의 스레드 간에 데이터를 전송하는 메커니즘이다.
그러나 그들 사이에는 몇 가지 주요 차이점이 있다.

1. Crossbeam 채널이 Rust의 mpsc 채널보다 더 효율적이고 유연하게 설계되었다.  
Crossbeam channel은 lock-free 알고리즘을 사용하여 구현된다. 즉, 높은 경합 시나리오에서도 높은 처리량과 낮은 대기 시간을 달성할 수 있다.
대조적으로 Rust의 mpsc channel은 특정 시나리오에서 성능 병목 현상을 일으킬 수 있는 전통적인 lock 메커니즘을 사용하여 구현된다.

2. Crossbeam channel은 data types 및 buffer size 측면에서 더 많은 유연성을 제공한다.  
예를 들어 Crossbeam channel은 arbitrary data type을 전송할 수 있는 반면 Rust의 mpsc 채널은 Send trait을 구현하는 데이터 전송으로 제한된다.
또한 Crossbeam channel은 다양한 사용 사례에 맞게 최적화하기 위해 다양한 buffer size로 구성할 수 있지만 Rust의 mpsc 채널은 fixed buffer size를 갖는다.

사용 사례와 관련하여 고성능 및 유연성이 중요한 시나리오에서는 Crossbeam channel이 선호될 수 있다.
예를 들어 고성능 네트워킹 응용 프로그램이나 데이터를 스레드 간에 빠르게 전송해야 하는 병렬 처리 시나리오에서 사용할 수 있다.

그러나 simplicity와 사용 용이성이 raw performance보다 더 중요한 시나리오에서는 Rust의 mpsc channel이 선호될 수 있다.
예를 들어 간단한 명령줄 응용 프로그램이나 성능이 중요한 요소가 아닌 다른 시나리오에서 사용할 수 있다.

Crossbeam 채널과 Rust의 mpsc 채널의 성능을 비교하기 위해 [benchmark](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel/benchmarks) 를 사용할 수 있다.
벤치마크는 Crossbeam 채널이 특정 시나리오에서 Rust의 mpsc 채널보다 더 높은 처리량과 더 낮은 대기 시간을 달성할 수 있음을 보여주었다.
그러나 각 채널 유형의 정확한 성능 특성은 스레드 수, 데이터 크기 및 버퍼 크기와 같은 요소에 따라 달라진다.
따라서 어떤 것이 더 적절한지 결정하기 위해 개발 중인 응용 프로그램의 특정 컨텍스트에서 두 채널 유형을 모두 벤치마킹하는 것이 중요하다.

### Examples of using Crossbeam channels
다음은 Crossbeam channel의 여러가지 사용 예이다.

- 한 스레드에서 다른 스레드로 여러 메시지 보내기
```rust
use crossbeam_channel::{unbounded, Sender};

fn main() {
    let (s, r) = unbounded();
    for i in 0..10 {
        let message = format!("Message {}", i);
        s.send(message).unwrap();
    }
    for _ in 0..10 {
        let received = r.recv().unwrap();
        println!("{}", received);
    }
}
```

- try_recv() 메서드를 사용해 non-blocking으로 메시지를 수신하는 방법
```rust
use crossbeam_channel::{unbounded, Sender, TryRecvError};

fn main() {
    let (s, r) = unbounded();
    s.send("Hello, world!".to_owned()).unwrap();
    match r.try_recv() {
        Ok(received) => println!("{}", received),
        Err(TryRecvError::Empty) => println!("No message available"),
        Err(TryRecvError::Disconnected) => println!("Channel closed"),
    }
}
```

- `crossbeam_deque`를 사용하여 work stealing algorithm을 구현
  이 예에서는 n개의 worker 스레드와 실행해야 하는 Task가 포함된 Task queue가 있다.
  각 worker 스레드에는 자체 deque가 있으며 처음에는 실행할 task를 찾는다.
  worker의 deque가 비어 있으면 다른 worker의 deque에서 task를 훔치려고 시도한다.
  worker 스레드는 task queue가 비워질 때까지 task를 계속 실행한다.
```rust
use crossbeam_deque::{Worker, Steal};
use std::thread;

fn main() {
    // Create a worker for each available CPU core
    let num_workers = num_cpus::get();
    let workers: Vec<_> = (0..num_workers).map(|_| Worker::new_fifo()).collect();

    // Spawn worker threads
    let handles: Vec<_> = workers
        .iter()
        .map(|worker| {
            let worker_clone = worker.clone();
            thread::spawn(move || worker_loop(worker_clone))
        })
        .collect();

    // Fill the task queue with tasks to be executed
    let tasks = vec![1, 2, 3, 4, 5];
    let task_queue = &mut Vec::new();
    for task in tasks {
        task_queue.push(task);
    }

    // Wait for workers to finish executing tasks
    for handle in handles {
        handle.join().unwrap();
    }
}

fn worker_loop(worker: Worker<i32>) {
    loop {
        // Attempt to pop a task from the worker's own deque
        match worker.pop() {
            Steal::Success(task) => {
                println!("Worker {} executing task {}", worker.index(), task);
                // Execute the task
            }
            Steal::Empty => {
                // Attempt to steal a task from other workers' deques
                for i in 0..num_cpus::get() {
                    if i == worker.index() {
                        continue;
                    }
                    let other_worker = &workers[i];
                    match other_worker.steal() {
                        Steal::Success(task) => {
                            println!(
                                "Worker {} stole task {} from worker {}",
                                worker.index(),
                                task,
                                i
                            );
                            // Execute the stolen task
                            break;
                        }
                        Steal::Empty => {}
                    }
                }
                // If no tasks were found, exit the worker loop
                if task_queue.is_empty() {
                    break;
                }
            }
            Steal::Retry => {}
        }
    }
}
```
이 예에서 'crossbeam_deque'는 worker deques 및 work stealing 기능을 구현하는 데 사용된다.
각 worker는 할 task를 찾을 때 task를 pop하는 자체 deque를 가지고 있다.
worker의 deque가 비어 있으면 task를 찾거나 task queue가 비워질 때까지 다른 worker의 deque에서 task를 훔치려고 시도한다.
task가 없고 task queue가 비어 있으면 worker loop가 종료된다.

## 6. Crossbeam atomic types
Crossbeam atomic types들은 Rust의 atomic types와 유사하지만 atomic instructions를 사용할 수 없거나 효율적이지 않은 경우 global lock을 사용한다.
이 섹션에서는 그것들이 어떻게 작동하는지, Rust의 atomic types와 비교하여 차이와 장단점을 알아보고 사용법을 살펴보자.

### Explanation of Crossbeam atomic types
Crossbeam의 atomic types는 mutable memory location에서 스레드로부터 안전하고 lock-free operation을 제공하는 set of atomic primitives이다.
Rust의 atomic types와 유사하게 설계되었지만 atomic instructions를 사용할 수 없거나 효율적이지 않을 때 global lock을 사용하는 flexibility가 추가되었다.
Crossbeam의 AtomicCell 객체를 만들어서 사용할때, 이것은 기본적으로 Atomic 연산으로 lock-free, wait-free 메카니즘으로 작동하지만,
is_lock_free 메서드가 false로 나온다면(atomic operation을 사용할 수 없거나, 주어진 type에 대해 효율적이지 않다),
그 상황에서는 fetch_add등의 메서드를 통해 그 내부의 구현으로 전역 lock을 사용한다.
여기의 fetch_add 등의 메서드 내부의 global lock은 `SeqLock`의 일부 구현이다. 유저가 직접 lock을 명시할 필요는 없다.

Crossbeam의 atomic library에서 중요한 module 중 하나는 Rust의 Cell type의 thread safe 버전을 제공하는 `AtomicCell` module이다.
Cell과 마찬가지로 AtomicCell은 여러 스레드에서 동시에 액세스하고 수정할 수 있는 변경 가능한 메모리 위치를 제공한다. 게다가 std 라이브러리의
Atomic types들과 마찬가지로 `load` 메서드는 `Acquire` Ordering, `store` 메서드는 `Release` Ordering 순서를 사용한다. 
그러나 Cell과 달리 AtomicCell은 기본 하드웨어의 기능에 따라 스레드 안전을 보장하기 위해 atomic instructions 또는 global lock을 사용한다.
```rust
pub struct AtomicCell<T> {
    value: UnsafeCell<MaybeUninit<T>>,
}

pub union MaybeUninit<T> {
    uninit: (),
    value: ManuallyDrop<T>,
}

pub struct ManuallyDrop<T: ?Sized> {
    value: T,
}

impl<T> ManuallyDrop<T> {
    pub const fn new(value: T) -> ManuallyDrop<T> {
        ManuallyDrop { value }
    }

    pub const fn into_inner(slot: ManuallyDrop<T>) -> T {
        slot.value
    }

    pub unsafe fn take(slot: &mut ManuallyDrop<T>) -> T {
        // SAFETY: we are reading from a reference, which is guaranteed
        // to be valid for reads.
        unsafe { ptr::read(&slot.value) }
    }

    pub unsafe fn drop(slot: &mut ManuallyDrop<T>) {
        unsafe { ptr::drop_in_place(&mut slot.value) }
    }
}

impl<T> MaybeUninit<T> {
    pub const fn new(val: T) -> MaybeUninit<T> {
        MaybeUninit { value: ManuallyDrop::new(val) }
    }

    pub const fn uninit() -> MaybeUninit<T> {
        MaybeUninit { uninit: () }
    }

    pub const fn uninit_array<const N: usize>() -> [Self; N] {
        // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
        unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
    }

    pub const fn zeroed() -> MaybeUninit<T> {
        let mut u = MaybeUninit::<T>::uninit();
        // SAFETY: `u.as_mut_ptr()` points to allocated memory.
        unsafe {
            u.as_mut_ptr().write_bytes(0u8, 1);
        }
        u
    }

    pub const fn as_ptr(&self) -> *const T {
        // `MaybeUninit` and `ManuallyDrop` are both `repr(transparent)` so we can cast the pointer.
        self as *const _ as *const T
    }
}
```
AtomicCell은 Rust의 std라이브러리 내의 Atomic struct와 비슷하게 UnsafeCell로 값을 저장하지만,
UnsafeCell내부의 타입에 MaybeUninit이라는 struct로 한층 더 래핑하여 커스텀된 타입을 사용한다.
MaybeUninit은 효율성과 안전성을 위한 기능을 제공한다. 예를 들어 MaybeUninit 내부의 타입이 array라면 효율적인 transpose 메서드도 제공한다.  
MaybeUninit의 목적은 유저에게 노출되지 않고 대신 라이브러리의 구현에 의해 내부적으로 사용되는 uninitialized memory position의 생성을 허용하는 것이다.
이렇게 하면 라이브러리에서 구현된 기능이 메모리 위치를 자동으로 초기화하는 컴파일러에 의존하지 않고 안전할 때 수동으로 메모리 위치를 초기화할 수 있다.

Rust는 `MaybeUninit`을 사용하여 유저에게 노출되기 전에 메모리 위치가 완전히 초기화되도록 할 수 있으므로 초기화되지 않은 메모리에서 읽거나 쓸 때
발생할 수 있는 정의되지 않은 동작을 피할 수 있다.

여기서 컴파일러가 자동으로 객체를 메모리에서 관리하지 못하게 하기 위해, MaybeUninit struct는 ManuallyDrop<T>를 값으로 갖는다.
이 struct는 zero-sized type으로 ManuallyDrop<T>는 T와 동일한 레이아웃을 갖도록 보장된다. 즉 T와 동일한 레이아웃 최적화가 적용된다.  
결과적으로 compiler가 메모리 위치의 내용에 대해 잘못된 가정을 하여 초기화하지 않도록 하여
초기화되지 않은 메모리로 작업하는 안전하고 효율적인 방법을 제공한다.

Crossbeam atomic library의 또 다른 중요한 module은 `Consume` memory ordering을 사용하여 primitive atomic type에서 읽을 수 있는 방법을 제공하는 `Consume` module이다.
이 순서는 Rust의 atomic type이 제공하는 `Acquire` order와 유사하지만 load 결과에 '의존하는' 작업의 순서만 보장한다.
즉 `Acquire`는 모든 후속 작업의 순서를 보장하는 반면 `Consume`은 로드된 값에 따라 달라지는 후속 작업의 순서만 보장하기 때문에 `Acquire` 순서가 `Consume` 순서보다 강력하다.
그렇지만 `Consume` Ordering은 memory fence instructions가 필요하지 않기 때문에 weak memory architectures에서 `Acquire` Ordering보다 빠를 수 있다.  
(`Consume` Ordering은 읽기 전에 발생하는 모든 메모리 읽기가 그보다 먼저 정렬되도록 보장하고, `Consume` 읽기가 후속 읽기 또는 쓰기로 재정렬될 수 없도록 한다.)
따라서 `Consume` Ordering은 서로 의존하는 여러 메모리 위치를 읽는 데 있어서는 `Acquire` Ordering과 같은 기능을 하면서, 더 빠르게 처리할 수 있는 유용한 순서가 된다.  
Consume module은 안전하고 일관된 방식으로 여러 AtomicCell에서 값을 읽는 데 사용되는 `ConsumeGuard`라는 type을 제공한다.
이는 AtomicCell의 슬라이스에서 consume 메서드를 호출하여 얻으며 모든 읽기가 consume 순서로 발생하도록 한다.  
다음은 Consume 모듈의 사용 예이다.
```rust
use crossbeam::atomic::AtomicCell;
use crossbeam::atomic::Consume;

let a = AtomicCell::new(1);
let b = AtomicCell::new(2);
let c = AtomicCell::new(3);

let values = Consume::consume(&[&a, &b, &c]);

let value_a = values[0].get();
let value_b = values[1].get();
let value_c = values[2].get();

println!("a = {}, b = {}, c = {}", value_a, value_b, value_c);
```
이 예제에서는 세 개의 AtomicCell을 만들고 Consume 모듈을 사용하여 해당 값을 읽는다.
`consume` 메서드는 읽기가 `Consume` order로 정렬되도록 하는 `ConsumeGuard`를 반환한다.
그런 다음 각 ConsumeGuard에서 get 메서드를 사용하여 AtomicCell의 값에 액세스한다.

전반적으로 Crossbeam의 atomic types는 Rust의 atomic types와 유사하지만 atomic instructions를 사용할 수 없거나 효율적이지 않을 때
global lock을 사용하는 유연성이 추가되었다. 이를 통해 atomic instructions를 지원하지 않는 하드웨어에서도 변경 가능한 메모리 위치에서
고성능 스레드 안전 작업을 제공할 수 있다. 그러나 global lock을 사용하는 것이 atomic instructions를 사용하는 것보다 느릴 수 있으므로
사용 사례에 적합한 atomic types를 선택하는 것이 중요하다.

### Comparison with Rust's Atomic types
Rust의 표준 라이브러리는 변경 가능한 메모리 위치에 대한 안전하고 동시적인 액세스를 허용하는 여러 atomic types들을 제공한다.
Crossbeam의 atomic types는 Rust의 atomic types들과 유사하도록 설계되었지만 몇 가지 중요한 차이점이 있다.

Rust의 Atomic types들에는  AtomicBool, AtomicIsize, AtomicUsize, AtomicPtr 등이 있으며,
Atomic load, store, exchange, CAS, fetch-and-add등의 기능이 있다.

아래는 주요 차이점이다.
- Crossbeam AtomicCell은 atomic instructions를 사용할 수 없거나 효율적이지 않을 때 global lock을 사용하는 반면,
  Rust의 atomic types들은 항상 하드웨어 기반 atomic instructions를 사용한다.
- Crossbeam AtomicCell은 경우에 따라 더 나은 성능을 제공하기 위해 시퀀스 잠금을 사용하는 기능(SeqLock)과 같은 추가 기능을 제공한다.

다음은 Crossbeam AtomicCell의 사용이 더 나은 선택일 수 있는 사례이다
- 하드웨어 기반 atomic instructions를 사용할 수 없거나 효율적이지 않은 경우.
- SeqLock을 사용하면 하드웨어 기반 atomic instructions보다 더 나은 성능을 제공할 수도 있다.
- Crossbeam AtomicCell은 하드웨어 기반 atomic instructions가 효율적이지 않은 특정한 경우에 더 많은 유연성과 더 나은 성능을 제공한다.
  그러나 Crossbeam AtomicCell을 사용하면 global lock 사용으로 인해 오버헤드가 추가될 수 있으며 Rust의 atomic types만큼 광범위하게 테스트되거나 신뢰성을 보장하지는 않을 수 있다.

일반적으로 애플리케이션이 Rust의 atomic types들을 사용할 수 있다면 그것을 사용하는 것이 권장되는 구현이다.
그러나 특정한 경우에 더 많은 유연성이나 더 나은 성능이 필요한 경우 Crossbeam AtomicCell이 좋은 선택일 수 있다.
### Examples of using Crossbeam atomic types
다음은 Rust 프로그램에서 Crossbeam atomic types들을 사용하는 방법에 대한 몇 가지 예이다.

- 예제 1: 스레드 간에 변경 가능한 값을 공유.
```rust
use crossbeam::atomic::AtomicCell;

let cell = AtomicCell::new(42);

// Spawn a thread to increment the value.
crossbeam::scope(|s| {
    s.spawn(|_| {
        let val = cell.fetch_add(1);
        println!("incremented value: {}", val + 1);
    });
}).unwrap();

// Wait for the thread to finish and print the final value.
let final_val = cell.into_inner();
println!("final value: {}", final_val);
```
AtomicCell은 여러 스레드에서 변경 가능한 값에 액세스할 수 있는 스레드 안전하고 lock-free 방법을 제공한다.
이 예제에서는 초기 값이 42인 AtomicCell을 생성하고 새 스레드를 생성하여 값을 증가시키고 증가된 값을 print한다.
스레드가 끝나면 AtomicCell의 최종 값을 print한다.

- 예제 2: global lock과 함께 AtomicCell을 사용하여 Queue 구현
queue에는 push 및 pop 작업이 모두 필요하므로 atomic instructions만 사용하는 것으로는 충분하지 않다.
대신 queue 수정 중에 상호 배제를 보장하기 위해 global lock을 사용한다.
```rust
use crossbeam::atomic::{AtomicCell, AtomicConsume};
use std::sync::{Arc, atomic::Ordering};

struct Queue<T> {
    head: AtomicCell<*mut Node<T>>,
    tail: AtomicCell<*mut Node<T>>,
}

struct Node<T> {
    value: T,
    next: AtomicCell<*mut Node<T>>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        let sentinel = Arc::new(Node {
            value: Default::default(),
            next: AtomicCell::new(std::ptr::null_mut()),
        });

        Queue {
            head: AtomicCell::new(Arc::into_raw(sentinel.clone())),
            tail: AtomicCell::new(Arc::into_raw(sentinel)),
        }
    }

    fn push(&self, value: T) {
        let mut sentinel = self.tail.load();
        let new_node = Arc::new(Node {
            value,
            next: AtomicCell::new(std::ptr::null_mut()),
        });
        loop {
            let next = unsafe { &mut *sentinel }.next.load();
            if next.is_null() {
                if unsafe { &mut *sentinel }.next.compare_and_swap(
                    std::ptr::null_mut(),
                    Arc::into_raw(new_node.clone()) as *mut _,
                    Ordering::Release,
                ).is_ok() {
                    self.tail.store(Arc::into_raw(new_node) as *mut _, Ordering::Release);
                    break;
                }
            } else {
                self.tail.store(next, Ordering::Release);
                sentinel = next;
            }
        }
    }

    fn pop(&self) -> Option<T> {
        let mut sentinel = self.head.load(AtomicConsume);
        loop {
            let next = unsafe { &mut *sentinel }.next.load();
            if next.is_null() {
                return None;
            }
            let value = unsafe { Arc::from_raw(next) }.value;
            if self.head.compare_and_swap(
                sentinel,
                next,
                Ordering::Release,
            ).is_ok() {
                return Some(value);
            }
            sentinel = self.head.load(AtomicConsume);
        }
    }
}
```
push 메소드에서 먼저 queue의 현재 tail을 load한다.
그런 다음 제공된 값으로 새 노드를 만들고 현재 tail 노드의 next 필드를 새 노드로 설정하여 queue에 추가하려고 시도한다.
이 작업이 성공하면 tail pointer가 새 노드를 가리키도록 업데이트한다.
compare_and_swap 작업이 실패하면 loop를 돌고 다시 시도한다.

pop 메소드에서 먼저 queue의 현재 head를 load한다.
그런 다음 head pointer를 현재 head 노드의 'next' 필드로 설정하여 queue의 다음 노드로 이동하려고 시도한다.
이 작업이 성공하면 제거된 노드의 값을 반환한다.
compare_and_swap 작업이 실패하면 loop를 돌고 Sentinel 포인터를 현재 헤드 포인터로 업데이트하고 다시 시도한다.

pop 메서드 후에 이전 head 노드가 대기열의 마지막 노드인지 확인한다.
그렇다면 queue가 이제 비어 있음을 알고 head 및 tail pointer를 모두 null로 업데이트해야 한다.
이전 head 노드가 queue의 마지막 노드가 아니면 단순히 제거된 노드의 값을 반환한다.

pop 메서드에서 제거된 노드의 값을 읽을 때 `AtomicConsume` Ordering을 사용한다는 점을 주목해야 한다.
이렇게 하면 노드가 제거되기 전에 발생한 모든 메모리 작업이 Queue에서 노드를 pop하는 스레드에 표시되도록 보장된다.
즉 해당 메모리 작업에 의존하는 모든 후속 작업이 올바르게 정렬되고 현재 로드 전에 발생하도록 재정렬되지 않는다.

load 메서드에서 `AtomicConsume` Ordering을 사용하면 현재 로드에서 반환된 값에 의존하는 모든 후속 로드가 현재 로드 이전에 발생하도록 재정렬되지 않는다.
이 순서는 서로 의존하는 일련의 로드가 있고 올바른 순서로 실행되도록 하려는 경우에 유용할 수 있다.
`AtomicConsume` Ordering은 `Acquire` Ordering보다 약한 순서이기 때문에 컴파일러 또는 프로세서에서 더 많은 재정렬을 허용할 수 있으므로
의도된 사용 사례에 대해 약한 순서가 충분한 경우에만 주의해서 사용해야 한다.

마지막으로 push 및 pop 메서드 모두에서 compare_and_swap 작업이 너무 많이 실패하면 global lock을 사용한다.
이는 하드웨어가 효율적인 atomic operations를 제공하지 않는 경우에도 계속 진행할 수 있도록 하는 fallback 메커니즘이다.

이러한 각 예제에서 Crossbeam AtomicCell을 사용하여 변경 가능한 메모리 위치에서 스레드로부터 안전하고 lock-free 작업을 구현하는 방법을 보여준다.

## 7. parking_lot
### Overview
parking_lot은 성능과 확장성에 중점을 둔 lock, condvar 및 barrier와 같은 다양한 동기 primitives들을 제공하는 external crate이다.
std::sync module에 대한 대체재로 설계되었지만 더 효율적이고 확장 가능한 구현들이 있다.
```rust
pub struct RwLock<R, T: ?Sized> {
    raw: R,
    data: UnsafeCell<T>,
}
```
위의 구현은 lock_api 모듈의 RwLock의 구현이다.
data는 역시 UnsafeCell<T>로 구현되어 있고, raw의 R은 RawRwLock이라는 struct이며 내부는 atomic 연산으로 동작하는 state로 구현되어 있다.
```rust
pub struct RawRwLock {
    state: AtomicUsize,
}
```
```rust
impl<R: RawRwLock, T: ?Sized> RwLock<R, T> {
    pub fn new(val: T) -> RwLock<R, T> {
        RwLock {
            data: UnsafeCell::new(val),
            raw: R::INIT,
        }
    }
    
    pub fn read(&self) -> RwLockReadGuard<'_, R, T> {
        self.raw.lock_shared();
        // SAFETY: The lock is held, as required.
        unsafe { self.read_guard() }
    }
    
    pub fn write(&self) -> RwLockWriteGuard<'_, R, T> {
        self.raw.lock_exclusive();
        // SAFETY: The lock is held, as required.
        unsafe { self.write_guard() }
    }
}

unsafe impl lock_api::RawRwLock for RawRwLock {
    const INIT: RawRwLock = RawRwLock {
        state: AtomicUsize::new(0),
    };
    fn lock_exclusive(&self) {
        if self
            .state
            .compare_exchange_weak(0, WRITER_BIT, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            let result = self.lock_exclusive_slow(None);
            debug_assert!(result);
        }
        self.deadlock_acquire();
    }
    unsafe fn unlock_exclusive(&self) {
        self.deadlock_release();
        if self
            .state
            .compare_exchange(WRITER_BIT, 0, Ordering::Release, Ordering::Relaxed)
            .is_ok()
        {
            return;
        }
        self.unlock_exclusive_slow(false);
    }
    fn lock_shared(&self) {
        if !self.try_lock_shared_fast(false) {
            let result = self.lock_shared_slow(false, None);
            debug_assert!(result);
        }
        self.deadlock_acquire();
    }
    unsafe fn unlock_shared(&self) {
        self.deadlock_release();
        let state = if have_elision() {
            self.state.elision_fetch_sub_release(ONE_READER)
        } else {
            self.state.fetch_sub(ONE_READER, Ordering::Release)
        };
        if state & (READERS_MASK | WRITER_PARKED_BIT) == (ONE_READER | WRITER_PARKED_BIT) {
            self.unlock_shared_slow();
        }
    }
    fn is_locked(&self) -> bool {
        let state = self.state.load(Ordering::Relaxed);
        state & (WRITER_BIT | READERS_MASK) != 0
    }

    fn is_locked_exclusive(&self) -> bool {
        let state = self.state.load(Ordering::Relaxed);
        state & (WRITER_BIT) != 0
    }
}
```
RwLock의 raw 필드는 AtomicUsize의 state로 구성되어 있기 때문에 upgrade or downgrade or unlock or lock 하는 방식은 매우 효율적으로 진행된다.
lock or unlock upgradable or downgradable 이 true or false라면(data의 load 또는 store가능 여부를 통해 알아냄),
AtomicUsize를 atomic 연산으로 fetch_add 또는 fetch_sub으로 bitwise 연산으로 매우 빠르게 상태를 변화시킨다.

std sync와 Parking_lot 알고리즘의 주요 차이점은 경합을 처리하는 방식에 있다. 즉 spin-lock의 차이에 있다.

std::sync에서 Mutex 또는 RwLock에 대한 lock을 획득하려는 스레드는 lock이 사용 가능해질 때까지 blocking되며 acquired되었을 때 계속 진행할 수 있다.
이로 인해 많은 스레드가 동일한 lock을 acquire하려고 할 때 모두 blocking되고 차례를 기다려야 하므로 많은 경합이 발생할 수 있다.

반면에 parking_lot은 보다 효율적인 spin-wait approach를 사용한다.
스레드가 lock을 획득하려고 시도하고 다른 스레드가 이미 보유하고 있음을 발견하면 lock이 사용 가능해질 때까지 짧은 시간 동안 loop에서 spin한다.
이렇게 하면 스레드가 lock을 기다리는 데 소요되는 시간이 줄어들고 경합이 높은 시나리오에서 성능이 향상될 수 있다.
```rust
fn lock_exclusive_slow(&self, timeout: Option<Instant>) -> bool {
        let try_lock = |state: &mut usize| {
            loop {
                if *state & (WRITER_BIT | UPGRADABLE_BIT) != 0 {
                    return false;
                }

                // Grab WRITER_BIT if it isn't set, even if there are parked threads.
                match self.state.compare_exchange_weak(
                    *state,
                    *state | WRITER_BIT,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => return true,
                    Err(x) => *state = x,
                }
            }
        };

        // Step 1: grab exclusive ownership of WRITER_BIT
        let timed_out = !self.lock_common(
            timeout,
            TOKEN_EXCLUSIVE,
            try_lock,
            WRITER_BIT | UPGRADABLE_BIT,
        );
        if timed_out {
            return false;
        }

        // Step 2: wait for all remaining readers to exit the lock.
        self.wait_for_readers(timeout, 0)
    }
```
lock_exclusive_slow 메서드 내부의 spin-wait 방식은 loop에서 `Instant`를 사용하여 시간 제한을 구현하는 데 사용된다.
loop는 성공하거나 제한 시간에 도달할 때까지 `WRITER_BIT`를 획득하려고 계속 시도한다.
이 spin-wait approach 방식은 스레드를 blocking하고 나중에 OS에서 스레드를 다시 예약하도록 허용하여 추가 오버헤드를 유발할 수 있는 것보다 더 효율적일 수 있다.
제한 시간이 있는 spin-wait loop를 사용하여 스레드는 context switching 오버헤드를 최소화하면서 lock이 사용 가능해질 때까지 능동적으로 대기할 수 있다.

즉, std::sync의 futex spin-lock은 loop에서 즉시 획득하지 못한다면, 바로 스레드를 blocking하고 context switching한다.
반면에 parking_lot의 spin-lock은 loop에서 즉시 획득하지 못하더라도, 특정 짧은 시간(시간 혹은 반복 횟수)동안 loop를 지속하여 continue하여
lock을 acquire할 수 있는지 반복 확인한다. 그래서 이 기간동안에 lock을 획득하지 못하더라도, 매 회전마다 확정적인 context switching을 하는
overhead보다는 오버헤드가 적을 가능성이 높기 때문에 일반적으로 성능이 더 좋게 나온다.
그러므로 lock의 acquire 및 release가 매우 빈번한 경우에 최적화되어 있으므로 일반적으로 이러한 동작을 나타내는 워크로드에 더 적합하다.
그러나 lock을 장기간 유지하고 acquire & release가 거의 없는 모델에서는 획득할 수 없는 lock을 위해 loop를 유지하는 오버헤드가 더 클 수 있다.
이렇듯 워크로드의 특정 사항에 따라 효율적인 모듈이 달라지기 때문에 다양한 lock 구현을 벤치마킹 하는 것이 좋다.

두 라이브러리의 또 다른 차이점은 Parking_lot이 Parking_lot::MutexGuard 및 Parking_lot::RwLockUpgradableReadGuard와 같이
표준 라이브러리에서 찾을 수 없는 일부 추가 동기 프리미티브를 제공한다는 것이다.
이러한 프리미티브를 사용하면 lock 프로세스를 보다 세밀하게 제어할 수 있으며 일부 특수 시나리오에서 유용할 수 있다.

### Advantages over std::sync
parking_lot은 std::sync module에 비해 몇 가지 이점을 제공한다.

- Better performance: blocking 대신 spin-waiting을 사용하면 스레드가 lock을 기다리는 시간을 줄이고 경합이 높은 시나리오에서 성능을 향상시킬 수 있다.
  또한 parking_lot crate는 성능과 확장성을 염두에 두고 설계되었으며 동기 primitives의 보다 효율적이고 확장 가능한 구현을 제공한다.
- Additional synchronization primitives: std::sync에서 제공하는 표준 동기 primitives 외에도 parking_lot은
  RwLockUpgradableReadGuard 및 MutexGuard와 같은 추가 프리미티브를 제공하여 lock 프로세스를 보다 세밀하게 제어할 수 있으며 일부 특수 시나리오에서 유용할 수 있다.
- Smaller memory footprint: parking_lot은 일부 동기 프리미티브에 더 작은 데이터 구조를 사용하므로 std::sync에 비해 더 작은 메모리 공간을 차지한다.
- More predictable behavior: parking_lot에서 spin-waiting을 사용하면 std::sync에서 blocking하는 것과 비교하여 더 예측 가능한 동작을 제공할 수 있다.
  경합이 높은 경우 blocking은 예측할 수 없고 잠재적으로 긴 대기 시간을 초래할 수 있는 반면 spin-waiting은 짧은 대기 시간으로 보다 결정적인 동작을 제공할 수 있다.
- Reduced context switching overhead: parking_lot의 spin-waiting approach 방식은 std::sync의 blocking에 비해 context switching 오버헤드를 줄일 수 있다.
  lock이 사용 가능해질 때까지 능동적으로 기다리면 스레드가 OS에 의해 blocking되고 스케줄이 변경되는 것을 방지할 수 있다. 이로 인해 추가 오버헤드가 발생할 수 있다.
- less contention: parking_lot의 프리미티브는 공유 리소스에 대한 경합을 줄이도록 설계되어 동시성이 높은 상황에서 확장성을 높인다.
- More features: parking_lot은 try_lock 메서드를 지원하는 Mutex 구현과 같이 표준 라이브러리에 없는 추가 기능을 제공한다.

전반적으로 parking_lot은 특히 높은 경합 시나리오에서 std::sync보다 성능이 뛰어나고 확장 가능한 대안이다.
또한 parking_lot에서 제공하는 추가 동기 프리미티브는 특정 시나리오에서 더 많은 제어와 유연성을 제공할 수 있다.

### Usage examples
다음은 parking_lot의 동기 primitives들 중 몇가지의 예이다.
- Mutex(Same usage as std::sync::Mutex)
```rust
use parking_lot::Mutex;

let mutex = Mutex::new(0);
let mut value = mutex.lock();
*value += 1;
```

- RwLock(Same usage as std::sync::RwLock)
```rust
use parking_lot::RwLock;

let rw_lock = RwLock::new(0);
let value = rw_lock.read();
assert_eq!(*value, 0);
let mut value = rw_lock.write();
*value += 1;
```

- Condvar(Same usage as std::sync::Condvar)
```rust
use parking_lot::{Mutex, Condvar};

let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = pair.clone();
thread::spawn(move || {
    let &(ref lock, ref cvar) = &*pair2;
    let mut started = lock.lock();
    *started = true;
    cvar.notify_one();
});

let &(ref lock, ref cvar) = &*pair;
let mut started = lock.lock();
while !*started {
    started = cvar.wait(started);
}
```

### Performance comparison with std::sync
다음은 std::sync::RwLock과 parking_lot::RwLock의 benchmark이다.
```rust
use std::sync::{RwLock as StdRwLock, Arc};
use parking_lot::RwLock as ParkingRwLock;
use std::thread;
use std::time::{Duration, Instant};

const NUM_THREADS: usize = 4;
const NUM_ITERATIONS: usize = 100_000;

fn run_benchmark_parking_rwlock(rw_lock: &Arc<ParkingRwLock<()>>) -> Duration {
    let start = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..NUM_THREADS {
        let rw_lock = rw_lock.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..NUM_ITERATIONS {
                let _guard = rw_lock.read();
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    start.elapsed()
}

fn run_benchmark_std_rwlock(rw_lock: &Arc<StdRwLock<()>>) -> Duration {
    let start = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..NUM_THREADS {
        let rw_lock = rw_lock.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..NUM_ITERATIONS {
                let _guard = rw_lock.read();
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    start.elapsed()
}

fn main() {
    let parking_rwlock = Arc::new(ParkingRwLock::new(()));
    let std_rwlock = Arc::new(StdRwLock::new(()));

    let std_duration = run_benchmark_std_rwlock(&std_rwlock);
    let parking_duration = run_benchmark_parking_rwlock(&parking_rwlock);

    println!("std::sync::RwLock: {:?}", std_duration);
    println!("parking_lot::RwLock: {:?}", parking_duration);
}
```

The benchmark was run on:  
processor - Ryzen 9 5950X  
OS - WSL2(ubuntu 20.04) on Windows 10  
rust version - 1.67.1

The results were:
```c
std::sync::RwLock: 31.902169ms
parking_lot::RwLock: 15.574434ms
```
결과는 parking_lot::RwLock이 이 벤치마크에서 std::sync::RwLock보다 약 2배 빠르다는 것을 보여준다.
그러나 다른 시나리오에서는 두 lock의 성능 벤치가 다르게 나올 수 있으며 최적의 선택은 프로그램의 특정 요구 사항에 따라 달라질 수 있다.

## 8. Work stealing with crossbeam and Rayon
### Explanation of work stealing algorithm
Crossbeam이 제공하는 기능 중 하나는 다중 스레드 시스템에서 로드 밸런싱을 위한 기술인 work stealing이다.
다중 스레드 시스템에서 task는 일반적으로 더 작은 하위 task로 분할되고 실행을 위해 사용 가능한 스레드 간에 분산된다.
그러나 task의 특성이나 스레드 속도의 차이로 인해 일부 스레드는 다른 스레드보다 일찍 task를 완료하고 다른 스레드가 여전히 사용 중인 동안 유휴 상태가 될 수 있다.
이러한 상황은 리소스 활용도를 저하시키고 전체 시스템 성능을 저하시킬 수 있다.

이 문제를 해결하기 위해 work stealing을 사용하여 사용 가능한 스레드 간에 워크로드를 동적으로 재분배한다.
Crossbeam에서 work stealing은 `work stealing deque`라는 데이터 구조를 사용하여 구현된다.
deque(double-ended queue)는 양쪽 끝에서 요소를 추가하거나 제거할 수 있는 데이터 구조이다.
work stealing deque는 우리가 아는 deque와 같이 다음의 두 가지 작업을 지원한다.

- push(): deque의 맨 위에 task를 추가한다.
- pop(): deque의 맨 아래에서 task를 제거하고 반환한다.

시스템의 각 스레드에는 자체 work stealing deque가 있다.
스레드에 task가 부족하면 다른 스레드의 deque 맨 아래에서 task를 `Steal`하려고 시도한다.
이렇게 하면 더 많은 task를 가진 스레드가 더 많은 task를 수행하는 반면 더 적은 task를 가진 스레드는 task를 훔쳐 바쁘게 일할 수 있다.

Crossbeam에서 work stealing deque는 fixed size의 array를 사용하여 구현된다.
각 스레드에는 자체 deque가 있으며 스레드에 task가 부족하면 훔칠 다른 deque를 무작위로 선택한다.
스레드가 다른 스레드의 deque에서 훔치고 있는 경우 가장 최근에 추가된 task이므로 cache-hot일 가능성이 더 높기 때문에 먼저 deque의 맨 위에서 훔치려고 시도한다.

전반적으로 Crossbeam의 work stealing은 다중 스레드 시스템의 스레드 간에 워크로드의 균형을 맞추는 효과적인 방법을 제공하여
리소스 활용률을 높이고 성능을 향상시킬 수 있다.

### Using Crossbeam and Rayon to implement work stealing

### Example of work stealing in action

## 9. Conclusion
### Recap of key concepts and features
### Importance of safe and efficient concurrent programming in Rust
### Future developments in Rust's concurrency landscape