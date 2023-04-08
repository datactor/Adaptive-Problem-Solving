# Unlocking the Crossbeam
Previous article: [advanced_sync_primitives1](https://github.com/datactor/Rustic-data-solving/blob/main/forge/rust_concepts_explained/advanced_sync_primitives1.md),

이전 article에서는 동기 데이터를 보호하는 근본적인 struct들인 Cell 라이브러리의 Cell, RefCell, UnsafeCell struct들과,
여러 동기 primitives들로 구성된 sync 라이브러리의 Atomic types, Semaphore, Condvar, Barrier, mpsc, Once, Mutex, RwLock를 포함하여 Rust의 고급 동기 primitives들에 알아보았다.
여기서는 효율적이고 안전한 동시성 프리미티브를 제공하는 Rust 라이브러리인 Crossbeam를 unlocking한다.

스레드 간에 데이터를 전송하도록 설계된 Crossbeam 채널, 공유 데이터에 대한 lock-free operation을 제공하는 Crossbeam atomic type,
계산 병렬화를 위한 강력한 메커니즘인 Crossbeam 및 Rayon을 사용한 work stealing에 대해 알아보자.

이전 기사에서 다룬 고급 동기 프리미티브와 이 기사의 다음 주제에 대한 지식을 통해 Rust의 동시성 기능을 확실하게 이해하고
효율적이고 안전한 동시 프로그램을 작성할 수 있을 것이다(있어야 한다. 반드시).

## 4. Introduction to crossbeam

## 5. Crossbeam channels

## 6. Crossbeam atomic types

## 7. Work stealing with crossbeam and Rayon

## 8. Conclusion