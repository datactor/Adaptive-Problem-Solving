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
(input 값이 아닌 input의 참조에 덮어씌운다. 아직까지 input의 '값'은 그대로 empty string인 상태)
3. read_to_string 메서드는 &mut input이 가리키는 메모리 위치에 기록하여 원본 값을 업데이트한다.
참조자는 당연히 메모리 위치를 기록하지만, 원본값을 업데이트 하는 것은 역참조와 mutable이 필요하다
(역참조가 없다면 참조자만 변경하고 원본 값은 변경되지 않음).
역참조 작업은 read_to_string 함수 내에서 명시적으로 수행되지 않는다.
대신 가변 참조인 &mut 입력이 read_to_string에 인수로 전달되는 순간 자동으로 역참조된다(read_to_string같은 특정 함수만 자동 역참조 기능이 있다).
그런 다음 메서드는 참조가 가리키는 메모리 위치(input value)의 끝부터 기록하여 역참조된 원본 input 값을 업데이트한다.


4. 만약 read_to_string의 인자를 &mut input이 아니라 mut input을 사용했다면,
input이 read_to_string으로 이동되면서 소유권도 이동되고
스코프가 끝나는 시점에 소비되면서 input이 삭제된다.
