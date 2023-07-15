# closure & ownership
클로저는 주변 환경을 캡처할 수 있는 익명 함수이며, 간결하고 직관적인 방식으로
함수와 데이터로 작업하기 위한 강력한 도구이다. Rust에서는 참조 또는 값으로 변수를 캡처할 수 있다.
그러나 closure는 발견하기 까다로운 소유권과 관련된 몇가지 문제를 야기 할 수도 있다.  
closure에 대한 개요와 발생할 수 있는 몇 가지 잠재적인 문제를 예와 함께 정리해보자.

## closure
closure는 일반적으로 범위가 제한된 함수를 inline으로 정의하는 방법이다.
주변 scope에서 변수를 캡처할 수 있으므로 매개변수로 직접 전달되지 않은
데이터에서 작동할 수 있다.  
다음은 클로저의 예다.
```rust
/// closure that takes an integer and adds it
/// to a value captured from the surrounding scope:
let x = 10;
let add_x = |y| x + y;
assert_eq!(add_x(5), 15);
```
```rust
/// A "function-like" closure that doesn't capture anything.
/// You can think of it as a function defined inline.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let mut read = || lines.next().unwrap();
    let n = read().parse::<usize>().unwrap();
}
```

## Ownership Issue with Closure
클로저는 매우 유용할 수 있지만 소유권과 관련된 몇 가지 문제를 일으킬 수도 있다.
특히, 클로저는 해당 변수의 소유권을 갖는 방식으로 주변 범위에서 변수를 캡처할 수 있다.
이로 인해 변수를 사용하여 클로저가 완료되기 전에 변수가 이동되거나 삭제되는 상황이 발생할 수 있으며,
이는 런타임 오류를 야기할 수 있다.

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
closure는 변수의 소유권을 가져가는 방식으로 주변의 변수를 캡처할 때 소유권 문제를 일으킬 수 있다.
위의 예제에서 counter(Mutex)의 락을 얻고 값에 접근한 다음 mutable reference(num 값)와 함께
스레드로 전달하는 closure를 정의한다. 스레드 내부에서 Mutex가 감싼 값이 증가된다.

그러나 이 코드는 스레드 내부의 클로저가 counter의 소유권을 갖기 때문에 컴파일 되지 않는다.
closure가 Mutex와 그 내부 값을 '소유'하고 원래 counter 바인딩에 더 이상 메인 스레드에서
접근할 수 없기 때문에 소유권 문제가 발생한다. 따라서 클로저 외부에서 counter에 접근을 시도한다면
컴파일 에러가 발생한다.
소유권을 가져간 클로저가 존재할 동안 원본을 통해 값에 접근할 수 없다.
이후에 스레드가 완료되어 클로저가 소비되면, 소유권은 다시 원래 범위로 돌아가고 값에 접근할 수 있다.
