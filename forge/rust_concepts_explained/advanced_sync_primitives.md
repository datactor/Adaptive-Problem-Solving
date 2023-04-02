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

## 2. Overview of Rust's standard shareable mutable containers.
Rust의 표준 라이브러리는 안전하고 효율적인 동시 프로그래밍을 가능하게 하는 여러 공유 shareable mutable containers들을 제공한다.
이 섹션에서는 이러한 컨테이너를 자세히 살펴보고 해당 기능, 장단점 및 최상의 사용 사례를 살펴보자.

### Cell: definition, how to use, and trade-offs
Cell<T>는 값 T의 내부 가변성을 제공하는 간단한 컨테이너이다.
즉, immutable reference의 일부인 경우에도 값을 변경할 수 있다.
Cell<T>는 스레드 간에 값을 공유하면서 값을 변경해야 할 때 사용된다.

다음은 rust의 Cell에 대한 구현이다. 더 자세한 정보는 아래의 링크를 참고한다.  
https://doc.rust-lang.org/core/cell/struct.Cell.html
```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[repr(transparent)]
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
  /// Creates a new `Cell` containing the given value.
  ///
  /// # Examples
  /// let c = Cell::new(5);
  #[stable(feature = "rust1", since = "1.0.0")]
  #[rustc_const_stable(feature = "const_cell_new", since = "1.24.0")]
  #[inline]
  pub const fn new(value: T) -> Cell<T> {
    Cell { value: UnsafeCell::new(value) }
  }

  /// Sets the contained value.
  /// # Examples
  /// let c = Cell::new(5);
  /// c.set(10);
  #[inline]
  #[stable(feature = "rust1", since = "1.0.0")]
  pub fn set(&self, val: T) {
    let old = self.replace(val);
    drop(old);
  }

  #[inline]
  #[stable(feature = "move_cell", since = "1.17.0")]
  pub fn swap(&self, other: &Self) {
    if ptr::eq(self, other) {
      return;
    }
    // SAFETY: This can be risky if called from separate threads, but `Cell`
    // is `!Sync` so this won't happen. This also won't invalidate any
    // pointers since `Cell` makes sure nothing else will be pointing into
    // either of these `Cell`s.
    unsafe {
      ptr::swap(self.value.get(), other.value.get());
    }
  }

  #[inline]
  #[stable(feature = "move_cell", since = "1.17.0")]
  pub fn replace(&self, val: T) -> T {
    // SAFETY: This can cause data races if called from a separate thread,
    // but `Cell` is `!Sync` so this won't happen.
    mem::replace(unsafe { &mut *self.value.get() }, val)
  }

  #[stable(feature = "move_cell", since = "1.17.0")]
  #[rustc_const_unstable(feature = "const_cell_into_inner", issue = "78729")]
  pub const fn into_inner(self) -> T {
    self.value.into_inner()
  }
}

impl<T: ?Sized> Cell<T> {
  /// Returns a raw pointer to the underlying data in this cell.
  ///
  /// # Examples
  /// let c = Cell::new(5);
  /// let ptr = c.as_ptr();
  #[inline]
  #[stable(feature = "cell_as_ptr", since = "1.12.0")]
  #[rustc_const_stable(feature = "const_cell_as_ptr", since = "1.32.0")]
  pub const fn as_ptr(&self) -> *mut T {
    self.value.get()
  }

  /// Returns a mutable reference to the underlying data.
  ///
  /// This call borrows `Cell` mutably (at compile-time) which guarantees
  /// that we possess the only reference.
  ///
  /// However be cautious: this method expects `self` to be mutable, which is
  /// generally not the case when using a `Cell`. If you require interior
  /// mutability by reference, consider using `RefCell` which provides
  /// run-time checked mutable borrows through its [`borrow_mut`] method.
  ///
  /// [`borrow_mut`]: RefCell::borrow_mut()
  ///
  /// # Examples
  /// let mut c = Cell::new(5);
  /// *c.get_mut() += 1;
  ///
  /// assert_eq!(c.get(), 6);
  #[inline]
  #[stable(feature = "cell_get_mut", since = "1.11.0")]
  pub fn get_mut(&mut self) -> &mut T {
    self.value.get_mut()
  }

  /// Returns a `&Cell<T>` from a `&mut T`
  ///
  /// # Examples
  /// let slice: &mut [i32] = &mut [1, 2, 3];
  /// let cell_slice: &Cell<[i32]> = Cell::from_mut(slice);
  /// let slice_cell: &[Cell<i32>] = cell_slice.as_slice_of_cells();
  ///
  /// assert_eq!(slice_cell.len(), 3);
  #[inline]
  #[stable(feature = "as_cell", since = "1.37.0")]
  pub fn from_mut(t: &mut T) -> &Cell<T> {
    // SAFETY: `&mut` ensures unique access.
    unsafe { &*(t as *mut T as *const Cell<T>) }
  }
}
```

