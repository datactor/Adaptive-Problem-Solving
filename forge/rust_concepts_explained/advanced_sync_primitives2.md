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

## 6. Crossbeam atomic types

## 7. Work stealing with crossbeam and Rayon

## 8. Conclusion
