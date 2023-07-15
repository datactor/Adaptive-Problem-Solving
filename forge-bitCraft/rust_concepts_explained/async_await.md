# Async/Await

## 1. Introduction
Asynchronous programming은 concurrent execution of code을 허용함으로써 더 빠르고 응답성이 뛰어난 애플리케이션을 작성할 수 있다.  
여기서 말하는 'concurrent execution of code'는 정확하게는 'asynchronously execute functions without blocking'으로,
여러 작업을 동시에 실행할 수 있는 프로그램의 기능을 나타낸다.
이는 프로그램이 blocking 없이 동시에 여러 기능을 실행할 수 있음을 의미한다.
파일 읽기 또는 네트워크 요청 응답 대기와 같은 장기 실행 작업은
작업이 완료될 때까지 기다리지 않고 다음 작업으로 이동하여 보다 효율적으로 처리할 수 있다.

Rust의 async/await는 `lazy evaluation` 원칙에 따라 구축된 언어 기능이다.  
즉, 코드가 즉시 실행되는 `eager evaluation`과 달리 필요할 때만 코드가 실행된다.
`lazy evaluation`에 익숙하지 않은 경우 [lazy vs eager](https://github.com/datactor/rust-problem-solving/blob/main/forge/rust_concepts_explained/lazy_vs_eager.md)
를 먼저 읽는 것이 좋다.

async/await를 사용하면 장기 실행 작업이 완료되기를 기다리는 동안 'pause'한 다음,
작업이 완료되면 실행을 재개할 수 있는 코드를 작성할 수 있다.
이렇게 하면 프로그램 실행을 blocking하지 않고 동시에 여러 작업을 수행할 수 있다.  
즉, 장기 실행작업으로 예상되는 작업에 `await`으로 'pause' 시키고 `Future`로 완료 신호를 받게 끔 구현해 놓은 후에(멀티 스레딩에서는 새로운 스레드를 열어서 완료 신호를 받는 방식으로 구현),
완료신호가 반환되면 `Future` trait은 이전의 코드지점으로 돌아와 실행을 재개할 수 있도록 `await` 구문을 해결한다.
여기서 'pause'시키는 것은 오로지 현재 코드의 지점이며, even though the context is switched, 이전에 cpu가 보냈던 명령은 다른 컴퓨팅 리소스가 그대로 수행한다.
즉, 여기서 말하는 'pause'는 오로지 context switching을 수행하는 것을 의미하며 작업 전체를 'pause'하는 것은 아니다.
이는 특히 I/O-bound 응용 프로그램에서 상당한 성능 향상을 가져올 수 있다.

### Brief explanation of what async/await is
Async/await은 비동기 프로그래밍 패턴으로, 복잡한 흐름 제어 없이 비동기 코드를 동기처럼 작성할 수 있도록 돕는다.
이 패턴은 CPU바운드 작업에 대해서는 이점이 없을 수 있지만, I/O 바운드 작업이나 타이머와 같은 대기 시간이 있는 작업을 효율적으로 처리하는데 유용하다.
Rust에서 async 키워드는 함수를 정의할 때 사용되며, 이 함수는 Future를 반환한다. Future는 미래에 완료될 작업을 나타내며, lazy evaluation을
기반으로 하고, 이는 비동기적으로 실행된다. await은 이 Future의 상태를 나타내는 Poll::Ready(T)를 기다리는데 사용된다.

async/await과 executor의 조합으로 만든 비동기 프로그래밍은 기본적으로 non-blocking
(즉, cpu가 io바운드 작업이나 타이머처럼 기다려야되는 작업을 만났을 때 쉬지 않고 수행한다.)이고,
대체로 선형적으로 수행되지만 비선형적으로 전환될 수 있다. 여기서 동기 프로그래밍에서 말하는 선형적이라는 것은 코드가 시퀀셜하게 수행된다
라는 의미도 내포하고 있어서, 타이머나 io바운드 작업을 만났을때는, 그 찰나의 순간에도 non-blocking을 유지하기 위해
시퀀셜을 깨버리고 이벤트 큐에서 다음 Task(Future)를 수행할 수 있다는 것이다.
비동기적 context 전환이 여기서 발생할 수 있다. 덕분에 이것은 I/O바운드 작업의 Future를 수행할 때,
단일 스레드에서도 병렬적으로 처리하는 것처럼 보이게 한다. non-blocking을 유지하도록 cpu가 쉬지 않도록 태스크를 선형적으로
수행하거나 전환하게 하기 위해 OS의 기능을 사용하여 runtime에서 관리한다.

### Motivation for using async/await
Rust 코드에서 async/await를 사용하려는 몇 가지 이유가 있다.
가장 일반적인 것 중 일부는 다음과 같다.

- concurrent execution of code를 허용하여 애플리케이션의 응답성 향상
- 스레드 생성 및 관리와 관련된 오버헤드 감소
- blocking 작업을 줄임으로써 보다 효율적이고 성능이 좋은 코드 작성
- 종종 외부 리소스가 사용 가능해질 때까지 기다려야 하는 I/O 작업을 더 쉽게 처리할 수 있다.

## 2. Async Functions and Futures

### Overview of async functions in Rust
Rust의 비동기 함수는 비차단 I/O 및 기타 동시 작업을 허용한다.
async/await 키워드를 사용하여 장기 실행 작업이 완료되기를 기다리는 동안 "pause"할 수 있는 함수를 정의한 다음,
작업이 완료되면 실행을 다시 시작한다. 이를 통해 컴퓨팅 리소스를 보다 효율적으로 사용하고 애플리케이션 응답성을 개선할 수 있다.

다음은 async 키워드의 직접적인 역할을 요약한 것이다.
1. `Future`의 생성 및 반환
2. 함수를 비동기 함수로 표시하여 `Future`의 완료를 기다리는 데 필요한 `await` 키워드를 사용할 수 있게 한다.
3. await 키워드가 사용될 때, compiler에게 함수 실행을 pause하고 재개하는 데 사용할 수 있는 상태 머신을 생성할 수 있게 한다.
4. async fn의 body code가 모두 완료되면 `Future`의 상태를 자동으로 `Poll::Ready(T)`로 변경하여,
   executor가 작업이 완료되었음을 감지할 수 있게 한다.

이러한 역할은 모두 함수가 스레드를 차단하지 않고 `Future`의 완료를 기다릴 수 있도록 하는 async/await의 기본 동작과 관련이 있다.

### How async functions return a Future
비동기 함수가 호출되면 `Future` trait 객체를 반환한다.
`Future` trait은 아직 완료되지 않은 비동기 계산을 나타낸다.
`Future`가 polling될 때 계산이 완료되면 Poll::Ready를 반환하고,
아직 완료되지 않았다면 Poll::Pending을 반환한다.
`Future`는 여러 비동기 작업을 함께 연결하는 데 사용할 수도 있다.

```rust
async fn my_func1() {
    println!("This is an async function");
}
```
위의 my_func1()을 async keyword를 사용하지 않고 구현하면 다음과 같다.
```rust
enum Poll<T> {
    Ready(T),
    Pending,
}

trait Future {
    type Output;
    // fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

fn my_func2() -> impl Future<Output = ()> {
    println!("This is an async function");
}
```

### Basic syntax of async functions
Rust에서 비동기 함수를 정의하려면 함수 시그니처 앞에 async 키워드를 사용하고 I/O 또는 타이머 작업(CPU외 리소스가 담당하는 작업)을
기다리는 동안 함수를 "pause"하기 위해 await 키워드를 사용한다.
비동기 함수는 동기 함수와 마찬가지로 Result 및 Option type을 사용하여 오류 및 누락된 값을 처리할 수도 있다.
다음은 비동기 함수의 예이다.

```rust
async fn learn_song() {
    let learning = time::Duration::from_sec(10);

    thread::sleep(learning);
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
```

```rust
async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
```

이 함수는 reqwest 크레이트를 사용하여 URL의 콘텐츠를 비동기적으로 가져오고 응답 본문이 포함된 Result를 반환하거나 요청이 실패하면 오류를 반환한다.

## 3. Await Expressions

### Overview of await expressions in Rust
Rust의 async/await 모델에서 `await`은 `Future`의 결과가 준비(Poll::Ready(T))될 때까지 함수 실행을 일시 중지하는 데 사용된다.
이 섹션에서는 Rust에서 await 표현식이 작동하는 기본 사항을 다뤄보자.

`Future`는 네트워크 요청이나 파일 읽기와 같이 아직 완료되지 않은 비동기 작업을 나타내는 trait이다.  
예를 들어 루틴 함수와, 서브 루틴인 비동기 함수에서 `await` 표현식을 만나면 서브 루틴 함수는 `Future`에서 `poll`메서드를 호출한다.
이 메서드는 관련 코드 스니펫(서브 루틴 함수)을 실행하고 `Future`에 `Poll` state를 반환한다(기본 값인 Pending state).
`Future`가 아직 준비 되지 않은 경우, poll 메서드는 `Poll::Pending` state를 반환하고, executor는 `Future`를 FIFO 대기열의 끝으로 푸시하고
executor는 thread::yield_now()를 호출하여 제어권을 OS scheduler로 넘긴다. OS scheduler는 실행할 다른 작업을 예약할 수 있다.
즉, Rust의 async/await에서는 직접 context switching하지 않고 OS scheduler를 이용한다.

- Note: 이것은 executor의 구현에 따라 다르다.
설명한 내용외에도 단일 스레드 executor가 busy-waiting(executor가 active하게 Future들을 폴링하여 준비된 것들을 실행, 여기서 심화되면 work-stealing)
을 사용하거나, 이벤트 기반 스케줄링(이벤트 루프와 결합하여 특정 이벤트에 의해 트리거 되는 태스크를 실행. e.g. tokio의 executor: 이벤트를 추적하는 epoll을 사용하는 Reactor + Task Scheduler)을 사용할 수 있다.

executor내의 FIFO 대기열을 통해 OS 스케줄러는 실행할 다른 작업을 예약하고 non-blocking context switching 할 수 있다(async/await에서는 기본적으로 thread switch가 아닌 process switch).
이렇게 되면 기존 thread에 대하여 차단하지 않고 OS 스케줄러에 의해 FIFO 대기열 순으로 스케줄링하며 코드를 실행할 수 한다.
나중에 Future가 준비되면 poll 메서드가 다시 호출되고 `Poll::Ready(T)` state를 반환한다.
그런 다음 Rust runtime은 async 함수 실행을 재개한다. 이 함수는 `await` 표현식이 발생한 지점부터 계속된다.
이를 통해 Rust compiler는 스레드를 차단하지 않고 I/O bound 작업을 처리할 수 있는 효율적이고 성능이 뛰어난 비동기 코드를 작성할 수 있다.

`await` 키워드는 함수에서 실행을 일시 중지하고 `Future`가 완료될 때까지 기다리는 것이 안전한 지점을 표시하는 데 사용된다.
`Future`는 실행을 재개할 준비가 되었을 때 런타임에 알릴 책임이 있으며, 이 시점에서 함수는 중단된 지점에서 계속할 수 있다.

`await` 키워드는 Rust의 async/await 모델의 가장 중요한 기능 중 하나인데, 이는 개발자가 더 읽기 쉽고 추론하기 쉬운 비동기 코드를 작성할 수 있게 해주기 때문이다.
비동기 작업으로 작업할 때에도 순차 프로그래밍 기술을 사용할 수 있으므로 효율적이고 유지 관리 가능한 코드를 더 쉽게 작성할 수 있다.

### How await suspends execution until a Future is ready
`await`이 호출되면 기다리고 있는 Future가 완료되었는지 확인한다.
그렇지 않은 경우 기능이 일시 중단되고 제어가 런타임으로 반환되어 다른 작업을 실행할 수 있다.
`Future`가 준비되면 기능이 재개되고 계속 실행된다.

Rust에서 HTTP 요청을 만드는 데 일반적으로 사용되는 reqwest crate의 예를 들어보자.

reqwest를 사용하여 비동기 HTTP GET 요청을 만들고 싶다고 가정.
URL의 응답 본문을 가져오고 Result<String, reqwest::Error>를 반환하는 비동기 함수를 정의할 수 있다.

```rust
async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
```
이 함수에서는 await 키워드를 사용하여 함수의 나머지 부분을 계속하기 전에 response future가 완료될 때까지 기다린다.
Future가 아직 완료되지 않은 경우 함수가 일시 중지되고 제어가 런타임으로 반환되어 다른 작업을 실행한다.

response future가 준비되면 함수가 다시 재개되어 응답 본문을 가져오기 위해 text future를 기다리는 다음 코드 줄을 계속 진행한다.

이것이 `.await` 표현식의 기본 연산이다.
기다리고 있는 `Future`가 완료될 때까지 현재 함수의 실행을 일시 중지한 다음 `Future`가 준비되면 실행을 재개한다.

요약하면 함수가 일시 중단되면 Rust runtime이 다른 작업을 실행하도록 스레드를 유지하고(non-blocking) 전환할 수 있다.
이를 통해 컴퓨팅 리소스를 보다 효율적으로 사용하고 앱 응답성을 향상시킬 수 있다.

### Basic syntax of await expressions
await은 Future 객체와 함께 사용되어 결과가 준비될 때까지 함수 실행을 일시 중단한다.

```rust
async fn my_function() -> Result<(), MyError> {
    let result = some_async_operation().await?;
    // Do something with the result
    Ok(())
}
```
여기서 some_async_operation()은 Future를 반환하는 함수이며 await 키워드는 완료를 기다리는 데 사용된다.
`?` 연산자는 Future 실행 중에 발생하는 모든 오류를 전파하는 데 사용된다.


## 4. Working with Futures
The core method of future, poll, attempts to resolve the future into a final value.
This method does not block if the value is not ready.
Instead, the current task is scheduled to be woken up when it’s possible to make further progress by polling again.
The context passed to the poll method can provide a Waker, which is a handle for waking up the current task.

When using a future, you generally won’t call poll directly, but instead .await the value.

Rust의 async/await 모델에서 `Future`는 비동기 코드 작성을 위한 기본 building block이다.
`Future`는 아직 완료되지 않았지만 미래의 어느 시점에서 결과를 생성할 수 있는 비동기 연산을 나타낸다.  

다음은 Rust std 라이브러리의 future 구현이다.
```rust
pub trait Future {
    /// The type of value produced on completion.
    #[stable(feature = "futures_api", since = "1.36.0")]
    type Output;
    #[lang = "poll"]
    #[stable(feature = "futures_api", since = "1.36.0")]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

#[stable(feature = "futures_api", since = "1.36.0")]
impl<F: ?Sized + Future + Unpin> Future for &mut F {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        F::poll(Pin::new(&mut **self), cx)
    }
}

#[stable(feature = "futures_api", since = "1.36.0")]
impl<P> Future for Pin<P>
    where
        P: ops::DerefMut<Target: Future>,
{
    type Output = <<P as ops::Deref>::Target as Future>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        <P::Target as Future>::poll(self.as_deref_mut(), cx)
    }
}
```

### Polling a Future for progress

Rust에서 executor는 반환된 값을 감시하기만 하고 Poll의 state를 변경하지는 않는다.
실제의 대부분의 구현에서 Poll의 state를 변경하는 역할은 Future의 poll() 메서드가 맡고 있다(Future의 poll() 메서드를 통해 Poll::ready() 호출,
또는 std::future::Ready struct의 poll() 메서드 호출).

다음은 Rust std 라이브러리의 Poll 구현이다.
```rust
/// Indicates whether a value is available or if the current task has been
/// scheduled to receive a wakeup instead.
#[must_use = "this `Poll` may be a `Pending` variant, which should be handled"]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(not(bootstrap), lang = "Poll")]
#[stable(feature = "futures_api", since = "1.36.0")]
pub enum Poll<T> {
    /// Represents that a value is immediately ready.
    #[lang = "Ready"]
    #[stable(feature = "futures_api", since = "1.36.0")]
    Ready(#[stable(feature = "futures_api", since = "1.36.0")] T),

    /// Represents that a value is not ready yet.
    ///
    /// When a function returns `Pending`, the function *must* also
    /// ensure that the current task is scheduled to be awoken when
    /// progress can be made.
    #[lang = "Pending"]
    #[stable(feature = "futures_api", since = "1.36.0")]
    Pending,
}

impl<T> Poll<T> {
    #[stable(feature = "futures_api", since = "1.36.0")]
    pub fn map<U, F>(self, f: F) -> Poll<U>
        where
            F: FnOnce(T) -> U,
    {
        match self {
            Poll::Ready(t) => Poll::Ready(f(t)),
            Poll::Pending => Poll::Pending,
        }
    }

    pub const fn is_ready(&self) -> bool {
        matches!(*self, Poll::Ready(_))
    }

    pub const fn is_pending(&self) -> bool {
        !self.is_ready()
    }

    pub fn ready(self) -> Ready<T> {
        Ready(self)
    }
}
```

`Future`로 작업할 때 `poll` 메서드를 사용하여 Future가 완료되었는지 확인할 수 있다.
poll 메소드는 `Poll` enum을 반환하는데, 이는 Future가 결과를 생성하기 전에 준비가 되었는지 또는 더 많은 작업을 수행해야 하는지를 나타낸다.

다음은 Future에서 poll 메서드를 사용하는 방법의 예다.

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// A simple future that returns a string after a delay
struct DelayedString {
    delay: u64,
    message: String,
}

impl Future for DelayedString {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the delay has elapsed yet
        match futures_timer::Delay::new(Duration::from_secs(self.delay)).poll_unpin(cx) {
            Poll::Ready(_) => Poll::Ready(self.message.clone()),
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn print_string_after_delay(delay: u64, message: String) {
    // Create a new DelayedString Future
    let delayed_string = DelayedString {
        delay,
        message,
    };

    // Poll the Future until it is ready
    loop {
        match delayed_string.poll().await {
            Poll::Ready(s) => {
                println!("{}", s);
                break;
            }
            Poll::Pending => {
                println!("Still waiting...");
            }
        }
    }
}
```

이 예에서 DelayedString Future는 delay 후 String을 반환한다.
print_string_after_delay()는 Future가 준비될 때까지 polling하고 그동안 메시지를 출력한다.

여기의 print_string_after_delay() 내에서 poll() 메서드는 루프 내에서 직접 호출된다.
그러나 대부분의 경우 poll을 직접 호출하는 것은 권장되지 않는다.
대신 ergonomic하고 유기적으로 error handling까지 설계된 async/await 구문에 의존해야 하는 것이 권장된다.
async/await 구문은 Future를 올바르게 polling하고 Task wakeup을 처리한다.

## 5. Pinning in Rust

### Overview of pinning in Rust
아래는 Rust std 라이브러리의 Pin의 내부 구현이다.
```rust
pub struct Pin<P> {
   // Long-term, `unsafe` fields or macro hygiene are expected to offer more robust alternatives.
   #[unstable(feature = "unsafe_pin_internals", issue = "none")]
   #[doc(hidden)]
   pub pointer: P,
}

impl<P: DerefMut> Pin<P> {
    pub fn as_mut(&mut self) -> Pin<&mut P::Target> {
        // SAFETY: see documentation on this function
        unsafe { Pin::new_unchecked(&mut *self.pointer) }
    }

    /// Assigns a new value to the memory behind the pinned reference.
    ///
    /// This overwrites pinned data, but that is okay: its destructor gets
    /// run before being overwritten, so no pinning guarantee is violated.
    #[stable(feature = "pin", since = "1.33.0")]
    #[inline(always)]
    pub fn set(&mut self, value: P::Target)
    where
        P::Target: Sized,
    {
        *(self.pointer) = value;
    }
}
```

이외의 추가적인 자세한 내용은 Rust의 [async 문서](https://rust-lang.github.io/async-book/04_pinning/01_chapter.html) 에서 살펴보자


#### Definition of pinning
Pinning은 Rust에서 메모리의 값을 "pinning"하는 행위를 가리키는 개념이다.
값이 "pinned"되면 프로그램 lifetime 동안 메모리 위치가 변경되지 않음을 의미한다.
즉, 값은 메모리의 해당 위치에 "pinned"되어 이동할 수 없다.

#### Why pinning is important in Rust
Pinning은 안전한 asynchronous 프로그래밍을 허용하기 때문에 Rust에서 중요하다.
비동기 프로그래밍에는 비동기 계산을 나타내는 Future 생성이 포함된다.
Future가 생성되면 관련 데이터의 메모리 위치가 변경되어 나중에 Future가 폴링될 때 오류가 발생할 수 있다.
Pinning은 관련 데이터의 메모리 위치가 변경되지 않도록 보장하여 Future를 안전하게 polling할 수 있도록 한다.

#### How pinning is implemented in Rust
Pinning은 `Pin` type을 사용하여 Rust에서 구현된다.
`Pin` type을 사용하면 값을 메모리의 현재 위치에 고정할 수 있다.
값이 고정되면 이동할 수 없으며 이동하려고 하면 컴파일러 오류가 발생한다.
고정된 값 내의 데이터에 액세스하기 위해 `Pin` type은 데이터를 안전하게 변경할 수 있는 as_mut() 및 get_mut() 메서드를 제공한다.

요약하면 pinning은 안전한 비동기 프로그래밍을 가능하게 하는 Rust의 중요한 개념이다.
Rust는 메모리에 값을 고정함으로써 Future의 관련 데이터가 변경되지 않도록 보장하여 Future를 안전하게 polling할 수 있다.

### Why pinning is important in async/await code

#### The problem with moving a Future before it completes
async/await code에서 Futures는 미래의 어느 시점에 완료될 것으로 예상되는 계산을 나타낸다.
Future가 생성될 때 아직 완료되지 않았을 수 있으므로 진행되는 동안 프로그램에서 제어하는 메모리 parts들을 이동/순환하며 작동한다.
그러나 완료되기 전에 Future를 이동하면 이동에 의해 변경될 수 있는 Future의 내부 상태에 따라 달라질 수 있으므로 정의되지 않은 동작이 발생할 수 있다.
이러한 작동에도 메모리의 위치가 변경되지 않게 해주는 보장이 Pin이다.

다음 예를 살펴보자.
```rust
async fn foo() -> u32 {
    // Some computation...
    42
}

async fn bar() {
    let f = foo();
    // Some code that moves the Future `f`...
    let result = f.await;
    println!("Result: {}", result);
}
```
여기서 Future 'f'는 결국 값 42를 반환할 일부 계산을 나타내는 Future를 반환하는 'foo()' 함수를 호출하여 생성된다.
pin 없이 코드가 복잡해지면, 프로그램이 완료되기 전에 정의되지 않은 동작이 발생할 수 있다.

#### How pinning helps to prevent this problem
Pinning은 Future가 완료되기 전에 이동하더라도 Future의 내부 상태에 안전하게 액세스할 수 있기 때문에 async/await 코드에서 중요하다.
Future를 메모리의 현재 위치에 고정함으로써 Rust는 Future의 내부 상태가 이동에 의해 영향을 받지 않도록 보장한다.

다음 예를 살펴보자.
```rust
async fn foo() -> u32 {
    // Some computation...
    42
}

async fn bar() {
    let mut f = Box::pin(foo());
    // Some code that moves the pinned Future `f`...
    let result = f.as_mut().await;
    println!("Result: {}", result);
}
```
여기에서 Future 'f'는 Future를 메모리의 현재 위치에 고정하는 Box::pin() 함수를 사용하여 생성된다.
그런 다음 Future는 프로그램의 다른 부분으로 이동되지만 내부 상태는 여전히 as_mut() 메서드를 사용하여 
Pin<> 형태를 Pin<&mut> 형태로 변환하여 액세스할 수 있다. await은 &mut 참조를 기반으로 작동하며,
이를 통해 가변 참조로 Future의 상태를 변경할 수 있다. 이는 Future가 성공적으로 완료되고 내부 상태가 이동의 영향을 받지 않음을 보장한다.

Rust의 Pinning은 안전한 type system을 기반으로 한다. 이를 통해 Future나 다른 타입이 고정된 상태에서만 사용될 수 있도록 한다.  
Rust는 Future를 type 시스템으로 검증된 Pinning으로 고정하고, 내부 상태가 이동에 의해 영향을 받지 않도록 보장하므로 Future가 안전하게 polling되고 완료될 수 있다.

즉, 값을 직접 변경하는 것이 아니라(이렇게 하면 할당과 같은 연산이 일어나면, 변수의 주소값은 그대로 유지되겠지만 내부 데이터의 값의 주소값이 변경된다.)
가변 참조로써 정의되지 않은 할당과 같은 연산은 제한하고, 데이터 주소값은 유지시킨다.

### Examples of using pinning in async/await code

#### Example 1: Using Pin<Box<T>>
```rust
use std::pin::Pin;
use std::future::Future;

async fn foo() {}

fn bar() -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(foo())
}
```
이 예제에서는 Pin<Box<T>> type을 사용하여 Future를 생성한다.
먼저 빈 () 출력을 반환하는 foo()라는 async fn을 정의한다.  
그리고 box type의 dyn Future<Output = ()> type을 반환하는 bar()라는 함수를 정의한다.
Box::pin() 메서드를 사용하여 foo() Future를 메모리의 현재 위치에 고정하고 박스형 Future로 반환한다.


#### Example 2: Using Pin<&mut T>
```rust
use std::pin::Pin;
use std::future::Future;

struct MyStruct {
    data: i32,
}

impl MyStruct {
    async fn my_method(&mut self) {}
}

fn my_function(s: &mut MyStruct) -> Pin<&mut impl Future<Output = ()>> {
    Pin::new(&mut s.my_method())
}
```
이 예제에서는 Pin<&mut T> type을 사용하여 메모리의 값에 대한 가변 참조를 고정한다.
빈 () 출력을 반환하는 my_method()라는 메서드를 사용하여 MyStruct struct를 정의한다.
그런 다음 MyStruct 인스턴스에 대한 mutable 참조를 취하고,
Pin::new() 메서드를 사용하여 my_method() Future에 고정된 참조를 반환하는 my_function()이라는 함수를 정의한다.

#### Example 3: Using Pin<&T>
```rust
use std::pin::Pin;
use std::future::Future;

struct MySharedStruct {
    data: i32,
}

impl MySharedStruct {
    async fn my_shared_method(&self) {}
}

fn my_shared_function(s: &MySharedStruct) -> Pin<&impl Future<Output = ()>> {
    Pin::new(&s.my_shared_method())
}
```
이 예제에서는 Pin<&T> type을 사용하여 메모리의 값에 대한 불변 참조를 고정한다.
빈 () 출력을 반환하는 my_shared_method()라는 메서드로 MySharedStruct struct를 정의합니다.
그런 다음 MySharedStruct 인스턴스에 대한 불변 참조를 취하고 Pin::new() 메서드를 사용하여
my_shared_method() Future에 고정된 참조를 반환하는 my_shared_function()이라는 함수를 정의한다.

### Unpin Trait

#### Definition of Unpin
Unpin trait은 type이 고정된 후 안전하게 이동할 수 있음을 나타내는 Rust의 Marker trait이다.
Unpin을 구현하는 type은 고정된 경우에도 안전하게 이동할 수 있으므로 Unpin을 구현하지 않는 유형보다 더 유연하다.

#### Why is Unpin important
Unpin은 비동기 프로그래밍에서 사용될 때 특정 type을 더 유연하게 할 수 있다.
type을 Unpin으로 표시함으로써 Rust는 type이 고정된 후 안전하게 이동할 수 있음을 보장한다.
이는 비동기 계산 중에 이동해야 하는 일부 type에 필수적이다.

#### How to implement Unpin
Rust에서 Unpin을 구현하려면 #[derive(Unpin)] trait을 사용하여 type을 Unpin으로 표시하기만 하면 된다.
이렇게 하면 type이 고정된 경우에도 type을 안전하게 이동할 수 있음을 Rust에 알린다.
type이 고정된 후 안전하게 이동할 수 없는 경우 type에 대해 고정 해제를 구현하지 않아야 한다.

일반적으로 Rust에서 Unpin을 구현하는 것은 권장되지 않는다. Pinning과 함께 제공되는 보장을 제거하기 때문이다.
예를 들면, 값이 Unpinned되면 완료되지 않은 경우에도 안전하게 이동할 수 있으므로 값이 polling 중인 Future의 일부인 경우 예기치 않은 동작이 발생할 수 있다.
즉, 그렇게 해야 할 강력한 이유가 있고 잠재적인 결과를 신중하게 고려한 경우가 아니라면,
async/await code와 함께 사용하도록 의도된 type에서 자체적으로는 Unpin을 구현하지 않는 것이 가장 좋다.

## 6. Understanding Context and Waker

### The role of Context in asynchronous programming

#### Definition of Context
Rust의 async/await 모델에서 Context는 Future의 poll 메서드에 전달되는 type이다.
'Context'는 Future가 executor와 상호 작용하는 데 필요한 정보를 제공하고 Future가 진행할 준비가 되었을 때 신호를 보낼 수 있도록 한다.

```rust
/// The context of an asynchronous task.
///
/// Currently, `Context` only serves to provide access to a [`&Waker`](Waker)
/// which can be used to wake the current task.
#[stable(feature = "futures_api", since = "1.36.0")]
pub struct Context<'a> {
    waker: &'a Waker,
    // Ensure we future-proof against variance changes by forcing
    // the lifetime to be invariant (argument-position lifetimes
    // are contravariant while return-position lifetimes are
    // covariant).
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}

impl<'a> Context<'a> {
   /// Create a new `Context` from a [`&Waker`](Waker).
   #[stable(feature = "futures_api", since = "1.36.0")]
   #[rustc_const_unstable(feature = "const_waker", issue = "102012")]
   #[must_use]
   #[inline]
   pub const fn from_waker(waker: &'a Waker) -> Self {
      Context { waker, _marker: PhantomData }
   }

   /// Returns a reference to the [`Waker`] for the current task.
   #[stable(feature = "futures_api", since = "1.36.0")]
   #[rustc_const_unstable(feature = "const_waker", issue = "102012")]
   #[must_use]
   #[inline]
   pub const fn waker(&self) -> &'a Waker {
      &self.waker
   }
}
```
여기의 `_marker`field는 `PhantomData` type으로, Rust의 `PhantomData`는 실제로 사용되지는 않지만, 해당 데이터와 라이프타임을 공유하기 위해 사용된다.
즉, Task가 완료되거나 삭제된 후 해당 Context가 사용될 수 없도록 하기 위함이다.
Task의 lifetime과 연계된 PhantomData가 존재함으로써, 작업이 삭제되면 PhantomData가 무효화되어 전체 Context를 무효화 시킨다.

이렇게 하면 부실하거나 잘못된 `Context`가 실수로 잘못된 Task를 깨우거나 해제된 메모리에 엑세스하는 데 사용되어
정의되지 않은 동작 또는 메모리 안전 문제로 이어질 수 있는 버그를 방지하는 데 도움이 된다.

#### How Context works in Rust's async/await model
Future의 poll 메서드가 호출되면 Context 객체가 전달된다.
Context에는 Future가 executor와 상호 작용할 수 있도록 하는 현재 작업에 대한 참조가 포함되어 있다.

Context는 또한 Waker 개체에 대한 참조를 제공한다. 'Waker'는 Future에서 진행 준비가 되었을 때 executor에게 알리는 데 사용된다.
이를 통해 executor는 future를 불필요하게 폴링하는 것을 방지하고 CPU 사용량을 줄여 성능을 향상시킬 수 있다.

#### Examples of using Context in Rust
다음은 Rust에서 Context를 사용하는 예이다.
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    counter: u32,
}

impl Future for MyFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.counter < 10 {
            self.counter += 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.counter)
        }
    }
}

fn main() {
   let mut future = MyFuture { counter: 0 };
   let mut cx = Context::from_waker(futures::task::noop_waker_ref());
   let mut pinned = Pin::new(&mut future);

   // Poll the future until it completes
   loop {
      match pinned.as_mut().poll(&mut cx) {
         Poll::Ready(output) => {
            println!("Future completed with output: {}", output);
            break;
         }
         Poll::Pending => {
            println!("Future not yet ready");
         }
      }
   }
}
```
이 예제에서는 10까지 세고 최종 개수를 출력으로 반환하는 사용자 지정 Future 'MyFuture'를 정의한다.
poll 메소드는 Context의 Waker 객체를 사용하여 Future가 아직 진행할 준비가 되지 않았음을 executor에게 알린다.

