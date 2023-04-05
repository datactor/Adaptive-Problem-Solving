# Unlocking the Crossbeam: Dive into Rust's Advanced Synchronization Primitives

## 1. Introduction
### Definition of synchronization primitives
동시 프로그래밍에서 동기 프리미티브는 공유 리소스에 대한 액세스를 조정하거나
한 번에 하나의 스레드만 특정 리소스에 액세스할 수 있도록 하는 데 사용되는 메커니즘이다.
동기 프리미티브의 예로는 lock, semaphores 및 atomic operations 등이 있다.

### Importance of synchronization primitives in concurrent programming
동시성은 최신 컴퓨팅의 필수 요소이지만 고유한 문제가 있다.
가장 큰 과제 중 하나는 여러 스레드가 안전하고 효율적인 방식으로 공유 리소스에 액세스할 수 있도록 하는 것이다.
동기 프리미티브는 공유 리소스에 대한 액세스를 조정하고 race condition, deadlocks 및 기타 동시성 버그를 방지하는 방법을 제공하기 때문에 중요하다.

### Overview of what will be covered in the article
이 기사에서는 크로스빔 크레이트에서 제공하는 고급 동기화 기본 기능, std 라이브러리의 cell과 sync mudule에 중점을 두고
Rust의 동기화 기본 기능에 대한 포괄적인 이해를 돕는 것이 목적이다.

