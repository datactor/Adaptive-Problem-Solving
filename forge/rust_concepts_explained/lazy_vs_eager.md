# Lazy Operations vs Eager Operations in Rust

## 1. Introduction
소프트웨어 개발에는 컴퓨팅에 대한 두 가지 주요 접근 방식인 lazy evaluation과 eager evaluation이 있다.

#### Explanation of lazy operations and eager operations
lazy evaluation은 실제로 필요할 때까지 계산을 연기하는 반면, eager evaluation은 즉시 계산을 수행한다.
Rust에서는 lazy와 eager 모두 지원한다.

이 글에서는 Rust의 lazy evaluation과 eager evaluation의 개념을 장단점, 사용 사례, 모범 사례를 살펴볼 것이다.
또한 lazy iterator, lazy colletion 및 기타 함수형 프로그래밍 기술을 포함하여 lazy evaluation과
eager evaluation을 위한 Rust의 내장 지원을 살펴볼 것이다.

먼저 lazy evaluation과 eager evaluation의 차이점을 정의하고 설명한다.
그런 다음 각 접근 방식에 대한 Rust의 지원에 대한 간략한 개요를 제공한다.
이 기사를 통해 탐구함으로써 Rust에서 lazy와 eager를 사용하는 시기와 방법,
그리고 Rust의 고유한 기능을 활용하는 효율적이고 효과적인 코드를 작성하는 방법을
명확하게 이해하는 것이 목표이다.

#### Brief overview of Rust's support for lazy and eager operations
Rust에서 지연 작업을 사용하려면 일반적으로 클로저 및 고차 함수와 같은 함수형 프로그래밍 기술을 사용한다.
예를 들어 map 함수를 사용하여 컬렉션의 각 항목을 느리게 변환하거나 필터 함수를 사용하여 컬렉션에서 특정 항목만 느리게 선택할 수 있다.

## 2. Lazy Operations

#### Overview of lazy operations in Rust
Rust의 lazy operation은 실제로 필요할 때까지 계산을 연기하는 작업이다.
이것은 무언가를 미리 계산하는 비용이 높거나 더 큰 데이터 세트의 일부만 계산해야 하는 상황에서 유용할 수 있다.
Rust는 lazy iterator, lazy collection, lazy evaluation을 포함하여 lazy operation을 위한 여러 메커니즘을 제공한다.

#### Advantages and disadvantages of lazy operations
lazy operation의 주요 이점은 필요할 때 필요한 만큼만 계산하여 계산 시간과 메모리 사용량을 절약할 수 있다는 것이다.
이는 upfront 컴퓨팅 비용이 높거나 더 큰 data set의 일부만 필요한 상황에서 특히 유용할 수 있다.

그러나 lazy operation에는 동일한 값을 반복적으로 계산해야 하는 경우 잠재적으로 전체 성능이 느려지고,
더 복잡한 코드가 생성될 수 있는 등의 단점이 있을 수 있다.

### 1) Lazy Iterators

#### Explanation of lazy iterators in Rust
Rust에서 iterator는 data collections를 순회하고 해당 요소를 처리하는 데 사용된다.
iterator를 생성하기 위해 Rust는 dataset의 어떤 요소가 다음에 처리될 것인지를 포함하여 반복 상태를 추적해야 한다.
이 상태는 dataset 자체와 별도로 저장되며 종종 iterator와 연결된 별도의 구조체 또는 객체로 저장된다.
즉 lazy이든 eager이든 공통적으로, iterator는 dataset의 상태를 나타내는 객체를 먼저 생성한다.

iterator에서 next() 메서드가 호출되면 Rust는 반복의 현재 상태에 따라 dataset에 액세스하고 다음 요소를 검색한다.
여기에는 특정 인덱스로 dataset에 액세스하거나 다음 요소를 검색하기 위해 다른 메커니즘을 사용하는 것이 포함될 수 있다.

