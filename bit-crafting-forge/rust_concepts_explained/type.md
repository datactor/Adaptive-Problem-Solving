# Rust type
러스트의 타입은

'stack에 올려도 안전한 타입'과

'stack에 올리면 안전하지 않은 타입'으로 나뉜다.

예를 들어 dynamic-sized(unsized) type, non-trivial ownership type, borrowing semantics type
같은 타입들은 stack에 올린다면 안전하지 않기 때문에 힙에 할당한다.
(stack에 배치하려면 소유권 시스템에서 관리할 수 있게 만들어서 pointer를 스택에 배치한다)


# Copy vs Clone?

- Copy trait is used for types that can be copied (bitwise copy) safely,
  without considering ownership or resources.
- Clone trait is used for types that need to be deep copied (clone the whole object).
- The Copy trait is automatically implemented by the compiler for types that
  are stored on the stack and satisfy certain requirements,
  such as scalar types, tuples, and arrays.
- The Clone trait is not automatically implemented, and you need to define your
  own implementation if your type needs to be deep copied.
- The .to_owned() method creates a new heap-allocated object and copies
  the original data into it, using the Clone trait to perform the copy.
- If a type implements both Copy and Clone traits, then the copy() method performs
  a shallow copy using the Copy trait, while the clone() method performs a deep copy
  using the Clone trait.


copy와 clone이 나뉜 이유는 copy를 통해서 값의 복사를 수행할 수 있기 때문에 copy를 사용하는 것이고
copy를 통해서 복사할 수 없는 데이터 타입들은 Clone을 수행 한다는 것이다.

값을 복사한다는 본질은 copy와 clone 모두 같다. 즉 여기서 말하는 copy는 swallow copy가 아닌 true copy이다.
(rust에서 swallow copy는 값을 복사하는 것이 아닌 참조를 복사하는 경우를 말한다.)
이러한 특성 때문에 'stack에 올리면 안전하지 않은 타입'은 소유권 시스템에서 관리할 수 있는 크기를 갖기 위해
힙에 할당 시킨다. 그렇기 때문에 힙에 할당된 타입은 소유권의 제어에 민감하고,
스택에 저장되는 타입은 소유권 제어에 관하여 상대적으로 자유롭다.

예를 들어 Rust의 idiosm에서 .copy() 메소드를 사용하지 않는 이유는
copy는 그저 다른 곳에 할당하거나 인자로 전달하는 것만으로 값을 복사할 수 있기 때문이다.
반면에 Clone()은 명시적으로 구현되어야 한다.

# Copy vs swallow copy(in Rust)
Copy trait과 swallow-copy(new reference copy in Rust idiom)는 둘다 스택의 값을 bitwise copy한다는 점에서
동일하다. 그렇지만 중요한 차이점은 복사되어 생성된 값의 소유권이다.
Copy trait을 사용하면 원래 값의 소유권은 그대로 유지되고, 새로운 소유권을 가진 새 복사 값이 생성된다.
반면에 swallow-copy(new reference copy)를 사용하면 원본 값을 참조하는 새로운 참조 pointer가 생성된다.
원본 값의 소유권은 변경되지 않으며 원래 값에 대한 새로운 소유권을 생성하지 않고 빌린 참조문과 동일한 소유권을 가진다.

원본 값의 레퍼런스를 만들면(swallow-copy하면) 소유권은 늘어나지 않는다.
이것을 몇번을 swallow-copy하든 새로운 소유권을 생성하지 않고, 모든 레퍼런스들은 원본의 소유권을 동일하게 '차용'한다.
mutable reference pointer와 immutable reference pointer 모두 차용하는 소유권은 하나이다.

# to_owned vs clone vs copy
to_owned(ToOwned trait)는 기존에 빌린 값에서 새로운 소유 값을 생성한다.
to_owned 메소드는 언제나 Clone을 수행하며, 값을 복사하여 의존성없이 만든다.

rust에서 스칼라 타입의 값은 clone() 메소드를 수행하면, 실제로는 copy 메소드를 수행한다(스택에 새 값을 복사).
그러므로 스칼라 타입의 값에 to_owned를 수행하면 복사하여 힙이 아닌 스택에 새 소유권의 값을 생성한다.