main에서 MyFuture의 새 인스턴스를 생성하고 아무것도 하지 않는 Waker로 Context를 초기화한다.
그런 다음 Pin을 사용하여 Future에 대한 mutable 참조를 만들고 준비될 때까지 poll 메서드를 반복적으로 호출한다.
poll을 호출할 때마다 Context 객체를 전달하여 Future가 executor와 상호 작용할 수 있도록 한다.
Future가 아직 준비되지 않은 경우 poll 메서드는 Poll::Pending을 반환하고 루프가 계속된다.
Future가 준비되면 poll 메서드는 Poll::Ready(output)을 반환하고 output을 print한다.

이 예제는 Rust에서 비동기 작업을 관리하기 위해 Context 및 Waker 객체가 어떻게 사용되는지 보여준다.
Future가 executor와 상호작용하고 진행할 준비가 되었을 때 신호를 보낼 수 있는 방법을 제공함으로써
Rust의 async/await 모델은 효율적이고 성능이 뛰어난 비동기 프로그래밍을 가능하게 한다.

### How Waker manages task wake-ups

#### Definition of Waker
`Waker`는 Future를 기다리고 있는 Task를 깨우는 역할을 하는 struct이다.
`Waker`는 기본적으로 executor에게 Task를 진행할 준비가 되었음을 알리는 데 사용할 수 있는 핸들이다.
```rust
#[repr(transparent)]
#[stable(feature = "futures_api", since = "1.36.0")]
pub struct Waker {
    waker: RawWaker,
}
```
#### Why Waker is important in Rust's async/await model
Task를 깨울 방법이 없다면 Future는 언제 진행할 준비가 되었는지 신호를 보낼 방법이 없다.
때문에 `Waker` struct는 Rust의 async/await 모델의 필수적인 부분이다.
이는 Future가 진행 준비가 되었을 때 executor에게 알릴 수 있기 때문이다.

