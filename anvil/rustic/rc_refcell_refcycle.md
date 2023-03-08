# Rc<RefCell< T >>

## Rc

Rc(Reference Counting)는 런타임 시 값에 대한 참조 수를 추적하는 type으로,
값의 다중 소유권을 허용한다.
Rc::clone을 수행할 때마다 clone된 포인터를 ptr필드에 기입하고 strong count를 증가시킨다.
반대로 Rc::clone된 스마트 포인터가 drop될 때마다 ptr필드에서 스마트 포인터는 제거되고
strong count를 감소시킨다. 강한 참조가 0이 된다면, Rc는 약한 참조를 확인한다.
약한 참조 횟수도 0일 경우 Rc는 dealloc을 수행해 모든 할당을 해제한다.
그러나 약한 참조 횟수가 0이 아닌 경우 dealloc되지 않고 활성상태로 유지된다는 점을 기억해야 한다.
예를 들어 강한 참조 횟수가 1이고 약한 참조 횟수가 0이 아닌 상황에서 Rc를 drop한다면,
인스턴스는 사라지지만 메모리는 할당해제되지 않기 때문에 주의해야 한다.
이 공유 데이터는 메모리에 남아 있지만 접근할 수단이 없기 때문에, 사실상 손실되어 사용할 수 없으며
메모리 낭비가 발생한다. 그렇기 때문에 weak참조를 사용할 경우 적절히 관리하여 이러한 상황을 방지해야 한다.

### Reference Pointer vs smart pointer Rc::clone
원본 값과, 참조 포인터가 있다고 했을때, 소유권은
원본 값의 소유권으로 1개이다(여러 개의 immutable 참조 포인터가 있다고 하더라도
서로 동일한 '원본'의 소유권을 '차용'한다).

Rc의 경우에도, Rc 원본 값과 Rc::clone의 소유권은 원본 하나이지만,
원본값과 일반 참조 포인터와의 관계와는 대조적으로,
'공유 데이터를 포함한 원본'을 Rc::clone된 스마트 포인터들이 모두 공유(소유)한다.
스마트 포인터인 Rc::clone이 죽으면 Rc의 소유권은 공유되어 있기 때문에 counter가 감소한다.
(move sementic(할당 등의 이유로 Rc type의 기능과는 별개로 borrow checker가 clone을 삭제시키면,
Rc원본에서 drop을 호출하여 count 감소) 등으로 drop이 호출되면 Rc원본의 count가 줄어든다)

즉, 둘의 차이를 요약하면
- 일반 포인터는 원본의 소유권을 '차용'한다. immutable 포인터는 읽기 전용 액세스 권한을 갖지만,
&mut을 통해 생성된 포인터라도 일시적인 독점적 엑세스 권한을 가질 수 있어 원본을 수정할 수 있지만
소유권은 소유하지 않는다.
- Rc 스마트 포인터는 'self를 수정하면 공유데이터이기 때문에 원본에도 반영된다'이다.

Rc가 생성되면 데이터 포인터 필드와 포인터필드로 구성되는데
(ptr 필드의 기본은 스트롱카운트 1, weak 카운트 1, self pointer 주소, data 필드에는 공유 데이터의 포인터가 들어 있다.)
Rc::clone을 하는 순간 원본 Rc에 strong counter를 증가시키면서 새로 생성한 포인터를 재귀형태의 Box 타입으로 push한다.
그리고 Rc::clone으로 생성된 새로운 바이트 주소는 소유권을 공유하고 원본 Rc의 포인터를 가리키고 있기 때문에,
drop되는 순간 원본 Rc의 drop method를 call하여 원본의 ptr 필드에서 dealloc 시키고, strong count를 줄인다.

```rust
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            let ptr = self.ptr.as_mut();
            let strong = ptr.as_mut().strong.get();
            let weak = ptr.as_mut().weak.get();

            if strong == 1 && weak == 0 {
                // This is the last strong reference, deallocate the allocation
                dealloc(ptr.as_ptr(), Layout::new::<RcBox<T>>());
            } else {
                // Decrement the strong reference count
                ptr.as_mut().strong.set(strong - 1);
            }
        }
    }
}
```
즉, 일반 reference pointer를 drop한다고해서 원본에는 아무 일도 일어나지 않지만(서로의 소유권을 공유하고 있지 않기 때문에),

Rc 스마트 포인터는 Rc 원본의 모든 부분을 '차용'이 아닌 '소유'하고 있기 때문에
Rc::clone의 self를 drop하면 원본 Rc에도 반영이 되는 것이다.
Rc::clone은 '원본 데이터를 그대로 소유하는 복제된 객체'를 생성하기 때문에
포인터임에도 불구하고 clone이라고 네이밍 되었다는 생각이 든다.


## RefCell
내부 가변성을 제공하는 유형으로, 변경할 수 없는 참조가 있는 경우에도
값의 변경을 허용한다. RefCell은 현재 차용 여부를 추적하고 borrow rule 위반이 있는 경우
런타임에 패닉 상태가 된다.