lazy iterator에서는 실제로 필요할 때까지 계산을 연기하는 데 사용된다.
즉, dataset에 접근하지 않으며 값을 반환하지 않고 기다린다.
대신 필요할 때마다 iterator에서 해당하는 index로 dataset에 접근하여 값을 즉석에서 생성하여 반환한다.

```rust
struct MyIterator<'a> {
    data: &'a [i32],
    index: usize,
}

impl<'a> Iterator for MyIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.index < self.data.len() {
            let result = Some(self.data[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}
```
lazy와 eager iterator는 공통적으로 dataset에 대하여 위와 같은 struct의 객체를 생성한다.
eager는 모든 next를 즉시 순환하여 data에 접근하는 반면, lazy는 next() 메소드가 call되기 전까지는 데이터에 접근하지 않는다.

이는 collecions의 일부만 계산해야 하거나 모든 요소를 미리 계산하는 비용이 높을 때 특히 유용할 수 있다.

필요할 때까지 계산을 연기함으로써 게으른 반복자는 계산 시간과 메모리 사용량을 절약할 수 있다.
예를 들어 collections에 많은 수의 요소가 포함된 경우 모든 요소를 미리 계산하면 계산 시간과 메모리 사용량 측면에서 비용이 많이 들 수 있다.
lazy iterator를 사용하면 실제로 필요한 요소만 계산되므로 전체 계산 시간과 메모리 사용량이 줄어든다.

#### Examples of lazy iterators in Rust
Rust의 일반적인 게으른 반복자에는 `map`, `filter` 및 `take_while`이 포함된다.

`map` 함수는 컬렉션의 각 항목에 함수를 게으르게 적용하는 데 사용된다.
`filter` 기능은 컬렉션에서 특정 항목만 선택하는 데 사용되며
`take_while` 기능은 특정 조건이 충족될 때까지 컬렉션에서 항목을 선택하는 데 사용된다.

```rust
// Using the map method on an iterator to transform each item lazily:
let numbers = vec![1, 2, 3];
let squared = numbers.iter().map( | n| n * n);

// Using the filter method on an iterator to select only certain items lazily:
let numbers = vec![1, 2, 3];
let even = numbers.iter().filter( | n| n % 2 == 0);


// Using the take_while method on an iterator to select an item from a collection until a certain condition is met lazily.
fn main() {
    let v = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut i = v.iter();
    for met in i.by_ref().take_while(|&&v| v < 4) {
        println!("Take While: {}", met);
    }

    for other in i {
        println!("Rest: {}", other);
    }
}
```

### 2) Lazy Collections

#### Explanation of lazy collections in Rust
rust에서 collections는 dataset을 순환하여 순환된 값을 특정 type으로 모아서 집합으로 만들어 collection으로 반환할 때 사용한다.
lazy collection은 필요할 때만 콘텐츠를 평가하는 컬렉션이다.
앞에서 본 MyIterator와 같이 dataset과 index 필드를 사용하여,
dataset의 모든 값들을 즉시 순환하지 않고, call될 때 해당 index 위치의 특정 값을 처리한다.

이는 매우 큰 컬렉션이나 계산 비용이 많이 드는 collection을 처리할 때 유용할 수 있다.

#### Examples of lazy collections in Rust
Rust에서 일반적인 lazy collections에는 HashMap::entry, Vec::drain 및 VecDeque::split_off가 포함된다.

##### HashMap::entry
`HashMap::entry` 메서드는 해시맵에 값을 삽입하거나 업데이트할 때 지연된 계산을 허용하는 지연 항목 객체를 반환한다.
이것은 해시맵으로 작업할 때 불필요한 중복된 계산을 피하는 데 도움이 될 수 있다.
```rust
let mut map = std::collections::HashMap::new();
map.insert("key1", 1);
map.insert("key2", 2);

// Using entry() to lazily update a value in a hashmap
map.entry("key3").or_insert_with(|| expensive_computation());
```
이 예제에서 entry()는 해시맵의 값을 느리게 업데이트하는 데 사용된다.
or_insert_with() 메서드는 키가 맵에 이미 존재하지 않는 경우에만 실행되는 클로저를 사용한다.
이는 값이 해시맵에 이미 존재하는 경우 불필요한 계산을 피하는 데 유용할 수 있다.