#### How Waker works in Rust
Future가 polling되면 `Waker` 객체에 대한 참조를 받는다.
Future는 이 객체를 사용하여 `Waker`에서 `wake` 메서드를 호출하여 진행할 준비가 되었음을 알릴 수 있다.
이 메서드는 executor에게 Task를 진행할 준비가 되었음을 알리는 데 사용되며 executor는 실행할 작업을 예약할 수 있다.
```rust
pub trait Future {
    type Output;

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub struct Context<'a> {
    waker: &'a Waker,
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}

pub struct Waker {
    waker: RawWaker,
}

impl Waker {
    pub fn wake(self: Arc<Self>) {
       // The actual wakeup call is delegated through a virtual function call
       // to the implementation which is defined by the executor.
       let wake = self.waker.vtable.wake;
       let data = self.waker.data;

       // Don't call `drop` -- the waker will be consumed by `wake`.
       crate::mem::forget(self);
    }

    pub fn wake_by_ref(&self) {
       // SAFETY: This is safe because `Waker::from_raw` is the only way
       // to initialize `wake` and `data` requiring the user to acknowledge
       // that the contract of `RawWaker` is upheld.
       unsafe { (wake)(data) };
    }
}
```

#### Examples of using Waker in Rust
다음은 Rust에서 wake-ups를 관리하기 위해 Waker type을 사용하는 방법의 예이다:
```rust
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::task::waker_ref;

struct MyFuture {
    waker: Option<Arc<Mutex<Waker>>>,
    counter: u32,
}

impl MyFuture {
    fn new() -> MyFuture {
        MyFuture { waker: None, counter: 0 }
    }

    fn notify(&mut self) {
        if let Some(waker) = self.waker.take() {
            let waker = waker.lock().unwrap().clone();
            waker.wake();
        }
    }
}

impl Future for MyFuture {
    type Output = u32;

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.counter < 10 {
            self.counter += 1;
            self.waker = Some(Arc::new(Mutex::new(cx.waker().clone())));
            Poll::Pending
        } else {
            Poll::Ready(self.counter)
        }
    }
}

fn main() {
    let mut future = MyFuture::new();
    let waker = Arc::new(Mutex::new(waker_ref(&future)));
    let mut cx = Context::from_waker(&*waker.lock().unwrap());

    loop {
        match future.poll(&mut cx) {
            Poll::Ready(output) => {
                println!("Future completed with output: {}", output);
                break;
            }
            Poll::Pending => {
                println!("Future not yet ready");
                future.notify();
            }
        }
    }
}
```
이 예제에서는 최대 10까지 세고 최종 개수를 출력으로 반환하는 사용자 지정 Future 'MyFuture'를 정의한다.
poll 메서드는 Context의 Waker 객체를 사용하여 Future가 아직 진행할 준비가 되지 않았음을 executor에게 알린다.