### borrow checker vs dyn(runtime) borrow checker
Rc<RefCell<T>>를 컴파일할 때, borrow checker의 검사와 borrow rule은 아무것도 변하는게 없다.
borrow checker가 봤을땐 RefCell은 immutable 변수로 보이기 때문에 검사를 해도
무결성으로 인식하는 것이다. 즉, borrow checker는 컴파일 시간에 한정하여 정적 코드에 기타 버그가
없는지 확인하는 정적 코드 분석 툴이다. 그렇기 때문에 RefCell을 사용했을 때는, 공유 데이터에 대한
가변 엑세스로 인해 발생할 수 있는 잠재적인 data races를 비롯한 문제들을 감지하지 못한다.

이후에 러스트는 평소와 같이 runtime 코드 분석 툴인 dyn borrow checker가 검사를 수행한다.
dyn borrow checker는 이러한 규칙이 런타임에도 올바르게 준수되는지 확인한다.

여기서 핵심은 borrow_mut() 메소드이다. borrow_mut()의 메소드를 수행했을 때,
dyn borrow checker가 동적 코드를 분석해 borrow rule을 따르는지 확인한다.

평소에는 이 두가지가 모두 수행되지만, RefCell을 수행했을 때는 dyn borrow checker의 검사만
수행한다고 보면 된다. 즉 Rc<RefCell<T>>는 컴파일러에 예외 사항을 주는 타입이 아니라,
rust보다 똑똑한 프로그래머의 기발한 발상인 것이다.

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    {
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        println!("b after = {:?}", b);
    }
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("c after = {:?}", c);
    println!("{}", Rc::strong_count(&a))
}
```

Rc와 RefCell을 결합하면 값의 다중 소유권 및 interior mutability를 허용한다.
그러나 Rc는 atomic하지 않아 참조 횟수 업데이트의 완전 성공과 완전 실패를 보장하지 않으며,
RefCell은 락을 걸지 않기 때문에, 다중 스레드 환경에서 Rc<RefCell<T>를 사용하는 것은
동시 프로그래밍에 적합하지 않다.

## Reference cycles

Reference cycles can lead to memory leak
순환 참조는 두 개 이상의 값이 서로를 참조하여 순환을 형성할 때 발생한다.
값이 올바르게 삭제되지 않으면 이로 인해 메모리 누수가 발생할 수 있다.

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Person {
    name: String,
    best_friend: Option<Rc<RefCell<Person>>>,
}

fn main() {
    let alice = Rc::new(RefCell::new(Person {
        name: String::from("Alice"),
        best_friend: None,
    }));

    let bob = Rc::new(RefCell::new(Person {
        name: String::from("Bob"),
        best_friend: None,
    }));

    alice.borrow_mut().best_friend = Some(Rc::clone(&bob));
    bob.borrow_mut().best_friend = Some(Rc::clone(&alice));
}
```
위의 코드와 같이, 서로를 순환 참조하는 Rc 스마트 포인터를 계속해서 생성한다면
strong count가 계속해서 증가하게 된다. alice 또는 bob의 참조 횟수가 0으로 떨어지지 않으면
Person 인스턴스(공유 데이터)가 메모리에 무기한 남아있기 때문에
메모리 누수로 이어질 수 있다.

이를 방지하기 위해 Rust는 순환 참조를 끊는 데 사용할 수 있는 weak pointer type을 제공한다.
weak count는 strong count와 달리 0부터 시작하며, 명시적으로 downgrade하지 않으면 count되지 않는다.
즉 명시적으로 downgrade 또는 upgrade 하지 않으면 Rc 포인터가 drop되거나 새로운 Rc::clone()이 생성되더라도
weak count의 수는 변경되지 않는다.
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Person {
    name: String,
    best_friend: Option<Weak<RefCell<Person>>>,
}

fn main() {
    let alice = Rc::new(RefCell::new(Person {
        name: String::from("Alice"),
        best_friend: None,
    }));

    let bob = Rc::new(RefCell::new(Person {
        name: String::from("Bob"),
        best_friend: None,
    }));

    alice.borrow_mut().best_friend = Some(Rc::downgrade(&bob));
    bob.borrow_mut().best_friend = Some(Rc::downgrade(&alice));

    // We can still access alice and bob here, but when their
    // reference counts drop to zero, they will be deallocated
}
```


Rc 및 RefCell을 사용할 때 런타임 패닉을 피하기 위해 차용 규칙을 인식하는 것과
참조 카운팅 및 내부 가변성을 사용하는 성능 영향을 고려하는 것이 중요하다.
Rc와 RefCell은 특정한 경우에 유용할 수 있지만 신중하게 필요한 경우에만 사용해야 한다.
또한 Rc를 사용할 때 순환 참조를 생성하지 않도록 주의해야 한다.