이러한 고급 동기 프리미티브를 완전히 이해하려면 이전에 다룬 [Rc<RefCell< T >>](https://github.com/datactor/rust-problem-solving/blob/main/forge/rust_concepts_explained/rcRefcell_and_refCycle.md),
[Arc<Mutex< T >>](https://github.com/datactor/rust-problem-solving/blob/main/forge/rust_concepts_explained/arc_mutex.md),
[lazy_vs_eager](https://github.com/datactor/rust-problem-solving/blob/main/forge/rust_concepts_explained/lazy_vs_eager.md) 및
[async/await](https://github.com/datactor/rust-problem-solving/blob/main/forge/rust_concepts_explained/async_await.md)
과 같은 Rust의 기본 동시성 개념을 먼저 이해하는 것이 좋다.

crossbeam's channel, atomic types 및 work stealing API에 대해 살펴보기 전에
Rust의 cell과 sync를 포함한 Rust의 표준 동기 프리미티브에 대한 개요부터 알아볼 것이다.
기사가 끝날 때쯤이면 이러한 동기 프리미티브를 사용하여 안전하고 효율적인 동시 Rust 프로그램을 작성하는 방법에 대한 이해도가 높아져 있을 것이다.

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
   - 참조 type은 함수에 전달되거나 변수에 할당될 때 값이 복사되지 않기 때문에 런타임 오버헤드가 낮다. 대신 데이터의 메모리 주소만 전달된다.
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
     여러 Ref<T>가 동시에 존재할 수 있지만 동일한 데이터에 대한 변경 가능한 참조(RefMut<T>)와 공존할 수는 없다(compile은 되지만 runtime panic!).
     mutable reference가 존재하는 경우에 borrow() 호출을 시도하면 런타임 panic!이 발생한다.
   - borrow_mut()은 내부 값에 대한 변경 가능한 참조를 반환한다. RefCell<T>에 대한 배타적 참조를 매개변수로 사용하고 RefMut<T> struct를 반환한다.
     한 번에 하나의 RefMut<T>만 존재할 수 있으며 어떤 Ref<T>와도 공존할 수 없다. 우리가 아는 &mut 과 같이, RefMut type의 변수를 수정하면 원본 RefCell<T>가 수정된다.
   - 이러한 메서드는 공유 데이터를 변경해야 하는 경우에 유용할 수 있는 runtime에 dyn borrow check를 수행하는 방법을 제공한다.
     그러나 borrow check로 인한 런타임 오버헤드 및 런타임 패닉 가능성과 같은 일부 장단점이 있다.
     RefCell<T>를 신중하게 사용하고 Ref<T> 및 RefMut<T> struct에 의해 시행되는 borrow rule을 이해하는 것이 중요하다.
   
3. RefCell<T>은 스레드로부터 안전하지 않다.
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

### UnsafeCell: definition, how to use, and trade-offs
UnsafeCell<T>는 Rust에서 공유 가능한 가장 낮은 수준의 mutable 컨테이너이다.
스레드 간에 공유할 수 있고 런타임 안전 보장을 제공하지 않는 값 T에 대한 raw pointer이다.
UnsafeCell<T>는 자체 동기 프리미티브 또는 데이터 구조를 구현해야 하거나
내부 가변성이 필요한 낮은 수준의 안전하지 않은 작업을 수행해야 할 때 사용된다.
UnsafeCell<T>에는 다음과 같은 특징이 있다.

- raw pointer type이므로 적절한 주의 없이 사용하는 것은 안전하지 않다.
- 내용에 대한 포인터일 뿐이므로 메서드가 없다.
- 잘못 사용하면 정의되지 않은 동작이 발생할 수 있다.

### std::cell: Cell vs. RefCell vs. UnsafeCell, and When to Use Each One
Rust의 std::cell 모듈은 Cell, RefCell 및 UnsafeCell을 제공하며 각각 고유한 특징과 사용 사례가 있다.
특정 사용 사례에 적합한 컨테이너를 선택하는 것은 안전하고 효율적인 동시 프로그램을 작성하는 데 중요하다.
다음은 각 컨테이너를 사용하는 경우에 대한 간단한 요약이다.

- 단일 스레드 내에서 공유되고 여러 참조 또는 런타임 borrow check가 필요하지 않으며, 값을 변경해야 하는 경우 Cell<T>를 사용한다.
- 단일 스레드 내에서 공유되고 여러 참조 또는 런타임 borrow check가 필요하며, 값을 변경해야 하는 경우 RefCell<T>를 사용한다.
- 고유 동기 프리미티브 또는 데이터 구조를 구현해야 하거나 내부 가변성이 필요한 낮은 수준의 안전하지 않은 작업을 수행해야 하는 경우
  UnsafeCell<T>를 사용한다. Cell type의 struct들은 기본적으로 내부에 UnsafeCell 필드를 가지고 있다.

## 3. Overview of Rust's standard synchronization primitives.
Rust는 여러 스레드에서 공유 데이터에 대한 액세스를 조정하는 데 사용할 수 있는 다양한 동기 프리미티브를 제공한다.
이러한 프리미티브는 모두 std::sync 모듈의 일부이며 자세한 내용은 아래의 링크를 참고한다.  
https://doc.rust-lang.org/core/sync/atomic/index.html

### std::sync: Atomic types and ordering guarantees
여러 스레드가 공유 메모리 위치에 엑세스할 때 데이터가 일관되고 손상되지 않도록 동기화가 필요하다.
동기화에 대한 한 가지 접근 방식은 한 번에 하나의 스레드만 데이터에 엑세스할 수 있도록 하는 `lock`을 사용하는 것이다.
여기서 `lock`은 스레드 간에 오버헤드 및 deadlock등의 문제를 유발할 수 있다.

__atomic type은 동기화에 대한 대체 접근 방식을 제공한다.__
그렇기에 atomic type은 스레드 간에 기본 공유 메모리 통신을 제공하며, 다른 concurrent types들의 building blocks이다.
atomic type들은 `compare-and-swap`과 같은 atomic operation을 사용하여 `lock` 없이 기술적으로 공유 메모리의 변수 값을 업데이트한다.
atomic operation은 변수가 atomically 업데이트되도록 보장한다.
즉, 작업이 다른 스레드에 의해 중단될 수 없으므로 일관되고 올바른 데이터가 생성된다.

아래는 concurrent programming 책을 보고 rust로 작동 방식을 보기 위해 구현해본 compare_and_swap이다.
```rust
/// CAS(Compare and Swap)은 동기 처리 기능의 하나인 세마포어(semaphore), lock-free, wait-free한 데이터 구조를
/// 구현하기 위해 이용하는 처리다.
fn compare_and_swap(mut p: u64, val: u64, new_val: u64) -> bool {
    if p != val {
        return false
    } p = new_val;
    true
}
// 여기서 반환값은 compare_and_swap의 성공 여부이다. 현재 값이 예상 값과 일치할 경우에만 compare_and_swap을 시도한다.
// 그렇지만 이 함수는 아토믹하다고 할 수 없다. 실제로 2행의 p != val은 4행의 p = new_val과 별도로 실행된다.
// 위 함수는 컴파일되어 어셈블리 레벨에서도 여러 조작을 조합해 구현된다. rust에도 이와 같은 조작을 아토믹으로 처리하기 위한 내장함수인
// compare_and_swap() 함수가 있다.
```

```rust
use std::sync::atomic::{AtomicU64, Ordering};

fn compare_and_swap(p: &AtomicU64, val: u64, newval: u64) -> bool {
    p.compare_and_swap(val, newval, Ordering::SeqCst) == val
}
```

예를 들어 여러 스레드가 동일한 AtomicU64를 증가시키는 경우 각 스레드는 다른 스레드를 방해하지 않고 값을 원자적으로 증가시킨다.
이렇게 하면 정수의 최종 값이 경쟁 조건이나 스레드 간섭으로 인한 임의의 값이 아니라 모든 증분의 합계가 된다.

Rust는 atomic operation에 대한 ordering guarantees도 제공하는데, atomic operation이 다른 메모리 연산과 비교하여 실행되는 방법을 지정한다.
사용 가능한 옵션은 Relaxed, Release, Acquire, Acquire + Release 및 Sequentially Consistent이다.
- AtomicOrdering 
  1. Relaxed: 순서를 보장하지 않음. data race 가능성 있음
  2. Acquire: 현재 스레드가 현재 작업 이전에 다른 스레드가 수행한 모든 메모리 작업을 관찰할 수 있는지 확인. 즉, 다른 스레드 작업을 관찰하고 획득 하라는 것(다른 스레드가 '작업 중'일때 획득하지 말라는 것)
     이는 스레드 간에 데이터를 동기화하여 현재 스레드가 가장 최신 버전의 데이터를 갖도록 하는 데 유용하다
  3. Release: 현재 스레드가 수행한 작업 중일 때, 작업을 마친 이후에 다른 스레드에 의해 관찰되도록 함. (현재 스레드가 '작업 중'일 때 다른 스레드에서 관찰하여 획득할 수 없음) 이는 스레드 간에 데이터를 동기화해 현재 스레드가 데이터 수정을 완료한 후 다른 스레드가 일관된 데이터 보기를 볼 수 있도록 하는데 유용하다.
  4. AcqRel: Acquire + Release 모두 제공
  5. SeqCst: 모든 메모리 작업은 모든 스레드에서 동일한 순서로 관찰한다. 즉 모든 스레드에서, 스레드마다 작업이 수행한 이후 다음 스레드가 관찰되도록 하기 + 다른 스레드 작업을 관찰하고 획득하기 이렇게 하면 Queue에 들어있는 것처럼, 스레드마다 순서가 정해져있는 것처럼 작동돼 순차 일관성이 보장된다.

`AtomicBool`과 `AtomicPtr`은 내부 필드에 값으로 UnsafeCell을 취한다. 내부 가변성을 제공하기 위함이다.
앞서 설명했듯이 UnsafeCell은 Rust의 안전한 메모리 관리에 대한 일반적인 borrow rule이 이를 방지하는 경우에도 우회하여,
콘텐츠를 변경하는 기능을 제공하는 type이다. atomic struct의 기본 값을 `UnsafeCell`로 래핑함으로써 내용이 변경될 수 있도록 하면서
여러 스레드에서 데이터에 atomically 엑세스하고 수정할 수 있도록 한다.  
즉, `UnsafeCell`에 atomic operation을 적용하여 데이터 경합을 일으키지 않고 내부 값이 여러 스레드에 의해 원자적으로 변경될 수 있다.

즉 Atomic struct는 내부 가변성을 가진 Cell struct와 atomic operation 기능을 가진 struct라고 보면 되겠다. 
cell type과 동일한 메서드인 get, get_mut 등의 메서드는 그 자체만으로는 atomic operation이 아니다

```rust
pub struct AtomicBool {
    v: UnsafeCell<u8>,
}

impl AtomicBool {
    pub const fn new(v: bool) -> AtomicBool {
        AtomicBool { v: UnsafeCell::new(v as u8) }
    }

    pub fn get_mut(&mut self) -> &mut bool {
        // SAFETY: the mutable reference guarantees unique ownership.
        unsafe { &mut *(self.v.get() as *mut bool) }
    }

    pub fn from_mut(v: &mut bool) -> &mut Self {
        // SAFETY: the mutable reference guarantees unique ownership, and
        // alignment of both `bool` and `Self` is 1.
        unsafe { &mut *(v as *mut bool as *mut Self) }
    }

    pub fn load(&self, order: Ordering) -> bool {
        // SAFETY: any data races are prevented by atomic intrinsics and the raw
        // pointer passed in is valid because we got it from a reference.
        unsafe { atomic_load(self.v.get(), order) != 0 }
    }

    pub fn store(&self, val: bool, order: Ordering) {
        // SAFETY: any data races are prevented by atomic intrinsics and the raw
        // pointer passed in is valid because we got it from a reference.
        unsafe {
            atomic_store(self.v.get(), val as u8, order);
        }
    }

    pub fn swap(&self, val: bool, order: Ordering) -> bool {
        // SAFETY: data races are prevented by atomic intrinsics.
        unsafe { atomic_swap(self.v.get(), val as u8, order) != 0 }
    }

    pub fn compare_and_swap(&self, current: bool, new: bool, order: Ordering) -> bool {
        match self.compare_exchange(current, new, order, strongest_failure_ordering(order)) {
            Ok(x) => x,
            Err(x) => x,
        }
    }
}

pub struct AtomicPtr<T> {
    p: UnsafeCell<*mut T>,
}

impl<T> AtomicPtr<T> {
    pub const fn new(p: *mut T) -> AtomicPtr<T> {
        AtomicPtr { p: UnsafeCell::new(p) }
    }

    pub fn get_mut(&mut self) -> &mut *mut T {
        self.p.get_mut()
    }

    pub fn from_mut(v: &mut *mut T) -> &mut Self {
        use crate::mem::align_of;
        let [] = [(); align_of::<AtomicPtr<()>>() - align_of::<*mut ()>()];
        // SAFETY:
        //  - the mutable reference guarantees unique ownership.
        //  - the alignment of `*mut T` and `Self` is the same on all platforms
        //    supported by rust, as verified above.
        unsafe { &mut *(v as *mut *mut T as *mut Self) }
    }

    pub fn get_mut_slice(this: &mut [Self]) -> &mut [*mut T] {
        // SAFETY: the mutable reference guarantees unique ownership.
        unsafe { &mut *(this as *mut [Self] as *mut [*mut T]) }
    }

    pub fn from_mut_slice(v: &mut [*mut T]) -> &mut [Self] {
        // SAFETY:
        //  - the mutable reference guarantees unique ownership.
        //  - the alignment of `*mut T` and `Self` is the same on all platforms
        //    supported by rust, as verified above.
        unsafe { &mut *(v as *mut [*mut T] as *mut [Self]) }
    }

    pub fn load(&self, order: Ordering) -> *mut T {
        // SAFETY: data races are prevented by atomic intrinsics.
        unsafe { atomic_load(self.p.get(), order) }
    }

    pub fn store(&self, ptr: *mut T, order: Ordering) {
        // SAFETY: data races are prevented by atomic intrinsics.
        unsafe {
            atomic_store(self.p.get(), ptr, order);
        }
    }

    pub fn swap(&self, ptr: *mut T, order: Ordering) -> *mut T {
        // SAFETY: data races are prevented by atomic intrinsics.
        unsafe { atomic_swap(self.p.get(), ptr, order) }
    }

    pub fn compare_and_swap(&self, current: *mut T, new: *mut T, order: Ordering) -> *mut T {
        match self.compare_exchange(current, new, order, strongest_failure_ordering(order)) {
            Ok(x) => x,
            Err(x) => x,
        }
    }
}
```

이외의 atomic scalar types들은 struct가 아닌, const type으로 구현되어 있다.
그 이유는 스칼라 타입을 유지하여, 사이즈드 된 값을 유지하고 bitwise 연산이 원활하게 수행되게 하기 위해서이다.
const 값으로 정의함으로써 Rust compiler는 atomic scalar types들이 정확한 크기와 정렬을 갖도록 보장할 수 있고,
그것들을 bitwise atomic operation에 적합하게 만든다.  

Atomic 스칼라 types들에는 fetch_add, fetch_sub 등을 포함하여 공유 메모리에서 atomic operation을 수행하기 위한 여러 함수와 매크로를 제공한다.
기본 atomic types에서 값을 원자적으로 더하거나 빼고 이전 값을 검색할 수 있다.
```rust
use std::sync::atomic::{AtomicI32, Ordering};

let my_atomic_int = AtomicI32::new(5);

let old_value = my_atomic_int.fetch_add(3, Ordering::SeqCst);

assert_eq!(old_value, 5);
assert_eq!(my_atomic_int.load(Ordering::SeqCst), 8);
```
이 예에서는 초기 값이 5인 새로운 AtomicI32를 생성한다.
그런 다음 fetch_add(3, Ordering::SeqCst)를 호출하여 기본 정수에 원자적으로 3을 더하고 이전 값을 반환한다(이 경우 5).
마지막으로 load(Ordering::SeqCst)를 사용하여 atomic integer의 새 값(이 경우 8)을 검색한다.

fetch_sub는 fetch_add와 유사하게 작동하지만 atomic integer에서 주어진 값을 뺀다.

fetch_add 및 fetch_sub 모두 작업에 대한 순서 보장을 결정하는 Ordering 인수를 사용한다.
사용 가능한 옵션은 load 및 store와 같은 다른 atomic operation과 동일하다.

atomic types 및 ordering guarantees을 사용하면 다중 스레드 프로그램에서 데이터 경합을 방지하고 메모리 안전을 보장하는 동시에,
lock 및 기타 동기 프리미티브의 필요성을 줄여 오버헤드를 줄이고 성능을 향상시킬 수 있다.

### Arc: definition, how to use, and trade-offs
Arc는 "Atomically Reference Counted"를 의미하며 스레드 간에 값의 소유권을 공유하는 스레드 안전 방식이다.
여러 스레드가 값의 소유권을 공유할 수 있다는 점을 제외하면 Rc와 유사하다.
Arc::new()로 생성된 인스턴스에 대해서 Arc::clone()으로 복제(새로운 인스턴스를 생성)하면, strong count를 fetch_add로 원자적으로 업데이트 하고,
새로 생성된 인스턴스는 원본 Arc의 ArcInner 값을 감싼 Null이 아님을 보증하는 가벼운 pointer인 NonNull 포인터와 phantom 필드를 가진 Arc 타입의 인스턴스를 반환한다.
여기서 NonNull 포인터는 스마트포인터의 기능인 라이프타임 관리 기능이 없어, Arc struct에 따로 phantom 필드를 넣어 원본 ArcInner 값과 라이프타임을 연동시킨다.

Arc의 카운팅은 CAS와 spin lock(lock-free algorithms), AtomicOrdering등을 포함한 동기 primitives를 사용하여 원자적으로 업데이트 되기 때문에, lock 없이도 스레드 간 안전한 방식으로 분류된다. 

자세한 정보는 https://doc.rust-lang.org/alloc/sync/struct.Arc.html 를 참조한다.
```rust
const MAX_REFCOUNT: usize = (isize::MAX) as usize;

pub struct Arc<T: ?Sized> {
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
}

#[repr(C)]
struct ArcInner<T: ?Sized> {
    strong: atomic::AtomicUsize,

    // the value usize::MAX acts as a sentinel for temporarily "locking" the
    // ability to upgrade weak pointers or downgrade strong ones; this is used
    // to avoid races in `make_mut` and `get_mut`.
    weak: atomic::AtomicUsize,

    data: T,
}

impl<T> Arc<T> {
    /// Constructs a new `Arc<T>`.
    ///
    /// # Examples
    /// use std::sync::Arc;
    /// let five = Arc::new(5);
    pub fn new(data: T) -> Arc<T> {
        // Start the weak pointer count as 1 which is the weak pointer that's
        // held by all the strong pointers (kinda), see std/rc.rs for more info
        let x: Box<_> = Box::new(ArcInner {
            strong: atomic::AtomicUsize::new(1),
            weak: atomic::AtomicUsize::new(1),
            data,
        });
        unsafe { Self::from_inner(Box::leak(x).into()) }
    }

    pub fn downgrade(this: &Self) -> Weak<T> {
        // This Relaxed is OK because we're checking the value in the CAS
        // below.
        let mut cur = this.inner().weak.load(Relaxed);

        loop {
            // check if the weak counter is currently "locked"; if so, spin.
            if cur == usize::MAX {
                hint::spin_loop();
                cur = this.inner().weak.load(Relaxed);
                continue;
            }

            // NOTE: this code currently ignores the possibility of overflow
            // into usize::MAX; in general both Rc and Arc need to be adjusted
            // to deal with overflow.

            // Unlike with Clone(), we need this to be an Acquire read to
            // synchronize with the write coming from `is_unique`, so that the
            // events prior to that write happen before this read.
            match this.inner().weak.compare_exchange_weak(cur, cur + 1, Acquire, Relaxed) {
                Ok(_) => {
                    // Make sure we do not create a dangling Weak
                    debug_assert!(!is_dangling(this.ptr.as_ptr()));
                    return Weak { ptr: this.ptr };
                }
                Err(old) => cur = old,
            }
        }
    }

    pub fn weak_count(this: &Self) -> usize {
        let cnt = this.inner().weak.load(Acquire);
        // If the weak count is currently locked, the value of the
        // count was 0 just before taking the lock.
        if cnt == usize::MAX { 0 } else { cnt - 1 }
    }

    pub fn strong_count(this: &Self) -> usize {
        this.inner().strong.load(Acquire)
    }

    pub fn inner(&self) -> &ArcInner<T> {
        // This unsafety is ok because while this arc is alive we're guaranteed
        // that the inner pointer is valid. Furthermore, we know that the
        // `ArcInner` structure itself is `Sync` because the inner data is
        // `Sync` as well, so we're ok loaning out an immutable pointer to these
        // contents.
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if this.is_unique() {
            // This unsafety is ok because we're guaranteed that the pointer
            // returned is the *only* pointer that will ever be returned to T. Our
            // reference count is guaranteed to be 1 at this point, and we required
            // the Arc itself to be `mut`, so we're returning the only possible
            // reference to the inner data.
            unsafe { Some(Arc::get_mut_unchecked(this)) }
        } else {
            None
        }
    }

    fn is_unique(&mut self) -> bool {
        // lock the weak pointer count if we appear to be the sole weak pointer
        // holder.
        //
        // The acquire label here ensures a happens-before relationship with any
        // writes to `strong` (in particular in `Weak::upgrade`) prior to decrements
        // of the `weak` count (via `Weak::drop`, which uses release).  If the upgraded
        // weak ref was never dropped, the CAS here will fail so we do not care to synchronize.
        if self.inner().weak.compare_exchange(1, usize::MAX, Acquire, Relaxed).is_ok() {
            // This needs to be an `Acquire` to synchronize with the decrement of the `strong`
            // counter in `drop` -- the only access that happens when any but the last reference
            // is being dropped.
            let unique = self.inner().strong.load(Acquire) == 1;

            // The release write here synchronizes with a read in `downgrade`,
            // effectively preventing the above read of `strong` from happening
            // after the write.
            self.inner().weak.store(1, Release); // release the lock
            unique
        } else {
            false
        }
    }
}

impl<T: ?Sized> Clone for Arc<T> {
    fn clone(&self) -> Arc<T> {
        let old_size = self.inner().strong.fetch_add(1, Relaxed);
        if old_size > MAX_REFCOUNT {
            abort();
        }
        unsafe { Self::from_inner(self.ptr) }
    }
}


impl<T: Clone> Arc<T> {
    /// Makes a mutable reference into the given `Arc`.
    pub fn make_mut(this: &mut Self) -> &mut T {
        // Note that we hold both a strong reference and a weak reference.
        // Thus, releasing our strong reference only will not, by itself, cause
        // the memory to be deallocated.
        //
        // Use Acquire to ensure that we see any writes to `weak` that happen
        // before release writes (i.e., decrements) to `strong`. Since we hold a
        // weak count, there's no chance the ArcInner itself could be
        // deallocated.
        if this.inner().strong.compare_exchange(1, 0, Acquire, Relaxed).is_err() {
            // Another strong pointer exists, so we must clone.
            // Pre-allocate memory to allow writing the cloned value directly.
            let mut arc = Self::new_uninit();
            unsafe {
                let data = Arc::get_mut_unchecked(&mut arc);
                (**this).write_clone_into_raw(data.as_mut_ptr());
                *this = arc.assume_init();
            }
        } else if this.inner().weak.load(Relaxed) != 1 {
            // Relaxed suffices in the above because this is fundamentally an
            // optimization: we are always racing with weak pointers being
            // dropped. Worst case, we end up allocated a new Arc unnecessarily.

            // We removed the last strong ref, but there are additional weak
            // refs remaining. We'll move the contents to a new Arc, and
            // invalidate the other weak refs.

            // Note that it is not possible for the read of `weak` to yield
            // usize::MAX (i.e., locked), since the weak count can only be
            // locked by a thread with a strong reference.

            // Materialize our own implicit weak pointer, so that it can clean
            // up the ArcInner as needed.
            let _weak = Weak { ptr: this.ptr };

            // Can just steal the data, all that's left is Weaks
            let mut arc = Self::new_uninit();
            unsafe {
                let data = Arc::get_mut_unchecked(&mut arc);
                data.as_mut_ptr().copy_from_nonoverlapping(&**this, 1);
                ptr::write(this, arc.assume_init());
            }
        } else {
            // We were the sole reference of either kind; bump back up the
            // strong ref count.
            this.inner().strong.store(1, Release);
        }

        // As with `get_mut()`, the unsafety is ok because our reference was
        // either unique to begin with, or became one upon cloning the contents.
        unsafe { Self::get_mut_unchecked(this) }
    }
    pub fn unwrap_or_clone(this: Self) -> T {
        Arc::try_unwrap(this).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl<T: ?Sized> Weak<T> {
    pub fn upgrade(&self) -> Option<Arc<T>> {
        // We use a CAS loop to increment the strong count instead of a
        // fetch_add as this function should never take the reference count
        // from zero to one.
        self.inner()?
            .strong
            // Relaxed is fine for the failure case because we don't have any expectations about the new state.
            // Acquire is necessary for the success case to synchronise with `Arc::new_cyclic`, when the inner
            // value can be initialized after `Weak` references have already been created. In that case, we
            // expect to observe the fully initialized value.
            .fetch_update(Acquire, Relaxed, |n| {
                // Any write of 0 we can observe leaves the field in permanently zero state.
                if n == 0 {
                    return None;
                }
                // See comments in `Arc::clone` for why we do this (for `mem::forget`).
                if n > MAX_REFCOUNT {
                    abort();
                }
                Some(n + 1)
            })
            .ok()
            // null checked above
            .map(|_| unsafe { Arc::from_inner(self.ptr) })
    }

    pub fn strong_count(&self) -> usize {
        if let Some(inner) = self.inner() { inner.strong.load(Acquire) } else { 0 }
    }

    pub fn weak_count(&self) -> usize {
        self.inner()
            .map(|inner| {
                let weak = inner.weak.load(Acquire);
                let strong = inner.strong.load(Acquire);
                if strong == 0 {
                    0
                } else {
                    // Since we observed that there was at least one strong pointer
                    // after reading the weak count, we know that the implicit weak
                    // reference (present whenever any strong references are alive)
                    // was still around when we observed the weak count, and can
                    // therefore safely subtract it.
                    weak - 1
                }
            })
            .unwrap_or(0)
    }
}
```

위의 메서드 중 get_mut은 lock-free 알고리즘인 CAS를 응용한 방식을 사용한다.  
`is_unique` 메서드는 `weak` 포인터 수와 함께 compare_exchange 메서드를 사용해, Arc 인스턴스가 고유한지 확인한다.  
현재 'Arc' 인스턴스가 `weak` 포인터의 유일한 소유자인 것처럼 보인다면 weak 포인터 수에 대한 lock을 획득하려고 시도한다.
```rust
if self.inner().weak.compare_exchange(1, usize::MAX, Acquire, Relaxed).is_ok() {
```
위의 if 분기문 현재의 `weak` count 값이 1이면, usize::MAX로 설정하여
다른 스레드가 weak count를 더이상 획득하지 못한다는 점에서 일종의 lock 역할을 한다.
현재의 weak count가 1이면 성공적으로 획득한다. 

```rust
let unique = self.inner().strong.load(Acquire) == 1;
```
그렇게 되면, strong pointer 수가 1인지 확인하며 이것을 bool값으로 저장해 둔다(unique 변수).

```rust
self.inner().weak.store(1, Release);
```
unique를 저장했으면, lock을 해제(1로 store)하고 bool 값을 반환한다.

여기에서 CAS 알고리즘을 사용하면 한 번에 하나의 스레드만 획득하고 'Weak' count에 액세스할 수 있으므로 count의 동시 수정을 방지할 수 있다.
이것은 lock 대신 atomic operation을 사용하여 스레드 간의 경합을 최소화하는 lock-free algorithms의 예다.
여기서 말한 lock은 기술적인 analogy로 lock이라고 표기했지만, 실제로는 lock이 아니라 atomic primitives를 말한다.

다음은 compare_exchange()에 대한 구현이다.
```rust
pub fn compare_exchange(
    &self,
    current: usize,
    new: usize,
    success: Ordering,
    failure: Ordering,
) -> Result<usize, usize> {
    // Convert the success ordering to a memory order
    let success_order = success.into();
    // Convert the failure ordering to a memory order
    let failure_order = failure.into();
    
    // Use a compiler intrinsic to perform the compare-and-swap operation
    let result = unsafe {
        atomic_compare_exchange(self.ptr, current, new, success_order, failure_order)
    };
    
    // If the operation succeeded, return the old value
    if result == current {
        Ok(result)
    } else {
        // Otherwise, return the current value and the new value was not stored
        Err(result)
    }
}
```

Arc 사용의 단점 중 하나는 참조 카운팅 프로세스(lock free algorithms)에 오버헤드가 추가되어 프로그램 속도가 느려질 수 있다는 것이다.
또한 스레드 간의 공유 값에 대한 작업 순서를 보장할 수 없다는 것이다. 공유 값에 대한 작업의 순서 지정에 사용되는 Ordering::Release 일관성 모델로 인해
각 스레드가 이벤트의 다른 순서를 관찰할 수 있기 때문이다.
프로그램의 정확성을 위해 작업 순서가 중요한 경우 미묘한 버그가 발생할 수 있다.

### Barrier: definition, how to use, and trade-offs
다음은 concurrent programming (다카노 유키)에서 구현한 배리어 동기이다.

단체 생활의 이동을 생각해 보자. 이동은 반드시 클래스 전체가 모였는지 확인한 후 진행한다.
이렇게 모두 모인 후에 실행 동기를 구현하는 것이 barrier synchronization이다.  
다음은 spin-lock 기반의 barrier synchronization이다.
```rust
fn barrier(cnt: &mut AtomicUsize, max: &mut AtomicUsize) { // 1
    cnt.fetch_add(1, Ordering::SeqCst); // 2
    while cnt.load(Ordering::SeqCst) < max.load(Ordering::SeqCst) {}; // 3
}
```
1) 공유 변수에 대한 값 cnt와 최대값 max를 인자로 받는다.
2) 공유 변수 cnt를 아토믹하게 증가시킨다.
3) cnt가 가리키는 값이 max가 될 때까지 대기한다.

위와 같이 spin-lock을 이용한 배리어 동기에서는 루프 처리를 수행하므로 불필요하게 cpu 리소스를 점유한다.
그러므로 Pthreads의 조건 변수(Condvar) 또는 세마포어와 같은 보다 정교한 동기 메커니즘을 사용해 barrier를 구현해야 한다.

배리어 동기는 여러 실행 스레드의 실행을 동기화하기 위해 parallel 컴퓨팅에서 사용되는 기술이다.
barrier는 프로그램에서 모든 스레드가 중지되고 스레드가 진행되기 전에 다른 모든 스레드가 동일한 지점에 도달할 때까지 기다려야 하는 지점이다.

배리어 동기는 모든 스레드가 다음 단계로 이동하기 전에 특정 계산 단계를 완료했는지 확인하는 데 사용된다.
예를 들어 parallel sorting algorithm에서 각 스레드는 데이터의 일부를 독립적으로 정렬할 수 있으며 배리어를 사용하여
정렬된 부분을 다시 병합하기 전에 모든 스레드가 해당 부분을 완료했는지 확인할 수 있다.

배리어 동기는 다음의 parallel algorithms에 유용 할 수 있다.
parallel algorithms that require multiple stages of computation, where each stage depends on the results of the previous stage.
배리어 동기의 또 다른 예는 parallel matrix multiplication algorithm이다. 각 스레드에는 계산할 행렬의 일부가 할당될 수 있으며
배리어를 사용하여 곱셈의 다음 단계로 이동하기 전에 모든 스레드가 해당 부분을 완료했는지 확인할 수 있다.

다음은 parallel algorithm의 배리어 동기의 예이다.
```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn barrier(cnt: &AtomicUsize, max: &AtomicUsize) {
    cnt.fetch_add(1, Ordering::SeqCst);
    while cnt.load(Ordering::SeqCst) < max.load(Ordering::SeqCst) {}
}

fn main() {
    let mut handles = vec![];
    let num_threads = 5;

    let cnt = Arc::new(AtomicUsize::new(0));
    let max = Arc::new(AtomicUsize::new(num_threads));

    for i in 0..num_threads {
        let cnt = cnt.clone();
        let max = max.clone();

        let handle = thread::spawn(move || {
            println!("Thread {} started", i);
            // do some computation
            barrier(&cnt, &max);
            println!("Thread {} finished", i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```
이 예제에서는 num_threads 스레드의 벡터를 생성한다.
각 스레드는 약간의 계산(do something)을 수행한 다음 다른 스레드가 barrier 기능을 사용하여 따라잡을 때까지 기다린다.
즉, 모든 스레드가 barrier를 호출할 때까지 각 스레드들을 blocking한다. 모든 스레드가 barrier를 호출하면 그때 blocking을 해제하고
스케줄러가 각 스레드들을 순환하면서 finished를 print 한다.

cnt 및 max 변수는 clone 메서드를 사용하여 모든 스레드 간에 공유되어 각 스레드에 대한 새 인스턴스를 생성한다.
모든 스레드가 완료된 후 프로그램을 종료하기 전에 스레드가 완료될 때까지 대기하기 위해 각 스레드의 handle에서 join 메소드가 호출된다.

일반적으로 barrier 동기는 여러 스레드가 스레드 간의 조정 또는 통신이 필요한 문제에 대해 작업하는 모든 상황에서 유용하다.
모든 스레드가 조정된 방식으로 함께 작동하는지 확인하고 경합 상태 및 기타 동기화 문제를 방지하는 데 도움이 될 수 있다.

그러나 배리어 동기화는 오버헤드를 유발할 수 있으며 주의해서 사용하지 않으면 비효율적일 수 있다는 점에 유의해야 한다.
경우에 따라 해결하려는 문제의 세부 사항에 따라 lock 또는 세마포어와 같은 다른 동기화 기술을 사용하는 것이 더 나을 수 있다.

위의 경우, spin-lock을 통해 구현한 배리어 동기는 루프 처리를 수행하므로 불필요하게 cpu 리소스를 점유한다.

Rust에서는 Condvar를 사용하여 배리어 동기를 구현했다.
```rust
pub struct Barrier {
    lock: Mutex<BarrierState>,
    cvar: Condvar,
    num_threads: usize,
}

// The inner state of a double barrier
struct BarrierState {
    count: usize,
    generation_id: usize,
}

impl Barrier {
    pub fn new(n: usize) -> Barrier {
        Barrier {
            lock: Mutex::new(BarrierState { count: 0, generation_id: 0 }),
            cvar: Condvar::new(),
            num_threads: n,
        }
    }

    pub fn wait(&self) -> BarrierWaitResult {
        let mut lock = self.lock.lock().unwrap();
        let local_gen = lock.generation_id;
        lock.count += 1;
        if lock.count < self.num_threads {
            // We need a while loop to guard against spurious wakeups.
            // https://en.wikipedia.org/wiki/Spurious_wakeup
            while local_gen == lock.generation_id {
                lock = self.cvar.wait(lock).unwrap();
            }
            BarrierWaitResult(false)
        } else {
            lock.count = 0;
            lock.generation_id = lock.generation_id.wrapping_add(1);
            self.cvar.notify_all();
            BarrierWaitResult(true)
        }
    }
}
```
Rust의 표준 라이브러리는 루프를 돌릴 필요 없이 효율적인 barrier 동기를 허용하는 Barrier type을 제공한다.
Barrier 구현은 내부적으로 Condvar 및 Mutex를 사용하여 스레드를 조정한다.
```rust
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let mut handles = vec![];
    let num_threads = 5;

    let barrier = Arc::new(Barrier::new(num_threads));

    for i in 0..num_threads {
        let barrier = barrier.clone();

        let handle = thread::spawn(move || {
            println!("Thread {} started", i);
            // do some computation
            barrier.wait();
            println!("Thread {} finished", i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub struct BarrierWaitResult(bool);

impl BarrierWaitResult {
    pub fn is_leader(&self) -> bool {
        self.0
    }
}
```
이 구현에서 `Barrier`는 `num_threads` 개수로 생성되며, 이는 해제되기 전에 barrier에 도달해야 하는 스레드 수를 나타낸다.
각 스레드는 계산을 완료한 후 Barrier에서 wait()를 호출하며 모든 num_threads 스레드가 wait()를 호출할 때까지 스레드를 blocking한다.
모든 스레드가 wait()를 호출하면 barrier가 released되고 모든 스레드가 실행을 계속합니다.
`Barrier`의 구현은 `Condvar`를 사용하여 barrier에서 대기 중인 스레드를 blocking하고 깨우고, `Mutex`를 사용하여 barrier의 내부 상태를 보호한다.
이를 통해 spinning loop(spin-lock)나 busy-waiting 없이 효율적인 동기화가 가능하다.

요약하면 Rust의 `Barrier` struct는 `Condvar`와 `Mutex`의 조합을 사용하여 스레드를 조정하고 불필요한 spinning loop를 방지함으로써
spin-lock 기반 barrier 동기화에 대한 보다 효율적이고 확장 가능한 대안을 제공한다.

Barrier 사용의 한 가지 단점은 쓰레드가 서로가 Barrier에 도달할 때까지 기다려야 하므로 프로그램에 오버헤드를 추가할 수 있다는 것이다.
또한 Barrier를 생성할 때 전달하는 스레드 수를 미리 알고 있어야 하므로 스레드 수가 동적으로 변할 수 있는 상황에는 적합하지 않을 수 있다.

### Condvar: definition, how to use, and trade-offs
`Condvar`(조건 변수)는 스레드가 실행을 계속하기 전에 특정 조건이 참이 될 때까지 대기할 수 있도록 하는 동기 프리미티브이다.
일반적으로 `Mutex` 또는 `RwLock`과 함께 사용되며 여기서 `Mutex` 또는 `RwLock`은 공유 값에 대한 액세스를 조정하는 데 사용되고,
`Condvar`는 해당 값의 변경(공유 상태의 변경)을 기다리는데(notified) 사용된다.

```rust
pub struct Condvar {
    inner: sys::Condvar,
}

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar { inner: sys::Condvar::new() }
    }

    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> LockResult<MutexGuard<'a, T>> {
        let poisoned = unsafe {
            let lock = mutex::guard_lock(&guard);
            self.inner.wait(lock);
            mutex::guard_poison(&guard).get()
        };
        if poisoned { Err(PoisonError::new(guard)) } else { Ok(guard) }
    }

    pub fn wait_while<'a, T, F>(
        &self,
        mut guard: MutexGuard<'a, T>,
        mut condition: F,
    ) -> LockResult<MutexGuard<'a, T>>
        where
            F: FnMut(&mut T) -> bool,
    {
        while condition(&mut *guard) {
            guard = self.wait(guard)?;
        }
        Ok(guard)
    }

    pub fn wait_timeout<'a, T>(
        &self,
        guard: MutexGuard<'a, T>,
        dur: Duration,
    ) -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)> {
        let (poisoned, result) = unsafe {
            let lock = mutex::guard_lock(&guard);
            let success = self.inner.wait_timeout(lock, dur);
            (mutex::guard_poison(&guard).get(), WaitTimeoutResult(!success))
        };
        if poisoned { Err(PoisonError::new((guard, result))) } else { Ok((guard, result)) }
    }

    pub fn wait_timeout_while<'a, T, F>(
        &self,
        mut guard: MutexGuard<'a, T>,
        dur: Duration,
        mut condition: F,
    ) -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>
        where
            F: FnMut(&mut T) -> bool,
    {
        let start = Instant::now();
        loop {
            if !condition(&mut *guard) {
                return Ok((guard, WaitTimeoutResult(false)));
            }
            let timeout = match dur.checked_sub(start.elapsed()) {
                Some(timeout) => timeout,
                None => return Ok((guard, WaitTimeoutResult(true))),
            };
            guard = self.wait_timeout(guard, timeout)?.0;
        }
    }

    pub fn notify_one(&self) {
        self.inner.notify_one()
    }

    pub fn notify_all(&self) {
        self.inner.notify_all()
    }
}


pub struct SysCondvar {
    // The value of this atomic is simply incremented on every notification.
    // This is used by `.wait()` to not miss any notifications after
    // unlocking the mutex and before waiting for notifications.
    futex: AtomicU32,
}

impl SysCondvar {
    #[inline]
    pub const fn new() -> Self {
        Self { futex: AtomicU32::new(0) }
    }

    // All the memory orderings here are `Relaxed`,
    // because synchronization is done by unlocking and locking the mutex.

    pub fn notify_one(&self) {
        self.futex.fetch_add(1, Relaxed);
        futex_wake(&self.futex);
    }

    pub fn notify_all(&self) {
        self.futex.fetch_add(1, Relaxed);
        futex_wake_all(&self.futex);
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        self.wait_optional_timeout(mutex, None);
    }

    pub unsafe fn wait_timeout(&self, mutex: &Mutex, timeout: Duration) -> bool {
        self.wait_optional_timeout(mutex, Some(timeout))
    }

    unsafe fn wait_optional_timeout(&self, mutex: &Mutex, timeout: Option<Duration>) -> bool {
        // Examine the notification counter _before_ we unlock the mutex.
        let futex_value = self.futex.load(Relaxed);

        // Unlock the mutex before going to sleep.
        mutex.unlock();

        // Wait, but only if there hasn't been any
        // notification since we unlocked the mutex.
        let r = futex_wait(&self.futex, futex_value, timeout);

        // Lock the mutex again.
        mutex.lock();

        r
    }
}
```
스레드가 Condvar에서 wait 메소드를 호출하면 연관된 Mutex 또는 RwLock을 원자적으로 해제하고 스레드를 blocking하며 조건이 충족되었다는 알림(notified 메서드)을 기다린다.
wait 메서드는 연결된 `Mutex` 또는`RwLock`을 유지하는 동안 호출해야 한다. 그렇지 않으면 panic!이 발생한다.
다른 스레드가 공유 리소스를 수정하고 notify 메서드를 호출하여 Condvar를 깨우면, 대기 중인 스레드가 깨어나 관련 `Mutex` 또는 `RwLock`을 다시 획득한다.

`Condvar`를 사용하려면 먼저 new() 메서드를 사용하여 새 Condvar를 만든 다음 wait() 메서드를 사용하여 조건이 true가 될 때까지 기다린다.
Condvar에서 대기 중인 스레드를 깨우기 위해 notify_one() 또는 notify_all() 메서드를 사용한다.

다음은 간단한 blocking Queue을 구현하기 위해 `Mutex`와 함께 `Condvar`를 사용하는 예이다.
```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct Queue<T> {
    data: Mutex<Vec<T>>,
    not_empty: Condvar,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            data: Mutex::new(Vec::new()),
            not_empty: Condvar::new(),
        }
    }

    fn push(&self, item: T) {
        let mut data = self.data.lock().unwrap();
        data.push(item);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> T {
        let mut data = self.data.lock().unwrap();
        while data.is_empty() {
            data = self.not_empty.wait(data).unwrap();
        }
        data.remove(0)
    }
}

fn main() {
    let queue = Arc::new(Queue::new());

    let producer = {
        let queue = queue.clone();
        thread::spawn(move || {
            for i in 0..10 {
                queue.push(i);
            }
        })
    };

    let consumer = {
        let queue = queue.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let item = queue.pop();
                println!("Consumer got {}", item);
            }
        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();
}
```
이 예에서 Queue struct에는 기본 데이터 벡터를 보호하는 Mutex와 Queue가 비어 있지 않을 때 신호를 보내는 데 사용되는 `Condvar`가 포함되어 있다.
push 메소드는 단순히 데이터 벡터에 항목을 추가하고 하나의 대기 중인 스레드에 알리는 반면,
pop 메소드는 데이터 벡터가 비어 있지 않을 때까지 blocking한 다음 첫 번째 항목을 제거하고 반환한다.

pop 메서드가 not_empty 조건이 충족되기를 기다리기 위해 단순한 if 문이 아니라 while 루프를 사용하는 방법에 유의해야한다.
이는 `Condvar`의 신호 이외의 이유로 스레드가 깨어날 수 있는 [가짜 깨우기 또는 의사 각성: Spurious wakeup](https://en.wikipedia.org/wiki/Spurious_wakeup) 을 처리하는 데 필요하다.

주의해야할 다른 중요한 점은 `Condvar`가 대기 중인 스레드가 깨어나는 순서에 대한 어떠한 보장도 제공하지 않는다는 것이다(not deterministic).
여러 스레드가 한 번에 깨어나거나 스레드가 차단된 순서와 다른 순서로 깨어날 수 있다.
(Executor와 Eventloop의 Queue의 오더링 순서를 정하더라도, 도움이 될 순 있지만 보장하지는 못한다.
러스트에서는 스레드의 스케줄링은 궁극적으로 OS 스케줄러의 권한이기 때문이다.)

따라서 특정 스레드 wakeup 순서에 의존하지 않는 방식으로 동기화 논리를 설계하는 것이 중요하다.
추가적인 오버헤드에 비해 효과는 미미하기 때문이다.
그렇기 때문에 wakeup 순서의 변동을 처리할 수 있을 만큼 충분히 유연하고 스레드 스케줄링에 대한 특정 가정에 의존하지 않는 동기화 메커니즘을 설계하는 것이 가장 좋다.

Condvar 사용의 한 가지 단점은 스레드가 실행을 계속하기 전에 조건이 참이 될 때까지 기다려야 하므로 프로그램에 오버헤드를 추가한다는 것이다.
또한 Condvar를 올바르게 사용하지 않으면 잠재적인 경합 상태가 발생할 수 있으므로 사용 시 주의해야 한다.

### mpsc: definition, how to use, and trade-offs

### Mutex: definition, how to use, and trade-offs

### Once: definition, how to use, and trade-offs

### RwLock: definition, how to use, and trade-offs

## 4. Introduction to crossbeam

## 5. Crossbeam channels

## 6. Crossbeam atomic types

## 7. Work stealing with crossbeam and Rayon

## 8. Conclusion