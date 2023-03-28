# Async/Await

## 1. Introduction
Asynchronous programming은 concurrent execution of code을 허용함으로써 더 빠르고 응답성이 뛰어난 애플리케이션을 작성할 수 있다.  
여기서 말하는 concurrent execution of code는 여러 작업을 동시에 실행할 수 있는 프로그램의 기능을 나타낸다.
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
Async/await은 코드의 동시 실행을 허용하는 프로그래밍 패러다임으로,
이를 통해 더 빠르고 반응이 빠른 애플리케이션을 작성할 수 있다.
Rust에서 async/await는 특수 구문의 조합과 Future 및 Poll traits의 사용을 통해 구현된다.

### Motivation for using async/await
Rust 코드에서 async/await를 사용하려는 몇 가지 이유가 있다.
가장 일반적인 것 중 일부는 다음과 같다.

- concurrent execution of code을 허용하여 애플리케이션의 응답성 향상
- 스레드 생성 및 관리와 관련된 오버헤드 감소
- blocking 작업을 줄임으로써 보다 효율적이고 성능이 좋은 코드 작성
- 종종 외부 리소스가 사용 가능해질 때까지 기다려야 하는 I/O 작업을 더 쉽게 처리할 수 있다.

## 2. Async Functions and Futures

### Overview of async functions in Rust
Rust의 비동기 함수는 비차단 I/O 및 기타 동시 작업을 허용한다.
async/await 키워드를 사용하여 장기 실행 작업이 완료되기를 기다리는 동안 "pause"할 수 있는 함수를 정의한 다음,
작업이 완료되면 실행을 다시 시작한다. 이를 통해 컴퓨팅 리소스를 보다 효율적으로 사용하고 애플리케이션 응답성을 개선할 수 있다.

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
위의 my_func1()을 async keyword를 사용하지 않고 간략하게 구현하면 다음과 같다.
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
Rust에서 비동기 함수를 정의하려면 함수 시그니처 앞에 async 키워드를 사용하고 장기 실행 작업을 기다리는 동안 함수를 "pause"하기 위해 await 키워드를 사용한다.
비동기 함수는 동기 함수와 마찬가지로 Result 및 Option type을 사용하여 오류 및 누락된 값을 처리할 수도 있다.
다음은 비동기 함수의 예이다.
```rust
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
예를 들어 루틴 함수와, 서브 루틴인 비동기 함수에서 `await` 표현식을 만나면 서브 루틴 함수는 `Future`에서 `poll`메서드를 호출하는 코드를 생성한다.
이 메서드는 관련 코드 스니펫(서브 루틴 함수)을 실행하고 `Future`에 `Poll` state를 반환한다(기본 값인 Pending state).
`Future`가 아직 준비 되지 않은 경우, poll 메서드는 `Poll::Pending` state를 반환하고, executor는 `Future`를 FIFO 대기열의 끝으로 푸시하고
executor는 thread::yield_now()를 호출하여 제어권을 OS scheduler로 넘긴다. OS scheduler는 실행할 다른 작업을 예약할 수 있다.
즉, Rust의 async/await에서는 직접 context switching하지 않고 OS scheduler를 이용한다.
executor내의 FIFO 대기열을 통해 OS 스케줄러는 실행할 다른 작업을 예약하고 non-blocking context switching 할 수 있다(async/await에서는 기본적으로 thread switching이 아닌 process switch).
이렇게 되면 기존 thread에 대하여 차단하지 않고 OS 스케줄러에 의해 FIFO 대기열 순으로 스케줄링하며 코드를 실행할 수 한다.
나중에 Future가 준비되면 poll 메서드가 다시 호출되고 `Poll::Ready(T)` state를 반환한다.
그런 다음 Rust runtime은 async 함수 실행을 재개한다. 이 함수는 `await` 표현식이 발생한 지점부터 계속된다.
이를 통해 Rust compiler는 스레드를 차단하지 않고 I/O bound 작업을 처리할 수 있는 효율적이고 성능이 뛰어난 비동기 코드를 작성할 수 있다.

`await` 키워드는 함수에서 실행을 일시 중지하고 `Future`가 완료될 때까지 기다리는 것이 안전한 지점을 표시하는 데 사용된다.
`Future`는 실행을 재개할 준비가 되었을 때 런타임에 알릴 책임이 있으며, 이 시점에서 함수는 중단된 지점에서 계속할 수 있다.

`await` 표현식은 Rust의 async/await 모델의 가장 중요한 기능 중 하나인데, 이는 개발자가 더 읽기 쉽고 추론하기 쉬운 비동기 코드를 작성할 수 있게 해주기 때문이다.
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

이것이 await 표현식의 기본 연산이다.
기다리고 있는 `Future`가 완료될 때까지 현재 함수의 실행을 일시 중지한 다음 `Future`가 준비되면 실행을 재개한다.

요약하면 함수가 일시 중단되면 Rust runtime이 다른 작업을 실행하도록 스레드를 유지하고(non-blocking) 전환할 수 있다는 점은 있다.
이를 통해 컴퓨팅 리소스를 보다 효율적으로 사용하고 앱 응답성을 향상시킬 수 있다.

### Basic syntax of await expressions
await는 Future 객체와 함께 사용되어 결과가 준비될 때까지 함수 실행을 일시 중단한다.

```rust
async fn my_function() -> Result<(), MyError> {
    let result = some_async_operation().await?;
    // Do something with the result
    Ok(())
}
```
여기서 some_async_operation()은 Future를 반환하는 함수이며 await 키워드는 완료를 기다리는 데 사용된다.
`?` 연산자는 Future 실행 중에 발생하는 모든 오류를 전파하는 데 사용된다(에러시 MyError 반환).

## 4. Working with Futures

### Polling a Future for progress

### Chaining Futures using combinators

### Handling errors with Result and ? operator


## 5. Executors

### Overview of Executors in Rust

### The Executor Trait

### Managing Tasks with a Custom Executor


## 6. Pinning in Rust

### Overview of pinning in Rust

### Why pinning is important in async/await code

### Examples of using pinning in async/await code


## 7. Advanced Topics

### Async streams and sinks

### Cancelling Futures

### Sharing state between Futures using Arc and Mutex


## 8. Best Practices and Pitfalls

### Best practices for writing efficient and maintainable async code

### Common pitfalls to avoid when working with async code


## 9. Conclusion

### Recap of key points

### Final thoughts and recommendations for learning more about async/await in Rust.