'notify' 메소드는 Future가 아직 준비되지 않았을 때 호출되며, Future를 다시 polling해야 한다는 신호를 executor에게 알리기 위해 waker를 깨운다.

main에서 MyFuture의 새 인스턴스를 만들고 Future를 가리키는 Waker로 Context를 초기화한다.
그런 다음 준비가 될 때까지 poll 메서드를 반복해서 호출한다.
아직 준비되지 않은 경우 notify 메서드를 사용하여 Waker를 깨운다.
Future가 준비되면 poll 메서드는 Poll::Ready(output)을 반환하고 output을 print한다.

이 예제는 Future가 진행될 준비가 되었을 때 executor에게 알려서 Rust에서 비동기 작업을 관리하기 위해 Waker 객체가 어떻게 사용되는지 보여준다.
executor가 Future를 불필요하게 polling하지 않도록 함으로써 Rust의 async/await 모델은 효율적이고 성능이 뛰어난 비동기 프로그래밍을 가능하게 한다.


## 7. Executors

### Overview of Executors in Rust
Executors are the backbone of asynchronous programming in Rust,
responsible for scheduling and coordinating tasks across threads and managing the lifecycle of Futures.

특정 executor의 구현마다 다르지만, executor는 (일반적으로 FIFO ordering)대기열에 대해서 감시 루프를 돌려서
Task(Futures) 대기열에 대해서 라운드 로빈 방식으로
각 Future를 번갈아가며 대기열을 모두 순환할 때까지 poll 메소드를 호출한다.
순환할 때는 기본적으로 현재의 task를 pop시키고, 기본적으로 pop된 현재 작업이 표시된다.
여기서 현재의 task가 완료되지 않았다면 다시 futures 대기열에 현재 Task를 push_back시킨다.  
즉, executor는 Tasks(Futures) managing 및 execute를 담당한다.