Cell<T>에는 다음과 같은 특징이 있다.

1. value type이라 복사가 저렴하다.
  - Cell<T>의 경우 value type이라는 것은 값이 다른 곳에 저장되고 참조로 가리키는 것이 아니라 Cell<T> 인스턴스 자체에 직접 저장됨을 의미한다.
    즉, Cell<T> 인스턴스가 복사되거나 함수에 전달될 때 해당 내용의 복사본이 만들어지며 이는 시간과 메모리 측면에서 저렴하다.
  - Cell<T>는 sized type이므로 컴파일 시간에 크기를 알 수 있으므로 copy 또는 clone이 저렴하고 효율적이다.
    이는 Cell<T>가 copy되거나 clone될 때 참조 형식의 경우처럼 데이터에 대한 참조를 복사하는 대신 내용의 비트 복사본이 만들어지기 때문이다.
    결과적으로 값비싼 메모리 할당 및 할당 해제 비용을 발생시키지 않고 스레드 간에 mutable 값을 공유하고 복사해야 하는 상황에서 Cell<T>를 사용할 수 있다.
2. 내용을 가져오고 설정하는 단일 메서드인 get, get_mut 및 set이 있다.
  - `Cell<T>`은 콘텐츠를 가져오고 설정하는 단일 메서드(get, get_mut 및 set)를 제공한다.
    get 메서드는 Cell<T> 내부의 값에 대한 immutable reference를 반환하는 반면 get_mut은 Cell<T> 내부의 값에 대한 mutable reference를 반환한다.
    이 두 가지 방법 모두 컴파일 타임에 Rust의 borrow rule을 적용한다.  
    구체적으로 설명하면, `Cell<T>` type에 대해서는 borrow checker가 동일한 값에 대한 두 개의 mutable reference가 동시에 보유되지 않도록
    compile time에 borrow rule을 시행한다. 우리는 UnsafeCell은 내부 가변성이 제공하고 borrow checker의 검사를 우회하는 것을 알고 있다.
    그럼에도 어떻게 borrow checker를 이용할까?  
    borrow checker는 UnsafeCell을 사용하지 않는 type에 대해 compile time에 borrow rule을 시행하지만,
    UnsafeCell을 사용하는 type에 대해서는 이를 시행할 수 없다. `Cell`의 경우 value type이기 때문에, 값에 대해 mut을 명시해야 하고,
    mut이 명시된 값을 Cell type으로 wrapping하여 borrow checker는 mutable reference가 한 번에 하나만
    존재할 수 있음을 보장하게 함과 동시에 내부 가변성을 제공한다.  
    이 보장은 value type이 아닌 RefCell에 대해서는 통하지 않는다.
    RefCell은 value type이 아니며, 값에 대해 외부에 mut keyword를 명시할 필요가 없기 때문에
    borrow checker의 검사를 우회하며 안전성의 보장은 프로그래머의 책임으로 귀속 된다.
  - 정리하면 Cell type은 mut keyword를 명시한 sized type을 Cell type으로 wrapping하는 과정을 포함하며, 여기의 mut keyword에 의해
    borrow checker가 하나의 가변참조를 보장하도록 검사할 수 있게 한다.  
    반면에 RefCell은 값에 대해 mut keyword를 명시할 필요가 없으며, value가 아닌 참조 카피하는 타입이므로 내부 가변성을 보장하지만 
    하나의 값에 대해 여러 개의 가변 참조가 있는 것에 대해서는 안전성을 보장해주지 못한다. runtime error가 나오겠지만, compile이 된다. 
  - Cell은 내부 가변성을 제공하고, value type이기 때문에 borrow나 borrow_mut 메서드가 필요 없다.
    get_mut으로 한번에 한가지 가변 참조만 불러 올 수 있고, 이것은 외부에는 표시되지 않는 내부 가변성이다.  
    또한 여기서 내부 가변성으로 불러온 변수의 외부에 &mut 키워드를 명시하면 한번에 두 가지 가변참조를 가질 수 있지만 이러한 접근은 위험하다.
  - set 메서드를 사용하면 Cell<T>에 저장된 값을 동일한 type의 새 값으로 바꿀 수 있다.
3. Copy 및 Clone을 구현하여 struct 및 기타 types에서 쉽게 사용할 수 있다.
  - Cell<T>는 Copy 및 Clone trait을 구현한다. 즉, 해당 trait이 필요한 struct 및 기타 types에서 쉽게 사용할 수 있다.
  - Cell<T>는 value type이므로 복사하거나 복제하는 것이 저렴하며 이러한 작업은 Cell<T> 내부에 저장된 값을 복사하기만 하면 된다.
    이것은 다중 스레드 코드로 작업할 때 유용할 수 있다. 값비싼 메모리 할당 및 할당 해제 비용을 발생시키지 않고 Cell<T> 및
    해당 콘텐츠의 복사본을 쉽게 만들어 스레드 간에 공유할 수 있기 때문이다.
