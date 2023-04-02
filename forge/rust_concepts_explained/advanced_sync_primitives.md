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
    이는 Cell<T>이 copy되거나 clone될 때 참조 형식의 경우처럼 데이터에 대한 참조를 복사하는 대신 내용의 비트 복사본이 만들어지기 때문이다.
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

### RefCell: definition, how to use, and trade-offs
`RefCell<T>`은 값 T의 내부 가변성을 제공하는 또 다른 컨테이너이다.
`Cell<T>`과 달리 `RefCell<T>`은 참조 type이며 모두 불변 참조이거나 단일 가변 참조인 한 값에 대한 여러 참조를 허용한다.
`RefCell<T>`은 스레드 간에 값을 공유하면서 값을 변경해야 하지만 참조에 대한 더 많은 제어가 필요할 때 사용된다.

다음은 rust의 RefCell에 대한 구현이다. 더 자세한 정보는 아래의 링크를 참고한다.  
https://doc.rust-lang.org/core/cell/struct.RefCell.html

```rust
#[cfg_attr(not(test), rustc_diagnostic_item = "RefCell")]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct RefCell<T: ?Sized> {
    borrow: Cell<BorrowFlag>,
    // Stores the location of the earliest currently active borrow.
    // This gets updated whenever we go from having zero borrows
    // to having a single borrow. When a borrow occurs, this gets included
    // in the generated `BorrowError/`BorrowMutError`
    #[cfg(feature = "debug_refcell")]
    // borrowed_at 필드는 코드에서 가장 먼저 활성화된 borrow가 발생한 위치를 추적하는 데 사용되며, 새 borrow가 생성될 때마다 업데이트 된다.
    // 주로 debugging 목적으로 사용되어 panic!이 발생한 경우 borrow rule 위반이 발생한 위치를 식별하는데 쓰인다.
    borrowed_at: Cell<Option<&'static crate::panic::Location<'static>>>,
    value: UnsafeCell<T>,
}

impl<T> RefCell<T> {
  /// Creates a new `RefCell` containing `value`.
  ///
  /// # Examples
  /// let c = RefCell::new(5);
  #[stable(feature = "rust1", since = "1.0.0")]
  #[rustc_const_stable(feature = "const_refcell_new", since = "1.24.0")]
  #[inline]
  pub const fn new(value: T) -> RefCell<T> {
    RefCell {
      value: UnsafeCell::new(value),
      borrow: Cell::new(UNUSED),
      #[cfg(feature = "debug_refcell")]
      borrowed_at: Cell::new(None),
    }
  }

  /// Consumes the `RefCell`, returning the wrapped value.
  ///
  /// # Examples
  /// let c = RefCell::new(5);
  /// let five = c.into_inner();
  #[stable(feature = "rust1", since = "1.0.0")]
  #[rustc_const_unstable(feature = "const_cell_into_inner", issue = "78729")]
  #[inline]
  pub const fn into_inner(self) -> T {
    // Since this function takes `self` (the `RefCell`) by value, the
    // compiler statically verifies that it is not currently borrowed.
    self.value.into_inner()
  }

  /// Replaces the wrapped value with a new one, returning the old value,
  /// without deinitializing either one.
  ///
  /// This function corresponds to [`std::mem::replace`](../mem/fn.replace.html).
  ///
  /// # Panics
  ///
  /// Panics if the value is currently borrowed.
  ///
  /// # Examples
  /// let cell = RefCell::new(5);
  /// let old_value = cell.replace(6);
  /// assert_eq!(old_value, 5);
  /// assert_eq!(cell, RefCell::new(6));
  #[inline]
  #[stable(feature = "refcell_replace", since = "1.24.0")]
  #[track_caller]
  pub fn replace(&self, t: T) -> T {
    mem::replace(&mut *self.borrow_mut(), t)
  }