##### Vec::drain
`Vec::drain` 메서드는 벡터에서 항목을 지연 제거하는 반복자를 반환한다.
이것은 벡터의 크기가 매우 크고 한 번에 모든 항목을 제거하는 것이 계산 비용이 많이 드는 경우에 유용할 수 있다.
```rust
let mut vec = vec![1, 2, 3, 4, 5];

// Using drain() to lazily remove items from a vector
let new_vec: Vec<_> = vec.drain(2..).collect();
```
이 예제에서 drain()은 벡터에서 항목을 지연 제거하는 데 사용된다.
'collect()' 메서드는 drain된 항목을 새 벡터로 수집하는 데 사용됩니다.
이것은 벡터의 크기가 매우 크고 한 번에 모든 항목을 제거하는 것이 계산 비용이 많이 드는 경우에 유용할 수 있다.

여기서 주의할점은 collect() 메소드는 eager 메소드이기 때문에 drain() 메서드는 collection에 의해 즉시 call되기 때문에
laziness는 의미가 없어진다.

```rust
let mut vec = vec![1, 2, 3, 4, 5];
let drained = vec.drain(2..);

// Process the drained items lazily
for item in drained {
    println!("Drained item: {}", item);
}
```
이 예에서 drain() 메서드는 인덱스 2부터 시작하여 vec에서 항목을 지연 제거하는 데 사용된다.
resulting iterator는 drained에 할당되고 for 루프를 사용하여 지연 처리된다.
이렇게 하면 실제로 필요한 항목만 처리되므로 대규모 컬렉션을 처리할 때 계산 시간과 메모리 사용량을 절약할 수 있다.

##### VecDeque::split_off
`VecDeque::split_off` 메서드는 원래 deque 뒤에 있는 요소가 포함된 새 deque를 반환한다.
이렇게 하면 원래 deque의 모든 요소를 복사하지 않고도 새 deque를 만들 수 있다.
```rust
let mut deque = std::collections::VecDeque::new();
deque.push_back(1);
deque.push_back(2);
deque.push_back(3);

// Using split_off() to lazily create a new deque
let new_deque = deque.split_off(1);
```
이 예제에서 split_off()는 원래 deque 이후의 요소를 포함하는 새 deque를 지연 생성하는 데 사용된다.
이렇게 하면 원래 deque의 모든 요소를 복사하지 않고 새 deque를 만들 수 있다.

### 3) Lazy Evaluation

#### Explanation of lazy evaluation in Rust
Rust의 lazy evaluation은 실제로 필요할 때까지 계산을 미루는 과정을 말한다.
즉 결과가 필요할 때만 expression을 평가하는 전략이다.
이는 결과가 필요하지 않더라도 가능한 한 빨리 모든 표현식을 평가하는 eager evaluation과 대조된다.
이는 upfront 컴퓨팅 비용이 높거나 더 큰 dataset의 일부만 필요한 상황에서 유용할 수 있다.

#### Examples of lazy evaluation in Rust
Rust에서 lazy evaluation의 일반적인 사용 사례 중 하나는
변수에 할당하고 다른 함수에 인수로 전달할 수 있는 익명 함수인 `closure`를 사용하는 것이다.
Rust의 closure는 느리게 평가된다. 즉, 실제로 호출될 때까지 계산이 지연된다.
```rust
let x = 5;
let y = 10;

let add = |a, b| a + b;

let result = add(x, y);
```

```rust
fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let mut read = || lines.next().unwrap();
    let n = read().parse::<usize>().unwrap();
}
```