여기의 executor는 대기열을 감시했을때, future들이 모두 Pending 상태라면 비차단 context 스위칭을 수행한다.
이후에 이벤트 루프에 의해 다시 executor의 차례가 와서 작동되어 대기열 목록이 올라가면, 다시 위의 작업을 반복한다.

여기서 executor에서 non-blocking context switching이 순간은 모든 queue가 Pending 상태일 때이다.

예를 들어 executor가 loop를 돌때, Poll::Ready(T)의 Future를 만나게 되면 곧바로 context switching 하지 않고,
Ready가 반환된 해당 Future(즉 .await 표현식을 만났던 async 함수)를 처리하고(Ready됨으로써 push_back 하지 않으며 Task queue에서 삭제된다),
executor의 구현에 따라 다음의 동작 중 하나를 수행한다.

### Managing Tasks with a Custom Executor
다음은 여러 Future 객체에 대해 모니터링 루프를 실행할 수 있는 executor 구현의 예이다.
```rust
use futures::task::{ArcWake, waker_ref};
use std::sync::Arc;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::RefCell;
use std::thread;

struct MyExecutor {
    tasks: RefCell<VecDeque<Arc<Task>>>,
    unparker: thread::Thread,
}

impl MyExecutor {
    fn new() -> MyExecutor {
        let thread_unparker = Arc::new((AtomicBool::new(false), thread::current()));
        let unparker = thread::spawn(move || {
            while !thread_unparker.0.load(Ordering::SeqCst) {
                thread::park();
            }
        });
        MyExecutor {
            tasks: RefCell::new(VecDeque::new()),
            unparker,
        }
    }

    fn spawn(&self, task: Arc<Task>) {
        self.tasks.borrow_mut().push_back(task);
        self.unparker.thread().unpark();
    }

    fn run(&self) {
        let waker = waker_ref(&MyWake::default());
        let mut context = Context::from_waker(&waker);

        loop {
            let mut pending = false;
            for task in self.tasks.borrow_mut().iter_mut() {
                if task.poll(&mut context).is_pending() {
                    pending = true;
                }
            }
            if !pending {
                break;
            }
            thread::yield_now();
        }
    }
}

struct Task {
    future: RefCell<Option<Box<dyn Future<Output = ()> + Send + 'static>>>>,
}

impl Task {
    fn new<F: Future<Output = ()> + Send + 'static>(future: F) -> Arc<Task> {
        Arc::new(Task {
            future: RefCell::new(Some(Box::new(future))),
        })
    }

    fn poll(&self, context: &mut Context<'_>) -> Poll<()> {
        let mut future = self.future.borrow_mut();
        match future.as_mut().unwrap().poll(context) { // 이 라인의 poll 메서드는 단순히 Poll의 상태를 반환하는 poll메서드가 아닌, Poll의 상태를 변경하는 custom poll 메서드이다.
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => Poll::Ready(()),
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        MyExecutor::spawn(&executor(), cloned);
    }
}

struct MyWake {
    pub awoken: AtomicBool,
}

impl Default for MyWake {
    fn default() -> Self {
        MyWake {
            awoken: AtomicBool::new(false),
        }
    }
}

impl ArcWake for MyWake {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.awoken.store(true, Ordering::SeqCst);
        executor().unparker.thread().unpark();
    }
}

fn executor() -> &'static MyExecutor {
    lazy_static! {
        static ref EXECUTOR: MyExecutor = MyExecutor::new();
    }
    &EXECUTOR
}

fn main() {
    let task1 = Task::new(async {
        println!("Task 1 started");
        delay_for(Duration::from_secs(1)).await;
        println!("Task 1 done");
    });

    let task2 = Task::new(async {
        println!("Task 2 started");
        delay_for(Duration::from_secs(2)).await;
        println!("Task 2 done");
    });

    executor().spawn(task1);
    executor().spawn(task2);
    executor().run();
}
```

## 8. Event loop in Asynchronous Programming

### Overview of Event loop in Rust
이벤트 루프는 executor를 포함한 external event들을 구동하고 스케줄링하는 역할을 한다.
이벤트 대기열 중 다음 queue로 넘어가기 위해 비차단 context 스위칭(여기서는 thread::yield_now(); 메소드를 통해)을 하여
다음의 이벤트를 thread에 할당하고 수행한다.
여기서도 동시성 프로그래밍을 보장하기 위해, executor의 loop의 구현방식과 비슷하게
이벤트 queue를 순환할 때, 이벤트 자체가 완료되지 않으면, 다시 이벤트 queue에 push back 시킨다.  
즉, event loop는 executor를 포함한 external event를 스레드에 할당하여 구동하는 역할을 한다.

Rust 자체에는 이벤트 루프가 내장되어 있지 않지만 tokio 또는 async-std와 같은 라이브러리에서 이벤트 루프 구현을 제공한다.
이러한 라이브러리는 OS 스케줄러 및 기타 I/O 시스템과 상호 작용하여 작업을 구동하고 외부 이벤트를 처리한다.
Rust에서 이벤트 루프를 구현할 수 있지만 I/O 이벤트 및 스케줄링을 처리하기 위해 OS 기능에 의존한다는 점에 유의해야 한다.

동시성 프로그래밍에서 이벤트 queue와 Task queue의 역할과 책임이 다르다.
그렇지만 둘은 nested queues set로 상위 구조(events queue)와 하위 구조(Tasks queue)의 구현이 유사하다.
예를 들어 각각의 대기열 순환, non-blocking context switch, push-back incomplete items to the queue 등이 있다.

### Implementing a Basic Event Loop
실행자 없이 기본 이벤트 루프를 구현하는 것은 이벤트 루프가 일반적으로 실행자가 실행할 작업을 구동하고 조정하는 역할을 하기 때문에 다소 어렵습니다. 그러나 작업이나 실행기를 명시적으로 관리하지 않고 I/O 이벤트와 타이머를 관리하는 간단한 이벤트 루프 구현을 제공할 수 있습니다.
다음은 I/O 이벤트 및 타이머에 대해 각각 mio 및 timer 크레이트를 사용하는 기본 이벤트 루프입니다.

```rust
use mio::{Events, Interest, Poll, Token};
use std::io;
use std::time::{Duration, Instant};
use timer::Timer;

const TIMER_TOKEN: Token = Token(0);

struct BasicEventLoop {
    poll: Poll,
    timer: Timer,
}

impl BasicEventLoop {
    fn new() -> io::Result<Self> {
        let poll = Poll::new()?;
        let timer = Timer::new();
        let timer_registration = timer.registration();
        poll.registry().register(
            &mut timer_registration,
            TIMER_TOKEN,
            Interest::READABLE,
        )?;

        Ok(BasicEventLoop { poll, timer })
    }

    fn run(&mut self) -> io::Result<()> {
        let mut events = Events::with_capacity(128);
        let timeout = Duration::from_secs(2);

        loop {
            self.poll.poll(&mut events, Some(timeout))?;

            for event in events.iter() {
                match event.token() {
                    TIMER_TOKEN => {
                        println!("Timer event triggered!");
                    }
                    _ => (),
                }
            }
            self.timer.set_timeout(Duration::from_secs(2), ());
        }
    }
}

fn main() -> io::Result<()> {
    let mut event_loop = BasicEventLoop::new()?;
    event_loop.run()
}
```
이 구현은 I/O 이벤트를 처리하기 위해 mio 크레이트를 사용하고 타이머를 처리하기 위해 timer 크레이트를 사용한다.
이벤트 루프는 이벤트를 폴링하고 그에 따라 처리한다.
이 예에서 이벤트 루프는 2초마다 트리거되는 타이머 이벤트만 관리한다.
이 구현에는 executor가 없으며 I/O 이벤트 및 타이머를 관리하는 이벤트 루프의 기본 구조만 보여준다.