  /// Replaces the wrapped value with a new one computed from `f`, returning
  /// the old value, without deinitializing either one.
  ///
  /// # Panics
  ///
  /// Panics if the value is currently borrowed.
  ///
  /// # Examples
  /// let cell = RefCell::new(5);
  /// let old_value = cell.replace_with(|&mut old| old + 1);
  /// assert_eq!(old_value, 5);
  /// assert_eq!(cell, RefCell::new(6));
  #[inline]
  #[stable(feature = "refcell_replace_swap", since = "1.35.0")]
  #[track_caller]
  pub fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
    let mut_borrow = &mut *self.borrow_mut();
    let replacement = f(mut_borrow);
    mem::replace(mut_borrow, replacement)
  }

  /// Swaps the wrapped value of `self` with the wrapped value of `other`,
  /// without deinitializing either one.
  ///
  /// This function corresponds to [`std::mem::swap`](../mem/fn.swap.html).
  ///
  /// # Panics
  ///
  /// Panics if the value in either `RefCell` is currently borrowed.
  ///
  /// # Examples
  /// let c = RefCell::new(5);
  /// let d = RefCell::new(6);
  /// c.swap(&d);
  /// assert_eq!(c, RefCell::new(6));
  /// assert_eq!(d, RefCell::new(5));
  #[inline]
  #[stable(feature = "refcell_swap", since = "1.24.0")]
  pub fn swap(&self, other: &Self) {
    mem::swap(&mut *self.borrow_mut(), &mut *other.borrow_mut())
  }
}

