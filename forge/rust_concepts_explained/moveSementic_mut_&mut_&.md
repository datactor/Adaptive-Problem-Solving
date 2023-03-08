## move sementic
값이 함수에 인수로 할당되거나 전달될 때 값을 복사하는 대신 이동할 수 있도록 허용하는 Rust의 개념.
1. 값을 이동하면 소유권이 한 변수에서 다른 변수로 이전된다.
2. 원래 소유권을 갖고 있던 변수는 유효하지 않거나 사용할 수 없게 된다.

copy, clone 등으로 새롭게 생성하지 않고 기존의 리소스를 사용하기 때문에
성능을 개선하고 불필요한 메모리 사용을 방지할 수 있다.

함수에서 값을 반환하는 것은 해당 값의 소유권을 옮기는 것과 관련이 있다.
함수에서 반환된 값은 함수의 스택 프레임에서 caller의 스택 프레임으로 이동한다.
즉 새로운 소유권을 생성하지 않고 소유권의 움직임은 다음과 같다.

`variable (initial ownership) -> function's stack frame as argument -> return value -> caller`

이것은 러스트 내부의 기능이 아닌 borrow checker가 수행하는 독립적인 기능이다.

즉, 여기서 할당 해제는 Drop trait이 수행하는 메소드가 아니고 자체적인 기능이며,
이외의 소유권, 차용 시스템 역시 rust의 라이브러리에 있는 기능이 아니다.

move sementic, 소유권, 차용 시스템은 borrow checker에서 수행하는 핵심 기능으로
덕분에 가비지 컬렉터 없이도 메모리 안전을 보장받을 수 있다.

move sementic의 이동시 원래 변수의 무효화 기능은 Use-After-Free를 방지하며,
borrow system은 참조와 불변 참조의 엄격한 borrow rule을 통해 스레드 단위에서도 data races가 방지된다.

이것들을 결합하여 GC의 오버헤드 없이 메모리 안전성과 스레드 안전성을 달성할 수 있다.

한편, move sementice은 성능과 메모리 사용을 개선할 수 있지만 모든 상황에서 적절하거나 바람직하지는 않을 수 있다.
예를 들어 큰 데이터 구조를 프로그램의 여러 부분에서 공유해야 하는 경우 복사하는 것이
이동하는 것보다 효율적일 수 있으며 이러한 경우 공유 소유권을 지원하는 타입인 Rc 또는 Arc를 사용할 수 있다.


## &mut
&mut 변수를 생성하는 것은 원본 변수에 독점적 권한을 조건으로, 소유권을 빌려간다.
&mut pointer가 존재할 때, 원본 접근방법은 원본에 직접 접근하는 것과 &mut pointer를 통해
접근하는 2가지 방법이 뿐이며, 새로운 참조 포인터 생성을 통해 접근할 수는 없다.
이는 뮤터블 레퍼런스가 원본에 독점적으로 엑세스할 수 있고 borrow checker가 data races를 비롯한
unsafe 동작을 방지하기 위해 컴파일 타임에 이 독점성을 시행하기 때문이다.

&mut pointer가 존재할 때, &mut pointer를 통하지 않고 원래 변수에 엑세스하면 reference가 소비되어
더 이상 이 reference는 유효하지 않다. 이는 &mut pointer의 계약은 '독점적' 소유권 빌림이었기 때문이다.
여기서 '독점적'이라는 계약 조건이 깨져버린다.

move sementic과 마찬가지로, &mut 계약, 소유권, 빌림은 rust 내부에서 구현되는 것이 아닌,
컴파일타임에 borrow checker에 의해 검사된다.