### Integrating Event Loops with Executors and Wakers

주어진 코드에서 EventLoop 구조체는 본질적으로 실행자와 이벤트 루프의 역할을 결합한다.
task queuing, spawn, execute를 단일 구조로 관리한다.
보다 완전하고 복잡한 비동기 시스템에서 이벤트 루프와 실행자는 고유한 역할과 책임을 갖는다.

thread::yield_now()는 현재 프로세스를 일시 중지하고 다른 스레드가 실행할 기회를 제공한다.
다른 스레드가 작업을 마친 후 현재 스레드가 즉시 재개된다는 보장은 없다.

호출자 함수를 이벤트 루프에 넣는 것은 이벤트 루프의 특정 구현에 따라 다르다.
일반적으로 호출자 함수가 Future를 반환하는 비동기 함수인 경우 executor가 이벤트 루프에 추가할 수 있다.
executor는 호출자 함수가 반환한 Future가 준비되었음을 감지하면 호출자 함수 실행을 재개하고 이벤트 루프 처리를 계속할 수 있다.

다음은 이벤트 루프에 executor를 결합한 구현 예이다.
```rust
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::{self, Thread};

struct EventLoop {
    tasks: Arc<Mutex<VecDeque<Pin<Box<dyn Future<Output = ()>>>>>>,
    unparker: Thread,
}

impl EventLoop {
    fn new() -> EventLoop {
        let tasks = Arc::new(Mutex::new(VecDeque::new()));
        let thread_unparker = Arc::new((Mutex::new(false), thread::current()));
        let unparker = thread::spawn(move || {
            while !*thread_unparker.0.lock().unwrap() {
                thread::park();
            }
        });
        EventLoop {
            tasks,
            unparker,
        }
    }

    fn spawn(&self, task: impl Future<Output = ()> + 'static) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push_back(Box::pin(task));
        self.unparker.thread().unpark();
    }

    fn run(&self) {
        let waker = Arc::new(EventLoopWaker {
            unparker: self.unparker.thread().clone(),
        });
        let mut context = Context::from_waker(&waker);

        loop {
            let mut tasks = self.tasks.lock().unwrap();
            while let Some(mut task) = tasks.pop_front() {
                if let Poll::Pending = task.as_mut().poll(&mut context) {
                    tasks.push_back(task);
                }
            }
            drop(tasks);
            thread::yield_now();
        }
    }
}

struct EventLoopWaker {
    unparker: Thread,
}

impl Waker for EventLoopWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.unparker.thread().unpark();
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let future = async {
        println!("Hello from the caller function!");
    };
    event_loop.spawn(future);
    event_loop.run();
}
```
이 예제에서 호출자 함수는 메시지를 인쇄하는 간단한 비동기 블록이다.
EventLoop 구조체에는 Future를 가져와 작업 대기열에 추가하는 spawn 메서드가 있다.
run 메서드는 대기열에서 작업을 반복적으로 팝하고 준비가 될 때까지 polling한다.
Task가 준비되지 않은 경우 대기열로 다시 push_back되고 스레드는 thread::yield_now()를 사용하여 넘긴다.
'EventLoopWaker' 구조체는 'Waker' trait을 구현하고 작업을 실행할 준비가 되었을 때 이벤트 루프 스레드를 깨우는 데 사용된다.

메인 함수는 EventLoop의 인스턴스를 생성하고 호출자 함수를 작업으로 생성한 다음 run을 호출하여 이벤트 루프를 시작한다.
Task를 실행할 준비가 되면 이벤트 루프가 깨어나 호출자 기능을 실행한다.

## 9. Advanced Topics

### Async streams and sinks
#### Definition of async streams and sinks
일반적으로 사용되는 동기식 스트림 및 싱크는 blocking IO를 사용하여 스트림과 데이터를 송수신한다.
즉, 작업이 완료될 때까지 송신 또는 수신 코드가 blocking된다.
이 접근 방식은 간단하고 이해하기 쉽지만 특히 많은 스트림을 동시에 처리할 때 성능 문제가 발생할 수 있다.

반면에 비동기 스트림 및 싱크는 non-blocking IO를 사용하며 blocking 없이 여러 스트림을 동시에 처리하도록 설계되었다.
비동기 스트림 및 싱크를 사용하면 전송 또는 수신 코드가 blocking되지 않으므로 스트림에서 데이터를 전송하거나 수신하는 동안
애플리케이션이 계속 실행될 수 있다. 이는 특히 동시성이 높은 애플리케이션에서 성능을 향상시킬 수 있다.

multi-thread 스트림 및 싱크는 다중 스레드를 사용하여 다중 스트림을 동시에 처리한다.
이 접근 방식은 비동기 스트림 및 싱크와 유사하지만 구현하기가 더 복잡할 수 있으며 제대로 처리되지 않으면 동기화 문제가 발생할 수 있으며,
많은 리소스를 필요로 한다.

비동기 스트림은 비동기적으로 생성되고 비동기적으로 소비되는 값의 스트림이다.
즉, 단일 값 대신 일련의 값을 반환하는 Future type이다.
비동기 싱크는 값을 소비자에게 비동기적으로 보내는 방법이다.

Rust에서 비동기 스트림과 싱크는 'Stream' 및 'Sink' trait을 사용하여 구현되어 동시성에 대해 고도로 최적화된 복잡한 데이터 흐름을 생성할 수 있다.
이러한 trait을 사용하여 여러 스트림을 동시에 처리하는 성능이 뛰어나고 확장 가능한 애플리케이션을 만들 수 있다.

#### Implementing async streams and sinks in Rust
비동기 스트림 또는 싱크를 구현하려면 각각 Stream 또는 Sink trate를 구현하는 구조체를 정의해야 한다.
이 두 가지 trait은 모두 future crate에서 제공된다.

다음은 난수 시퀀스를 생성하는 비동기 스트림의 예이다.
```rust
use futures::Stream;
use rand::{thread_rng, Rng};

struct RandomNumberStream {
   count: usize,
}

impl Stream for RandomNumberStream {
   type Item = u32;

   fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
      let mut rng = thread_rng();
      if self.count < 10 {
         self.count += 1;
         Poll::Ready(Some(rng.gen()))
      } else {
         Poll::Ready(None)
      }
   }
}
```
여기에서 Stream trait을 구현하는 RandomNumberStream이라는 구조체를 정의한다.
poll_next 메서드는 난수를 생성하고 이를 Some 값으로 반환한다.
10개의 값을 생성한 후 None을 반환하여 스트림의 끝을 알린다.

다음은 RandomNumberStream을 사용하여 10개의 난수를 생성하고 print하는 방법의 예이다.
```rust
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = RandomNumberStream { count: 0 };
    while let Some(num) = stream.next().await {
        println!("{}", num);
    }
}
```
이 예제에서는 RandomNumberStream의 새 인스턴스를 만들고 future crate에서 제공하는 StreamExt trait을 사용하여 스트림을 소비한다.
while let 루프를 사용하여 스트림을 반복하고 각 값을 print한다. next() 메서드는 스트림의 다음 값으로 해석되는 Future를 반환한다.
우리는 이 Future가 다음 값을 얻기를 기다린다.

#### Using async streams and sinks in Rust
채팅 서버의 예를 들어 들어보자.
클라이언트로부터 메시지를 수신하고 연결된 모든 클라이언트에게 브로드캐스트하는 채팅 서버를 구현한다고 가정한다.

```rust
// Define Message type
struct Message {
   sender: String,
   content: String,
}
```

```rust
// Define server struct, which will keep track of all connected clients and incoming messages
use futures::StreamExt;
use futures::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, Sender, Receiver};

struct ChatServer {
   clients: Vec<Sender<Message>>,
   messages: Receiver<Message>,
}
```
clients는 연결된 각 클라이언트에 대해 하나씩 있는 Sender의 벡터이다.
서버가 메시지를 수신할 때마다 각 클라이언트의 'Sender'를 통해 메시지를 브로드캐스팅하여 연결된 모든 클라이언트에게 해당 메시지를 보낸다.

messages는 서버가 클라이언트로부터 메시지를 수신하는 데 사용하는 Receiver이다.
각 수신 메시지는 이 'Receiver'에 추가된다.

```rust
// implement the server's run method
impl ChatServer {
   async fn run(&mut self) -> Result<(), Box<dyn Error>> {
      while let Some(message) = self.messages.next().await {
         for client in self.clients.iter_mut() {
            client.send(message.clone()).await?;
         }
      }
      Ok(())
   }
}
```
여기서는 messages Receiver의 next() 메서드를 사용하여 다음 수신 메시지를 기다린다.
메시지가 도착하면 연결된 각 클라이언트의 Sender를 반복하고 send() 메서드를 사용하여 메시지를 브로드캐스트한다.

```rust
// Define a method to handle new client connections
async fn handle_client(mut stream: TcpStream, sender: Sender<Message>) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 1024];

    loop {
        let n = stream.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        let message = Message {
            sender: format!("{}", stream.peer_addr()?),
            content: String::from_utf8_lossy(&buf[0..n]).to_string(),
        };

        sender.send(message).await?;
    }

    Ok(())
}
```
이 함수는 각각의 새 클라이언트 연결에 대해 호출된다.
클라이언트의 TcpStream에서 들어오는 메시지를 읽고 서버의 messages Receiver로 보낸다.