`Future`는 Rust에서 lazy evaluation을 구현하는 또 다른 방법이다.
Future는 아직 사용할 수 없지만 미래에 비동기적으로 계산될 값을 나타낸다.
Future는 실제로 필요할 때까지 계산을 연기하거나 동시에 계산을 수행하는 데 사용할 수 있다.
예를 들어, Future는 프로그램의 나머지 부분이 계속 실행되는 동안 백그라운드에서 수행되는 비용이 많이 드는 계산의 결과를 나타낼 수 있다.
```rust
async fn expensive_computation() -> i32 {
    // simulate an expensive computation that takes some time
    std::thread::sleep(std::time::Duration::from_secs(5));
    42
}

fn main() {
    let fut = async {
        let result = expensive_computation().await;
        println!("The result of the expensive computation is {}", result);
    };
    // Do other work while the expensive computation is happening in the background
    std::thread::sleep(std::time::Duration::from_secs(2));
    // Wait for the future to complete
    futures::executor::block_on(fut);
}
```

`lazy static`은 전역 변수를 느리게 초기화할 수 있도록 하는 Rust의 기능이다.
lazy static이 정의되면 해당 값이 즉시 계산되지 않는다.
대신 변수가 실제로 처음 사용될 때 계산된다.
이는 복잡한 데이터 구조를 초기화하거나 특정 상황에서만 필요한 값의 계산을 연기하는 데 유용할 수 있다.
```rust
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MAP: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("foo", 42);
        m.insert("bar", 69);
        m
    };
}

fn main() {
    // The hashmap is not calculated until first used
    let value = MAP.get("foo");
    println!("The value is {:?}", value);
}
```

## 3. Eager Operations

### Overview of eager operations in Rust
Eager evaluation은 Rust의 기본 evaluation strategy로, 대부분의 메서드는 eager이다.
일반적으로 대부분의 사용 사례에서 더 효율적이고 편리하기 때문에 Eager evaluate하도록 설계되었다.

Rust의 Eager evaluation은 결과가 즉시 필요하지 않더라도 가능한 한 빨리 표현식을 평가하는 과정을 말한다.
이것은 실제로 필요할 때까지 계산을 연기하는 lazy evaluation과 대조된다.

### Advantages and disadvantages of eager operations
eager evaluation은 계산 비용이 상대적으로 낮거나 모든 값이 결국 사용되는 상황에서 유리할 수 있다.
즉시 평가는 또한 모든 계산이 미리 발생하도록 하여 프로그램 논리를 단순화할 수 있다.
그러나 계산된 값의 일부만 사용되거나 계산 비용이 상대적으로 높은 상황에서는 낭비가 될 수 있다.

### Examples of purposeful eager operations in Rust
Rust에서 일반적으로 대부분의 함수는 eager evaluation이지만 즉시 처리해주는 것이 유리한 상황임을 인지하고,
목적성을 갖고 eager evaluation를 사용하는 예는 다음과 같다.

- 일부 값이 사용되지 않더라도 배열 또는 벡터의 모든 값을 미리 초기화한다.
- 파일의 일부만 사용하더라도 전체 파일을 한 번에 메모리로 읽는다.
- 안전을 위해 나중에 프로그램에서 사용하거나 사용하지 않을 수 있는 값을 미리 계산한다.

## 4. When to Use Lazy Operations vs Eager Operations

#### Discussion of when to use lazy vs eager operations in Rust
lazy evaluation과 eager evaluation은 Rust에서 두 가지 뚜렷한 평가 전략이며,
각각 고유한 장점과 단점이 있다. 각 접근 방식을 언제 사용해야 하는지 이해하면 보다 효율적이고
유지 관리 가능한 코드를 작성하는 데 도움이 될 수 있다.

##### Advantages and Disadvantages of Lazy Evaluation

다음은 lazy evaluation의 장점이다.

