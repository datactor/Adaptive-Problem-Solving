# Unlocking the Crossbeam: Dive into Rust's Advanced Synchronization Primitives

## 1. Introduction
### Definition of synchronization primitives
동시 프로그래밍에서 동기화 프리미티브는 공유 리소스에 대한 액세스를 조정하거나
한 번에 하나의 스레드만 특정 리소스에 액세스할 수 있도록 하는 데 사용되는 메커니즘이다.
동기화 프리미티브의 예로는 lock, semaphores 및 atomic operations 등이 있다.

### Importance of synchronization primitives in concurrent programming
동시성은 최신 컴퓨팅의 필수 요소이지만 고유한 문제가 있다.
가장 큰 과제 중 하나는 여러 스레드가 안전하고 효율적인 방식으로 공유 리소스에 액세스할 수 있도록 하는 것이다.
동기화 프리미티브는 공유 리소스에 대한 액세스를 조정하고 race condition, deadlocks 및 기타 동시성 버그를 방지하는 방법을 제공하기 때문에 중요하다.

### Overview of what will be covered in the article
이 기사에서는 크로스빔 크레이트에서 제공하는 고급 동기화 기본 기능, std 라이브러리의 cell과 sync mudule에 중점을 두고
Rust의 동기화 기본 기능에 대한 포괄적인 이해를 돕는 것이 목적이다.

이러한 고급 동기화 프리미티브를 완전히 이해하려면 이전에 다룬 [Rc<RefCell< T >>](https://github.com/datactor/rust-problem-solving/blob/main/forge/rust_concepts_explained/rcRefcell_and_refCycle.md),
[Arc<Mutex< T >>](https://github.com/datactor/rust-problem-solving/blob/main/forge/rust_concepts_explained/arc_mutex.md),
[lazy_vs_eager](https://github.com/datactor/rust-problem-solving/blob/main/forge/rust_concepts_explained/lazy_vs_eager.md) 및
[async/await](https://github.com/datactor/rust-problem-solving/blob/main/forge/rust_concepts_explained/async_await.md)
과 같은 Rust의 기본 동시성 개념을 먼저 이해하는 것이 좋다.

crossbeam's channel, atomic types 및 work stealing API에 대해 살펴보기 전에
Rust의 cell과 sync를 포함한 Rust의 표준 동기화 프리미티브에 대한 개요부터 알아볼 것이다.
기사가 끝날 때쯤이면 이러한 동기화 프리미티브를 사용하여 안전하고 효율적인 동시 Rust 프로그램을 작성하는 방법에 대한 이해도가 높아져 있을 것이다.