## mut vs &mut
mutable refernce는 mutable 변수에 대해 원본 값을 변경할 수 있는 레퍼런스이다.
reference가 원본의 데이터를 수정할 수 있는 대신 몇가지 제약사항이 있다.
그 중 널리 알려진 것은 borrow rule에 포함되어 있다.
- borrow rules
  1. Each value in Rust has a variable that's called its owner.
  2. There can only be one owner for each value at any given time.
  3. When the owner goes out of scope, the value will be dropped.
  4. References to a value can be created using the & symbol.
     References do not have ownership and do not affect the lifetime of the value.
  5. ~~Immutable references (&T) can be created to a value as many times as you want,
     but you cannot create a mutable reference (&mut T)
     if there's an existing immutable reference to that value.~~  
     You can create as many immutable references as you want, but if you create a mutable reference
     when there is an existing immutable reference to that value,
     the existing immutable reference becomes invalid and cannot be used.
     From the time of creating a variable reference, only the variable reference becomes valid.
  6. You can have either one mutable reference or any number of immutable references to a value,
     but not both at the same time.
  7. References must always be valid; that is,
     they must point to a value that still exists and has not yet been dropped.
     Rust's borrow checker enforces this rule at compile-time.
  8. Ownership can be transferred using the move keyword.
  
5번째 rule은 현재 기준으로 수정한다.  
immutable references들은 얼마든지 생성할 수 있지만, immutable reference가 유효한 상태에서 mutable reference를 생성하면,
기존의 immutable references들을 모두 무효화 하면서 새로 생성한 mutable reference만 유효하게 만든다.
그러므로 새로운 mutable reference를 생성할 수 있다.

borrow rule에 명시되지 않은 mut과 &mut의 주요 차이점 중 하나는 다음과 같다.  
&mut이 원본 값에 대한 독점적인 계약인 반면, mut은 단순히 변수를 변경 가능하게 표시한다는 것이다. 
여기서 독점적이라는 것은, &mut reference를 통해 접근하여 수정하는 것 이외의 모든 행위(원본에 직접 엑세스 해서 수정하는 경우 포함하여)는
계약 위반이라는 뜻이다. 위반하면 &mut reference는 무효화 된다.

이러한 제약을 가지고 있는 &mut에 비하면 일반 mut은 제약이 없는 편이지만,
한가지 주의 해야 할 사항이 있다.  
5번째 borrow rule과 비슷하게 mut variable을 immutable reference로 차용 중 일때,
immutable reference를 소비하기 전에 변수를 수정하면 immutable reference는 무효화 되면서
변수가 수정 된다.

&mut 사용 예:
```Rust
use std::{
    io::{self, prelude::*},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new(); // 1
    io::stdin().read_to_string(&mut input)?; // 2~3
    
    Ok(())
}
```

1. String type의 input 객체를 mutable로 생성한다.
2. stdin().read_to_string으로 받은 IO를 &mut input에 덮어 씌운다.
(input 값이 아닌 input의 참조 포인터 객체에 덮어씌운다. 아직까지 input의 '값'은 그대로 empty string인 상태)
3. read_to_string 메서드는 &mut input이 가리키는 메모리 위치에 기록하여 원본 값을 업데이트한다.
참조자는 당연히 메모리 위치를 기록하지만, 원본값을 업데이트 하는 것은 역참조와 mutable이 필요하다
(역참조가 없다면 참조자만 변경하고 원본 값은 변경되지 않음).
역참조 작업은 read_to_string 함수 내에서 명시적으로 수행되지 않는다.
대신 가변 참조인 &mut 입력이 read_to_string에 인수로 전달되는 순간 자동으로 역참조된다(read_to_string같은 특정 함수만 자동 역참조 기능이 있다).
그런 다음 메서드는 참조가 가리키는 메모리 위치(input value)의 끝부터 기록하여 역참조된 원본 input 값을 업데이트한다.

4. 만약 read_to_string의 인자를 &mut input이 아니라 mut input을 사용했다면,
input이 read_to_string으로 이동되면서 소유권도 이동되고
스코프가 끝나는 시점에 소비되면서 input이 삭제된다.