- 계산 시간 단축: 실제로 필요할 때까지 계산을 연기함으로써 게으른 평가는 결코 사용되지 않는 값의 불필요한 계산을 피함으로써 계산 시간을 절약할 수 있다.
- 메모리 사용량 감소: 필요한 값만 계산하기 때문에 지연 평가는 불필요한 값의 저장을 피함으로써 메모리를 절약할 수 있다.
- 향상된 유연성: 필요에 따라 값을 계산함으로써 지연 평가는 정확한 계산 요구 사항을 미리 알 수 없는 상황에서 즉시 평가보다 더 많은 유연성을 제공할 수 있다.
  
그러나 lazy evaluation에는 몇 가지 단점도 있다.

- 복잡성 증가: 지연 평가는 계산 타이밍과 순서를 신중하게 관리해야 하므로 코드베이스에 추가 복잡성을 도입할 수 있다.
- 성능 저하: 경우에 따라 지연 평가는 특히 계산 비용이 상대적으로 낮을 때 즉시 평가보다 느릴 수 있다.
- 어려운 디버깅: 지연 평가를 사용하는 디버깅 코드는 즉시 평가를 사용하는 디버깅 코드보다 더 어려울 수 있다. 프로그램 실행 후반까지 오류가 포착되지 않을 수 있기 때문이다.

##### Examples of When to Use Lazy Evaluation
lazy evaluation은 다음과 같은 상황에서 유용할 수 있다.

- 계산 비용이 높고 계산된 값의 일부만 필요할 때.
- infinite streams 또는 비동기 작업의 경우와 같이 실제로 필요할 때까지 계산을 연기할 수 있다.
- 정확한 계산 요구 사항은 미리 알 수 없을 때.

Rust에서 지연 평가를 사용하는 경우의 예는 다음과 같다.

- 모든 값을 미리 계산하는 것이 비실용적인 매우 큰 데이터 세트를 처리할 때.
- 나중에 프로그램 실행에 필요할 수도 있고 필요하지 않을 수도 있는 복잡한 값을 계산할 때.
- 결과를 즉시 사용할 수 없는 네트워크 요청과 같은 비동기 작업 작업.

##### The impact of lazy evaluation on Rust's ownership model
Rust에서 소유권 모델은 주어진 시간에 메모리 조각에 대해 단 한 명의 소유자만 있고 소유자가 범위를 벗어나면 해당 메모리가 할당 해제되도록 한다.
이 모델은 동일한 메모리 조각에 대한 여러 참조가 동시에 존재하지 않도록 하여 데이터 경합 및 기타 메모리 관련 버그를 방지하는 borrow checker에 의해 시행된다.

lazy evaluation은 몇 가지 방식으로 Rust의 소유권 모델에 영향을 미칠 수 있다.
lazy evaluation를 사용하는 경우 필요할 때까지 변수 생성을 연기할 수 있으므로 메모리 사용량을 줄이는 데 도움이 될 수 있다.
그러나 이러한 변수가 제대로 관리되지 않으면 ownership 및 borrowing 문제로 이어질 수 있다.
예를 들어 lazy variable이 함수 내에서 생성되고 참조로 반환되는 경우 참조가 사용되기 전에 범위를 벗어날 수 있으므로 댕글링 참조 및 런타임 오류가 발생할 수 있다.
```rust
fn get_lazy_value() -> &'static i32 {
    lazy_static! {
        static ref LAZY_VALUE: i32 = {
            println!("Initializing lazy value");
            42
        };
    }
    &LAZY_VALUE
}

fn main() {
    let lazy_ref = get_lazy_value();
    // At this point, the lazy value has not been initialized yet
    println!("Lazy value: {}", lazy_ref);
}
```
이 예에서는 lazy_static 매크로를 사용하여 LAZY_VALUE라는 지연 평가 변수를 생성한다.
get_lazy_value 함수는 이 변수에 대한 참조를 반환한다.
그러나 변수는 느리게 평가되므로 함수가 반환될 때 아직 초기화되지 않을 수 있다.
그런 다음 참조가 함수 외부에서 사용되면 댕글링 참조 및 런타임 오류가 발생할 수 있다.

