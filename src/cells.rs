use std::{cell::UnsafeCell, fmt::Debug, any::Any};

use crate::tokens::TokenWith;

/// A value whose ownership is tied to a [TokenWith], usually a
/// [Token](crate::tokens::Token). A `Cell<T>` is treated and stored exactly like a T without
/// indirection. For example, accessing a value in a `Vec<Cell<usize>>` is as efficient as accessing a
/// value in a `Vec<usize>`.
///
/// Can take any value regardless of `T` in [TokenWith]. However, both `ID`s must be the
/// same.
///
/// - A `&T` can be created from: 
///     - `&self` + `&Token`
/// - A `&mut T` can be created from:
///     - `&self` + `&mut Token`
///     - `&mut self` (see [Cell::get_mut] for details)

//TODO: More cell types. Currently, Token and Cell have a one-to-many relationship, but it may be
//useful in the future to create a token/cell with a many-to-one relationship such as in
//exaples/arena.rs
#[derive(Default)]
#[repr(transparent)]
pub struct Cell<T, const ID: usize> {
    pub(crate) inner: UnsafeCell<T>,
}

//TODO: Figure out whether these are safe, then write a blurb about why they're safe.
unsafe impl<T: Send, const ID: usize> Send for Cell<T, ID> {}
unsafe impl<T: Send + Sync, const ID: usize> Sync for Cell<T, ID> {}

/// Very simple debugging function; if you want the inner value, instead use 
/// ```println!("{:?}", cell.get(&token))```;
impl<T: Debug + Any, const ID: usize> Debug for Cell<T, ID> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cell<{}, {}>", std::any::type_name::<T>(), ID)
    }
}

impl<T, const ID: usize> Cell<T, ID> {
    /// Creates a new cell that can only be accessed by a token with the same ID.
    ///
    /// # Example
    /// ```Rust
    /// let t = first().unwrap().token();
    ///
    /// let a = Cell::new('a');
    ///
    /// // Rust's type inference attaches `a` to `t`
    /// a.borrow(&t);
    /// ```
    pub const fn new(t: T) -> Self {
        Self {
            inner: UnsafeCell::new(t),
        }
    }

    /// Reinterpret a `&mut T` into a `&mut Self`. This may be useful if you only need to
    /// temporarily attach a value to a token, for example in a closure.
    pub fn from_mut(m: &mut T) -> &mut Self {
        unsafe {std::mem::transmute(m)}
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    pub fn as_ptr(&self) -> *const T {
        self.inner.get()
    }

    /// Reinterpret a `&self` as a `&T`
    ///
    /// # Safety
    /// Since `Cell` depends on the interior mutability pattern, using this function without
    /// passing the appropriate token to prove ownership could result in aliased mutability.
    ///
    /// ```
    /// let mut token = first().unwrap().token();
    /// let cell = Cell::new(String::from("Hello"));
    ///
    /// let cell_mut = cell.borrow_mut(&mut token);
    /// let cell_ref = unsafe {cell.get()};
    ///
    /// drop(cell_mut);
    /// drop(cell_ref);
    /// ```
    pub unsafe fn get(&self) -> &T {
        unsafe {std::mem::transmute(self)}
    }

    /// Reinterpret a `&mut self` as a `&mut T`. 
    pub fn get_mut(&mut self) -> &mut T {
        unsafe {std::mem::transmute(self)}
    }

    /// Use a `&Token` to prove no `&mut T` currently exists and recieve a `&T` in return
    /// 
    /// # Example
    /// ```
    /// let mut token = first().unwrap().token();
    /// let cell = Cell::new(String::from("ABC"));
    /// let cell_cell = Cell::new(&cell);
    ///
    /// println!("{}", cell_cell.borrow(&token).borrow(&token));
    /// 
    /// ```
    pub fn borrow<U>(&self, _: &TokenWith<U, ID>) -> &T {
        unsafe {self.inner.get().as_ref().unwrap_unchecked()}
    }

    /// Use a `&mut Token` to prove no `&mut T` or `&T` currently exist and recieve a `&mut T` in return.
    ///
    /// # Example
    /// ```
    /// let mut token = first().unwrap().token();
    /// let cell = Cell::new(String::from("Hello Worl"));
    ///
    /// let safe_ref = &cell;
    ///
    /// // whoops, typo!
    /// cell.borrow_mut(&mut token).push('d');
    ///
    /// println!("{}", safe_ref.borrow(&token));
    /// ```
    pub fn borrow_mut<U>(&self, _: &mut TokenWith<U, ID>) -> &mut T {
        unsafe {self.inner.get().as_mut().unwrap_unchecked()}
    }
}