impl<T: ?Sized> RefCell<T> {
  /// Immutably borrows the wrapped value.
  ///
  /// The borrow lasts until the returned `Ref` exits scope. Multiple
  /// immutable borrows can be taken out at the same time.
  ///
  /// # Panics
  ///
  /// Panics if the value is currently mutably borrowed. For a non-panicking variant, use
  /// [`try_borrow`](#method.try_borrow).
  ///
  /// # Examples
  /// let c = RefCell::new(5);
  /// let borrowed_five = c.borrow();
  /// let borrowed_five2 = c.borrow();
  ///
  /// An example of panic:
  ///
  /// ```should_panic
  /// let c = RefCell::new(5);
  /// let m = c.borrow_mut();
  /// let b = c.borrow(); // this causes a panic
  /// ```
  #[stable(feature = "rust1", since = "1.0.0")]
  #[inline]
  #[track_caller]
  pub fn borrow(&self) -> Ref<'_, T> {
    self.try_borrow().expect("already mutably borrowed")
  }
  /// Immutably borrows the wrapped value, returning an error if the value is currently mutably
  /// borrowed.
  ///
  /// The borrow lasts until the returned `Ref` exits scope. Multiple immutable borrows can be
  /// taken out at the same time.
  ///
  /// This is the non-panicking variant of [`borrow`](#method.borrow).
  ///
  /// # Examples
  ///
  /// ```
  /// use std::cell::RefCell;
  ///
  /// let c = RefCell::new(5);
  ///
  /// {
  ///     let m = c.borrow_mut();
  ///     assert!(c.try_borrow().is_err());
  /// }
  ///
  /// {
  ///     let m = c.borrow();
  ///     assert!(c.try_borrow().is_ok());
  /// }
  /// ```
  #[stable(feature = "try_borrow", since = "1.13.0")]
  #[inline]
  #[cfg_attr(feature = "debug_refcell", track_caller)]
  pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
    match BorrowRef::new(&self.borrow) {
      Some(b) => {
        #[cfg(feature = "debug_refcell")]
        {
          // `borrowed_at` is always the *first* active borrow
          if b.borrow.get() == 1 {
            self.borrowed_at.set(Some(crate::panic::Location::caller()));
          }
        }

        // SAFETY: `BorrowRef` ensures that there is only immutable access
        // to the value while borrowed.
        let value = unsafe { NonNull::new_unchecked(self.value.get()) };
        Ok(Ref { value, borrow: b })
      }
      None => Err(BorrowError {
        // If a borrow occurred, then we must already have an outstanding borrow,
        // so `borrowed_at` will be `Some`
        #[cfg(feature = "debug_refcell")]
        location: self.borrowed_at.get().unwrap(),
      }),
    }
  }
  /// Mutably borrows the wrapped value.
  ///
  /// The borrow lasts until the returned `RefMut` or all `RefMut`s derived
  /// from it exit scope. The value cannot be borrowed while this borrow is
  /// active.
  ///
  /// # Panics
  ///
  /// Panics if the value is currently borrowed. For a non-panicking variant, use
  /// [`try_borrow_mut`](#method.try_borrow_mut).
  ///
  /// # Examples
  /// let c = RefCell::new("hello".to_owned());
  /// *c.borrow_mut() = "bonjour".to_owned();
  /// assert_eq!(&*c.borrow(), "bonjour");
  ///
  /// An example of panic:
  ///
  /// ```should_panic
  /// let c = RefCell::new(5);
  /// let m = c.borrow();
  /// let b = c.borrow_mut(); // this causes a panic
  /// ```
  #[stable(feature = "rust1", since = "1.0.0")]
  #[inline]
  #[track_caller]
  pub fn borrow_mut(&self) -> RefMut<'_, T> {
    self.try_borrow_mut().expect("already borrowed")
  }
  /// Mutably borrows the wrapped value, returning an error if the value is currently borrowed.
  ///
  /// The borrow lasts until the returned `RefMut` or all `RefMut`s derived
  /// from it exit scope. The value cannot be borrowed while this borrow is
  /// active.
  ///
  /// This is the non-panicking variant of [`borrow_mut`](#method.borrow_mut).
  ///
  /// # Examples
  /// use std::cell::RefCell;
  ///
  /// let c = RefCell::new(5);
  /// {
  ///     let m = c.borrow();
  ///     assert!(c.try_borrow_mut().is_err());
  /// }
  /// assert!(c.try_borrow_mut().is_ok());
  #[stable(feature = "try_borrow", since = "1.13.0")]
  #[inline]
  #[cfg_attr(feature = "debug_refcell", track_caller)]
  pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
    match BorrowRefMut::new(&self.borrow) {
      Some(b) => {
        #[cfg(feature = "debug_refcell")]
        {
          self.borrowed_at.set(Some(crate::panic::Location::caller()));
        }

        // SAFETY: `BorrowRefMut` guarantees unique access.
        let value = unsafe { NonNull::new_unchecked(self.value.get()) };
        Ok(RefMut { value, borrow: b, marker: PhantomData })
      }
      None => Err(BorrowMutError {
        // If a borrow occurred, then we must already have an outstanding borrow,
        // so `borrowed_at` will be `Some`
        #[cfg(feature = "debug_refcell")]
        location: self.borrowed_at.get().unwrap(),
      }),
    }
  }
  
  /// Returns a mutable reference to the underlying data.
  ///
  /// Since this method borrows `RefCell` mutably, it is statically guaranteed
  /// that no borrows to the underlying data exist. The dynamic checks inherent
  /// in [`borrow_mut`] and most other methods of `RefCell` are therefore
  /// unnecessary.
  ///
  /// This method can only be called if `RefCell` can be mutably borrowed,
  /// which in general is only the case directly after the `RefCell` has
  /// been created. In these situations, skipping the aforementioned dynamic
  /// borrowing checks may yield better ergonomics and runtime-performance.
  ///
  /// In most situations where `RefCell` is used, it can't be borrowed mutably.
  /// Use [`borrow_mut`] to get mutable access to the underlying data then.
  ///
  /// [`borrow_mut`]: RefCell::borrow_mut()
  ///
  /// # Examples
  /// let mut c = RefCell::new(5);
  /// *c.get_mut() += 1;
  /// assert_eq!(c, RefCell::new(6));
  #[inline]
  #[stable(feature = "cell_get_mut", since = "1.11.0")]
  pub fn get_mut(&mut self) -> &mut T {
    self.value.get_mut()
  }
}
```

RefCell<T>에는 다음과 같은 특징이 있다.

1. 참조형이므로 런타임 오버헤드가 적고 복사할 수 없다.
   `RefCell<T>` type은 참조 type으로 구현되며, 이는 실제 데이터를 포함하는 메모리 위치에 대한 pointer임을 의미한다. 여기에는 몇 가지 의미가 있다.
   - 참조 type은 함수에 전달되거나 변수에 할당될 때 값이 복사되지 않기 때문에 런타임 오버헤드가 낮다. 대신 데이터의 메모리 주소만 전달됩니다.
     이는 성능 측면에서 RefCell<T>를 효율적으로 사용할 수 있도록 한다.(여기서 말하는 '복사되지 않는다'의 의미는 값에 대한 deep copy(clone())를 하지 않으며,
     값의 pointer에 대한 swallow copy(pointer copy)는 수행한다.)
   - RefCell<T>은 참조 type이므로 값을 복사할 수 없다. 즉, RefCell<T>을 다른 변수에 할당하거나 함수에 전달할 때,
     실제로 복사본이 아닌 원본 RefCell<T>에 대한 참조를 전달하는 것이다. 이는 코드에서 RefCell<T>를 사용하는 방법에 영향을 미치기 때문에 명심해야 한다.
    ```rust
    use std::cell::RefCell;
    
    fn main() {
        let my_ref_cell = RefCell::new(42);
        let reference = &my_ref_cell;
        let value = *reference.borrow(); // Dereference the resulting reference to get the actual value
        let copy = my_ref_cell; // Compile error
   
    
        println!("{:?}", reference.borrow()); // Prints 42
        println!("The value is {}", value); // Dereference the resulting reference to get the actual value and print it.
        println!("{:?}", copy.borrow()); // Compile error
    }
    ```
     reference 변수에서 borrow()를 호출하려고 하면 제대로 작동하고 42를 출력하는 것을 볼 수 있다.
     그러나 copy 변수에서 borrow()를 호출하려고 하면 컴파일 오류가 발생한다. 이는 RefCell의 복사본을 만들 수 없기 때문이다. 참조만 만들 수 있다. 
     이는 RefCell을 복사할 수 없지만 대신 참조로 전달하거나 원래 RefCell에 대한 참조를 사용해야 함을 보여준다.  
     reference 변수는 원래의 RefCell의 복사본이 아닌, 원래 RefCell에 대한 '참조'이다.
     reference에서 borrow()를 사용할 때 여전히 원래 RefCell에 액세스하고 있다.

2. 내용에 대한 참조를 얻기 위해 borrow 및 borrow_mut의 두 가지 방법이 있다.
   - borrow()는 내부 값에 대한 불변 참조를 반환한다. 이것은 RefCell<T>에 대한 공유 참조를 매개변수로 취하고 Ref<T> struct를 반환한다.
     여기서 주의할 것은 borrow()메서드로 생성한 새로운 참조 변수는 RefCell<T> type이 아니라, Ref<T> type이라는 것이다.
     이것은 본질적으로 런타임에 Rust의 borrow rule을 시행하는 smart pointer이다.
     여러 Ref<T>가 동시에 존재할 수 있지만 동일한 데이터에 대한 변경 가능한 참조(RefMut<T>)와 공존할 수는 없다.(compile은 된다.)
     mutable reference가 존재하는 경우에 borrow() 호출을 시도하면 런타임 panic!이 발생한다.
   - borrow_mut()은 내부 값에 대한 변경 가능한 참조를 반환한다. RefCell<T>에 대한 배타적 참조를 매개변수로 사용하고 RefMut<T> struct를 반환한다.
     한 번에 하나의 RefMut<T>만 존재할 수 있으며 어떤 Ref<T>와도 공존할 수 없다. 우리가 아는 &mut 과 같이, RefMut type의 변수를 수정하면 원본 RefCell<T>가 수정된다.
   - 이러한 메서드는 공유 데이터를 변경해야 하는 경우에 유용할 수 있는 runtime에 dyn borrow check를 수행하는 방법을 제공한다.
     그러나 borrow check로 인한 런타임 오버헤드 및 런타임 패닉 가능성과 같은 일부 장단점이 있다.
     RefCell<T>를 신중하게 사용하고 Ref<T> 및 RefMut<T> struct에 의해 시행되는 borrow rule을 이해하는 것이 중요하다.
   
3. RefCell<T>는 스레드로부터 안전하지 않다.
   - RefCell<T>를 여러 스레드에서 동시에 사용하는 것은 안전하지 않다.
     이는 RefCell<T>이 메모리 안전성을 단일 스레드 내에서 작동할때만 강화하고, 오버헤드를 줄이기 위해 Rust의 borrow rule에 의존하기 때문이다.
     때문에 RefCell<T>에는 다중 스레드 환경에서 데이터의 안전성을 위한 lock을 포함한 여러 동기 primitives들의 구현이 없다.
     borrow checker는 다중 스레드 환경에서 데이터의 일관성 및 data race 또는 기타 메모리 안전문제를 방지하지 못하기 때문에,
     RefCell<T>이 다중 스레드 환경에 놓이면 컴파일이 실패하게끔 RefCell<T>은 Send trait을 구현하지 않았다.
     여러 스레드가 동일한 RefCell<T>에 동시에 액세스하는 경우 borrow rule은 data race 또는 기타 메모리 안전 문제를 방지할 수 없다.

4. borrow rule을 위반하면 런타임에 패닉이 발생할 수 있다.
   - Rust의 borrow checker는 `static borrow checker`와 `dynamic borrow checker` 두 가지로 구성되어 있다.
   - static은 compile time에 borrow rule 위반 가능성을 감지하는 정적 분석 도구이며,
     참조가 invalidated 되거나 borrow rule을 위반하는 방식으로 사용되지 않는지 확인한다. 잠재적 위반을 감지하면 컴파일 되지 않는다.
   - dyn은 runtime 검사를 수행하여 borrow rule이 runtime에 위반되지 않도록 한다. 위반이 감지되면 프로그램이 panic!상태가 되어 종료된다.
   - 그러나 어떤 경우에는 static borrow checker가 compile time에 모든 잠재적 위반을 포착할 수 없다.
     이러한 경우 borrow checker는 borrow rule이 위반되지 않았는지 확인하기 위해 runtime check에 의존한다.
   - RefCell<T> type은 static borrow check를 우회하고, 공유 및 mutable reference에 대한 runtime(dyn) borrow checker를 이용해 검사한다.
     RefCell<T>에서 Borrow() 메서드를 호출하면 래핑된 값에 대한 immutable reference가 반환된다.
     래핑된 값에 대한 mutable reference가 이미 있는 경우 Borrow()를 호출하면 런타임에 패닉이 발생한다.
     마찬가지로, RefCell<T>에 대해 borrow_mut() 메서드를 호출하면 래핑된 값에 대한 mutable reference를 반환하고
     래핑된 값에 대한 변경 가능하거나 변경 불가능한 참조가 이미 있는 경우 borrow_mut()를 호출하면 런타임에 패닉이 발생한다.
     즉, dyn borrow checker도 borrow rule을 동일하게 따른다.
   - runtime panic을 방지하려면 항상 borrow rule을 따르고 이를 위반하지 않도록 해야 한다.
     panic하지 않고 Result type을 반환하는 try_borrow() 및 try_borrow_mut() 메서드를 사용하는 것도 좋은 방법이다.
     이렇게 하면 프로그램을 종료하지 않고도 borrow errors를 정상적으로 gracefully하게 처리할 수 있다.
     borrow checker는 컴파일 시간에 Rust의 borrow rule에 대한 잠재적인 위반을 감지하지만 경우에 따라 모든 잠재적 위반을 포착할 수 없으며
     borrow rule이 위반되지 않았는지 확인하기 위해 runtime check가 필요하다.