```rust
// Finally, implement the main function that creates the ChatServer,
// listens for incoming client connections, and spawns a new task to handle each incoming connection
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let mut server = ChatServer {
      clients: vec![],
      messages: mpsc::channel(1024).1,
   };

   let listener = TcpListener::bind("localhost:8080").await?;
   println!("Listening on {}", listener.local_addr()?);

   loop {
      let (stream, _) = listener.accept().await?;
      println!("Accepted connection from {}", stream.peer_addr()?);

      let (tx, rx) = mpsc::channel(1024);
      server.clients.push(tx);

      let sender = server.messages.clone();
      tokio::spawn(async move {
         if let Err(e) = handle_client(stream, rx).await {
            eprintln!("Error: {}", e);
         }
      });
   }

   Ok(())
}
```
main() 함수는 mpsc::channel 메서드를 사용하여 빈 clients 벡터와 새 Receiver가 있는 새 ChatServer 인스턴스를 생성한다.
그런 다음 새 TcpListener 인스턴스를 만들고 포트 8080에서 수신 대기를 시작한다.

이어지는 무한 루프에서 서버는 'listener.accept()' 메서드를 사용하여 들어오는 클라이언트 연결을 수락한다.
들어오는 각 연결에 대해 handle_client() 함수는 tokio::spawn() 메서드를 사용하여 새 작업에서 호출된다.
handle_client() 함수는 클라이언트의 TcpStream 및 Sender<Message>를 args로 사용한다.
TcpStream에서 들어오는 메시지를 읽고 수신된 메시지로 새 Message 인스턴스를 만들고 Sender.send() 메서드를 사용하여 서버의 messages Receiver로 보낸다.

ChatServer 인스턴스의 messages Receiver는 StreamExt trait에서 제공하는 next() 메서드를 사용하여 루프에서 소비된다.
새 메시지가 수신될 때마다 for 루프는 Sender.send() 메서드를 사용하여 연결된 모든 클라이언트에 메시지를 보낸다.

handle_client 함수가 오류를 반환하면 표준 오류 스트림에 print된다.
handle_client 기능 자체는 여기에 표시되지 않지만 들어오는 클라이언트 메시지를 처리하고 ChatServer 인스턴스를 사용하여 다른 클라이언트에 보내는 것으로 가정한다.

이 구현은 비동기 스트림과 싱크를 사용하여 여러 클라이언트를 동시에 처리할 수 있는 고성능 채팅 서버를 구축하는 방법을 보여준다.
async/await 구문인 tokio::spawn()과 futures 및 tokio 크레이트에서 제공하는 Stream 및 Sink trait을 사용하면
복잡한 데이터 흐름을 쉽게 구축하고 여러 비동기 작업을 동시에 처리할 수 있다.

### Cancelling Futures
future와 async/await 구문을 사용하는 Rust의 비동기 프로그래밍은 여러 Task와 Task를 동시에 처리할 수 있는 고성능 코드를 작성하는 강력한 방법을 제공한다.
그러나 때로는 특히, 장기 실행 애플리케이션에서 실행 중인 future를 취소해야 할 필요가 있다.

#### The need for canceling Futures
여러 상황에서 Future를 취소해야 할 필요성이 있을 수 있다.
예를 들어 사용자가 진행 중인 Task를 취소하기로 결정하거나 제한 시간이 만료되어 장기 실행 작업을 취소해야 하는 경우이다.
Future를 취소하는 것은 더 이상 특정 작업을 수행할 필요가 없을 때에도 유용할 수 있다.

#### Implementing cancellation in Rust
Rust는 Future에 대한 cancellation을 구현하는 다양한 방법을 제공한다.
한 가지 방법은 'futures-util' crate에서 제공하는 `AbortHandle` 및 `Abortable` type을 사용하는 것이다.

다음은 장기 실행 future를 취소하기 위해 `Abortable`을 사용하는 예이다.
```rust
use std::time::Duration;
use tokio::{time, task};
use futures::future::{self, FutureExt};
use futures_util::future::AbortHandle;
use futures_util::future::Abortable;

async fn long_running_operation() {
   println!("Long running operation started");
   time::sleep(Duration::from_secs(10)).await;
   println!("Long running operation finished");
}

async fn run() {
   let (abort_handle, abort_registration) = AbortHandle::new_pair();
   let long_running_task = task::spawn(long_running_operation());
   let abortable_task = Abortable::new(long_running_task, abort_registration);

   time::sleep(Duration::from_secs(3)).await;
   abort_handle.abort();
}

#[tokio::main]
async fn main() {
   run().await;
}
```
이 예제에서는 먼저 단순히 10초 동안 휴면하는 'long_running_operation'을 정의한다.
그런 다음 `AbortHandle::new_pair` 메서드를 사용하여 `AbortHandle` 및 `AbortRegistration`을 생성하는 'run' 함수를 정의한다.
그런 다음 'run' 함수는 `task::spawn` 메서드를 사용하여 새 작업을 생성하고 `Abortable::new` 메서드를 사용하여 `Abortable` Future에 wrapping한다.
이렇게 하면 'long_running_operation'을 비동기적으로 실행하고 `abort_handle`을 사용하여 중단할 수 있는 새 작업이 생성된다.

그런 다음 'run' 함수는 'time::sleep' 메서드를 사용하여 3초 동안 대기하고 `abort_handle.abort()`를 호출하여 장기 실행 작업을 취소한다.

#### Best practices for handling cancellation in Rust
Rust에서 cancellation을 구현할 때 코드가 안전하고 효율적인지 확인하기 위해 몇 가지 모범 사례를 따르는 것이 중요하다.

다음은 몇가지 권장 tip이다.
- `futures-util` 크레이트에서 제공하는 `Abortable` type을 사용하여 장기 실행 Future를 wrapping하여 취소 가능하게 한다.
- `AbortHandle` 및 `AbortRegistration`을 사용하여 새로운 `AbortHandle` 및 `Abortable` pair를 만든다.
- 장기 실행 future에서 `AbortHandle::is_aborted` 메서드를 사용하여 취소 여부를 항상 확인하여 적절하게 취소할 수 있는지 확인한다.
- 일정 시간이 지나면 장기 실행 Future를 자동으로 취소하기 위해 time-out을 사용하는 것을 고려하기.
- I/O 작업과 같은 부작용이 있을 수 있는 Future를 취소할 때 주의하고 Future가 취소되더라도 제대로 정리되었는지 확인.

이러한 모범 사례를 따르면 Rust 애플리케이션에서 안전하고 효율적인 취소를 구현할 수 있다.

### Sharing state between Futures using Arc and Mutex

#### The challenge of sharing state in asynchronous programming
비동기 프로그래밍에는 종종 여러 작업 또는 Future 간에 공유 상태로 작업하는 것이 포함된다.
그러나 Task가 동시에 실행되고 동일한 데이터에 동시에 액세스할 수 있으므로
데이터 경합, 교착 상태 및 기타 동기화 문제를 비롯한 여러 가지 문제가 발생할 수 있으므로 작업 간에 상태를 공유하는 것은 어려울 수 있다.
Rust에서는 OS레벨에서 최적화된 Arc 및 Mutex type등의 공유 상태 Task를 위한 도구를 제공한다.

#### Implementing shared state in Rust
Arc 및 Mutex type을 사용해서 Task 간에 상태를 안전하게 공유할 수 있다.
Arc는 여러 작업에서 값의 소유권을 공유하는 데 사용되는 반면 Mutex는 한 번에 하나의 작업만 값에 액세스할 수 있도록 상호 배제 및 내부 가변성을 제공한다.

다음은 Arc와 Mutex를 사용하여 Task간에 상태를 공유하는 방법의 예이다.
```rust
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

struct SharedState {
    counter: Arc<Mutex<u32>>,
    sender: mpsc::UnboundedSender<String>,
}

impl SharedState {
    fn new(counter: Arc<Mutex<u32>>, sender: mpsc::UnboundedSender<String>) -> Self {
        Self { counter, sender }
    }

    async fn increment_counter(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        self.sender.send(format!("Counter: {}", *counter)).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let (sender, mut receiver) = mpsc::unbounded_channel();
    let shared_state = SharedState::new(counter.clone(), sender);

    tokio::spawn(async move {
        loop {
            shared_state.increment_counter().await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    while let Some(message) = receiver.recv().await {
        println!("{}", message);
    }
}
```
이 예제에서는 Arc<Mutex<u32>> 및 mpsc::UnboundedSender<String>을 포함하는 SharedState struct를 정의한다.
Arc<Mutex<u32>>는 Task 간에 u32 값의 소유권을 공유하는 데 사용되는 반면 mpsc::UnboundedSender<String>은 작업 간에 메시지를 보내는 데 사용된다.

counter 값을 증가시키고 보낸 사람에게 메시지를 보내는 increment_counter라는 SharedState struct에 대한 새 메서드를 정의한다.
counter 값은 한 번에 하나의 작업만 값에 액세스할 수 있도록 하는 Mutex에 의해 보호된다(lock).

기본 함수에서 새 Arc<Mutex<u32>> 및 mpsc::UnboundedSender<String>을 만들고 새 SharedState 인스턴스에 전달한다.
그런 다음 shared_state 인스턴스에서 increment_counter 메서드를 반복적으로 호출하고 호출 사이에 1초 동안 휴면하는 새 작업을 생성한다.

마지막으로 receiver를 반복하고 수신된 메시지를 print한다.

