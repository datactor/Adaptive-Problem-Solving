# Rc<RefCell< T >>

Rc(Reference Counting)는 런타임 시 값에 대한 참조 수를 추적하는 type으로,
값의 다중 소유권을 허용한다. Rc 값이 clone되면 참조 횟수가 증가하고 삭제되면
참조 횟수가 감소한다. 참조 횟수가 0이 되면 값이 삭제된다.

RefCell은 내부 변경 가능성을 제공하는 유형으로, 변경할 수 없는 참조가 있는 경우에도
값의 변경을 허용한다. RefCell은 현재 차용 여부를 추적하고 borrow rule 위반이 있는 경우
런타임에 패닉 상태가 된다.

## borrow checker vs dyn(runtime) borrow checker
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