이 문제를 피하기 위해 Rust의 Rc 또는 Arc type을 사용하여 lazy variable의 소유권을 관리할 수 있다.
이러한 유형은 데이터가 조기에 삭제되지 않도록 하면서 동일한 데이터에 대한 여러 참조를 허용한다.

다음은 lazy variable를 관리하기 위해 Rc를 사용하는 업데이트된 예다:
```rust
use std::rc::Rc;

fn get_lazy_value() -> Rc<i32> {
    lazy_static! {
        static ref LAZY_VALUE: Rc<i32> = {
            println!("Initializing lazy value");
            Rc::new(42)
        };
    }
    Rc::clone(&LAZY_VALUE)
}

fn main() {
    let lazy_ref = get_lazy_value();
    // Now, the lazy value is guaranteed to be initialized before we use it
    println!("Lazy value: {}", lazy_ref);
}
```
이 예제에서는 Rc를 사용하여 lazy variable의 소유권을 관리한다.
get_lazy_value 함수는 이제 참조 카운터에서 관리하는 변수에 대한 Rc 참조를 반환한다.
이렇게 하면 변수가 중간에 삭제되지 않고 참조가 항상 유효하다.

다음으로 eager evaluation은 소유권과 차용이 적절하게 관리되도록 하는 데 도움이 될 수 있지만 불필요한 메모리 사용과 성능 저하로 이어질 수도 있다.
예를 들어 전체 파일이 일부만 사용되더라도 열심히 메모리에 로드되면 불필요한 메모리 사용 및 성능 저하가 발생할 수 있다.

전반적으로 Rust의 소유권 모델에 대한 lazy evaluation과 eager evaluation의 영향을 신중하게 고려하고 적절하게 사용하는 것이 중요하다.
소유권과 차용의 적절한 관리는 메모리 관련 버그를 피하고 Rust 프로그램의 안전성과 정확성을 보장하는 데 중요하다.

##### Examples of When to Use Eager Evaluation
즉시 평가는 일반적으로 대부분의 사용 사례에서 지연 평가보다 더 효율적이고 편리하다.
Rust에서 즉시 평가를 사용하는 경우의 예는 다음과 같다.

- 계산 비용이 상대적으로 낮은 작은 데이터 세트 처리.
- 프로그램 실행에서 나중에 필요하다고 보장되는 값을 미리 계산.
- 일부 값이 사용되지 않는 경우에도 배열 또는 벡터의 모든 값을 사전 초기화.

#### Trade-offs between laziness and eagerness
Rust에서 lazy evaluation과 eager evaluation 사이의 선택은 종종 성능,
메모리 사용 및 코드 복잡성의 절충을 수반한다.
예를 들어 지연 평가는 경우에 따라 계산 비용을 절약하고 메모리 사용량을 줄일 수 있지만,
적절하게 사용하지 않으면 오버헤드가 추가되고 성능이 저하될 수 있다.

일반적으로 다음 고려 사항은 Rust에서 lazy evaluation과 eager evaluation 사이의 결정을 판단하는 데 도움이 될 수 있다.

- 계산 비용이 높고 모든 값이 즉시 필요하지 않은 경우 lazy evaluation 사용을 고려.
- 계산 비용이 낮고 결국 모든 값이 필요한 경우 eager evaluation 사용을 고려.
- 메모리 사용량이 우려되는 경우 불필요한 값을 메모리에 저장하지 않도록 lazy evaluation을 사용하는 것을 고려.
- 성능이 중요한 경우 lazy evaluation의 오버헤드를 피하기 위해 eager evaluation을 사용하는 것을 고려.
- lazy 초기화를 사용하여 실제로 필요할 때까지 값 계산을 연기.
- eager 초기화를 사용하여 프로그램 논리를 단순화하고 자주 사용할 것으로 예상되는 값을 미리 계산.
- 선택한 평가 전략이 주어진 문제에 적합한지 확인하기 위해 메모리 사용량과 성능을 모니터링.