#### Best practices for working with shared state in Rust
Rust에서 공유 상태로 작업할 때 동기화 문제 및 기타 문제를 방지하기 위해 모범 사례를 따르는 것이 중요하다.
몇 가지 모범 사례는 다음과 같다.

- Minimize the amount of shared state: 필요한 Task 간에만 데이터를 공유하고, 공유할 필요가 없는 데이터는 공유하지 않는다.
- Use locks sparingly: lock은 경합을 일으키고 프로그램 속도를 저하시킬 수 있으므로 필요할 때만 사용하기.
- Avoid dead

## 10. Asynchronous Patterns and Best Practices

### Chaining Futures using combinators

더 복잡한 비동기식 워크플로우를 생성하기 위해 combinators를 사용하여 `Future`를 결합하고 변환할 수 있다.
가장 일반적인 combinators 중 일부는 다음과 같다.

- `and_then()`: 두 개의 Future를 함께 연결하여 첫 번째 Future의 출력을 두 번째 Future로 전달한다.
- `map()`: Future의 출력에 함수를 적용하여 변환된 출력으로 새로운 Future를 생성한다.
- `join()`: 두 개의 Future를 출력의 튜플을 생성하는 단일 Future로 결합한다.

다음은 and_then()과 map() combinator를 사용하여 두 개의 future를 연결하는 예이다.

```rust
async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

async fn fetch_url_and_count_words(url: &str) -> Result<usize, reqwest::Error> {
    reqwest::get(url)
        .await
        .and_then(|response| response.text())
        .map(|body| body.split_whitespace().count())
}
```
이 예제에서는 먼저 reqwest::get(url)을 호출하여 Response 개체로 확인되는 Future를 검색한다.
그런 다음 이 Future에서 and_then 메서드를 호출한다.
이 메서드는 Future가 값으로 확인되면 실행될 클로저를 사용한다.
이 클로저에서 Response 개체의 text 메서드를 호출한다.
이 메서드는 응답 본문을 String으로 해석하는 새로운 Future를 반환한다.

마지막으로 결과 Future에서 map 메소드를 호출하여 출력을 응답 본문의 단어 수인 usize로 변환한다.

and_then combinator를 사용하여 이 두 개의 future를 함께 연결하면 첫 번째 Future의 출력이 두 번째 Future의 입력으로 전달되는
두 개의 비동기 호출을 순서대로 만들 수 있다.

### Error handling with Result and the ? operator
실패할 수 있는 비동기 코드로 작업할 때 오류를 호출 스택 위로 전파하고 적절하게 처리할 수 있는 방식으로 오류를 처리하는 것이 중요하다.
동기식 표현인 Result type이나 `?` 연산자를 사용하여 여전히 async 함수에도 사용하여 오류 발생 가능성을 나타낼 수 있다.

다음은 reqwest 크레이트를 사용하여 URL을 가져올 때 Result 유형 및 ? 연산자를 사용하여 오류를 처리하는 방법의 예이다.
```rust
async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
```
이 예에서 fetch_url 함수는 성공 시 String과 함께 Result 값을 반환하고 실패 시 reqwest::Error를 반환한다.
`?` 연산자는 비동기 작업(예: reqwest::get 또는 response.text() 호출)이 Err 변형을 반환하는 경우 호출 스택 위로 오류를 전파하는 데 사용된다.

비동기 코드에서도 Result type과 `?` 연산자를 사용하여 효율적이고 읽기 쉽고 추론하기 쉬운 코드를 작성할 수 있는 방식으로 오류를 처리할 수 있다.

### Efficiently using async/await and avoiding common pitfalls
#### Avoiding blocking operations
I/O 또는 파일 액세스 대기와 같은 blocking 작업은 비동기 코드의 성능에 부정적인 영향을 미칠 수 있다.
코드가 효율적이고 반응이 빠르도록 non-blocking 작업을 사용하는 것이 권장된다.
```rust
async fn read_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
```
이 예제에서는 open() 및 read_to_string() 메서드의 비동기 버전을 사용하여 파일 내용을 읽는다.
이 메서드는 파일을 읽을 수 있을 때 비동기적으로 해결될 Future를 반환한다.

#### Using timeouts and cancellation
Future가 무한 루프에 빠지거나 완료하는 데 예상보다 오래 걸리면 비동기 코드를 디버깅하기 어려울 수 있다.
제한 시간 및 취소를 사용하면 이러한 문제를 방지하고 코드의 응답성을 개선하는 데 도움이 될 수 있다.

```rust
async fn run_with_timeout<F, T>(future: F, timeout: Duration) -> Result<T, ()>
where
    F: Future<Output = T>,
{
    select! {
        result = future => Ok(result),
        _ = tokio::time::delay_for(timeout) => Err(())
    }
}
```
이 예제는 시간 제한이 있는 Future를 실행하기 위한 Tokio 라이브러리의 매크로인 select! 매크로를 사용한다.
이 매크로는 Future를 실행하고 제한 시간 내에 완료되면 출력을 반환한다.
Future가 제한 시간 내에 완료되지 않으면 매크로가 오류를 반환한다.

#### Avoiding unnecessary heap allocations
힙 할당은 비동기 코드 속도를 늦추고 메모리 사용량을 증가시킬 수 있다.
코드를 효율적으로 실행하려면 가능한 한 불필요한 할당을 피하는 것이 중요하다.
```rust
async fn calculate_sum(numbers: &[u32]) -> u32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}
```
이 예에서는 가변 합계 변수(sum)와 for 루프를 사용하여 슬라이스에 있는 모든 숫자의 합계를 계산한다.
이 접근 방식은 숫자를 저장하기 위해 새 컬렉션에 메모리를 할당할 필요가 없다.

#### Using async blocks
async 블록은 읽기 쉽고 효율적인 방식으로 비동기 코드를 작성하기 위한 강력한 도구이다.
이를 통해 Future를 반환하는 코드 블록을 정의할 수 있으며, 이는 더 복잡한 비동기식 워크플로를 우리에게 익숙한 동기식 함수로 구축하는 데 사용할 수 있다.
```rust
async fn fetch_all_urls(urls: &[&str]) -> Vec<String> {
    let mut results = Vec::new();
    for url in urls {
        let response = reqwest::get(url).await.unwrap();
        let body = response.text().await.unwrap();
        results.push(body);
    }
    results
}
```
이 예에서는 async 블록을 사용하여 여러 URL의 콘텐츠를 동시에 가져온다.
`async` 블록을 사용하면 여러 비동기 작업을 병렬로 실행하고 응답 벡터로 확인되는 단일 Future를 반환할 수 있다.

#### Choosing the right executor
executor의 선택은 비동기 코드의 성능과 동작에 상당한 영향을 미칠 수 있다.
각각 고유한 강점과 약점이 있는 다양한 executor들이 있다.
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ...
}
```
이 예제에서는 tokio::main 매크로를 사용하여 새 Tokio 런타임을 생성하고 프로그램의 기본 executor로 설정한다.
이 executor는 처리량이 많은 네트워크 애플리케이션에 최적화되어 있으며 많은 비동기 워크로드에 적합하다.

## 11. Conclusion

### Recap of key points
비동기 프로그래밍의 기초와 Rust의 async/await 구문을 사용하여 비동기 코드를 작성하는 방법을 다뤄봤다.
Rust에서 Future와 Task가 어떻게 작동하는지, Stream과 Sinks를 사용하여 비동기 Context에서 데이터 스트림을 처리하는 방법에 대해 살펴봤다.
또한 비동기 코드에서 오류 및 cancellation을 처리하는 방법과 Arc<Mutex<Pin<Future>>를 사용하여 Future 간에 상태를 공유하는 방법을 다뤄봤다.

다음은 주요 내용의 요약이다.

- 비동기 프로그래밍을 사용하면 여러 I/O 작업을 동시에 처리할 수 있는 비차단 코드를 작성할 수 있다.
- Rust의 async/await 구문은 읽기 쉽고 추론하기 쉬운 동기식 문법의 비동기 코드를 작성하는 방법을 제공한다.
- Future와 Task는 Rust의 비동기 프로그래밍 모델의 building blocks들이며 비동기 계산을 표현하고 예약할 수 있도록 한다.
- stream 및 sink는 비동기 context에서 데이터 스트림을 처리할 수 있도록 하는 추상화이며 복잡한 데이터 흐름을 구축하는 데 사용할 수 있다.
- Error handling 및 cancellation은 비동기 프로그래밍의 중요한 측면이며 Rust는 이를 처리하는 데 도움이 되는 여러 가지 도구와 모범 사례를 제공한다.
- Future 사이에서 상태를 공유하는 것은 어려울 수 있지만 Rust의 Arc, Mutex type은 이를 위한 안전하고 효율적인 방법을 제공한다.

### Final thoughts and recommendations for learning more about async/await in Rust.
Rust의 async/await 구문을 처음 시작하는 경우 주제에 대한 훌륭한 소개를 제공하는 공식 Rust 문서부터 시작하는 것이 좋다.
또한 Rust에서 비동기 프로그래밍에 대해 더 많이 배울 수 있도록 도와주는 많은 훌륭한 Rust 리소스와 커뮤니티가 있다.

일반적으로 권장되는 일부 특정 리소스는 다음과 같다.

[The Rust Programming Language book](https://doc.rust-lang.org/book/title-page.html)  
[The Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)  
[The Tokio documentation](https://tokio.rs/)  
[The Rustonomicon](https://doc.rust-lang.org/nomicon/concurrency.html)  
The Rust async ecosystem  