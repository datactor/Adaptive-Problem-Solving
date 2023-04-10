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
### Explanation of Crossbeam atomic types
### Comparison with Rust's Atomic types
### Examples of using Crossbeam atomic types

## 7. Work stealing with crossbeam and Rayon
### Explanation of work stealing algorithm
### Using Crossbeam and Rayon to implement work stealing
### Example of work stealing in action

## 8. Conclusion
### Recap of key concepts and features
### Importance of safe and efficient concurrent programming in Rust
### Future developments in Rust's concurrency landscape