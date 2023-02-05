use std::cell::UnsafeCell;

use cell::{TokenWith, TokenBuilder, first};

// While the main crate provides one-to-many (token-to-cell) memory primitives, this example
// provides many-to-one primitives. Namely, we will use many `TokenWith<usize>`s to address a
// single Arena. 
//
// Generally these are the only two useful memory management models, since "one-to-one" describes
// how Rust already works, and "many-to-many" will almost always allow for unsafe aliased
// mutability.

/// A push-only arena. If an index exists, the object is guarenteed to still exist.
pub struct Arena<T, const ID: usize> {
    inner: UnsafeCell<Vec<T>>,
}

type Index<const ID: usize> = TokenWith<usize, ID>;

// A single TokenBuilder can be exchanged
impl<T, U, const ID: usize> From<TokenWith<U, ID>> for Arena<T, ID> {
    fn from(_: TokenWith<U, ID>) -> Self {
        Arena {
            inner: UnsafeCell::new(Vec::new())
        }
    }
}

impl<T, const ID: usize> Arena<T, ID> {
    // Pushing an item may invalidate old references by causing a reallocation, so we still need a &mut self
    fn push(&mut self, item: T) -> Index<ID> {
        let inner = self.inner.get_mut();
        let pos = inner.len();

        inner.push(item);

        // Safety
        //
        // Normally it is unsafe to create multiple `TokenWith`s with the same ID. However, since
        // each `Index` has a unique `pos` and is guarenteed to reference different data, this is
        // safe.
        unsafe {Index::new(pos)}
    }

    fn get(&self, index: &Index<ID>) -> &T {
        let inner: &Vec<T> = unsafe {self.inner.get().as_ref().unwrap()};

        unsafe {inner.get_unchecked(*index.get())}
    }

    fn get_mut(&self, index: &mut Index<ID>) -> &mut T {
        let inner: &mut Vec<T> = unsafe {self.inner.get().as_mut().unwrap()};

        unsafe {inner.get_unchecked_mut(*index.get())}
    }
}

fn main() {
    let (t1, next) = first().unwrap().token();
    let (t2, _) = next.token();

    let mut chars = Arena::from(t1);
    let mut nums = Arena::from(t2);

    let mut a = chars.push('a');
    let b = chars.push('b');
    let c = chars.push('c');

    let one = nums.push(1u32);
    let two = nums.push(2);

    // This compiles:
    println!("{}", chars.get(&a));
    // This doesn't:
    // println!("{}", chars.get(&one));

    // This compiles:
    *chars.get_mut(&mut a) = 'ä';
    // This doesn't. With this item, individual items can be declared as mutable or immutable:
    // *chars.get_mut(&mut b) = 'ß';
    
    // A borrowing pattern not possible with a normal Vec<T>:
    let a = chars.get_mut(&mut a);
    let b = chars.get(&b);

    drop(b);
    drop(a);

    // Under the hood, an `Index` is just a usize. The following:
        // chars.get(&c);
        // nums.get(&c);
    // is equivalent to:
        // chars.get(2);
        // nums.get(2);
    // However, the `Index` model used here can prove, at compile time, that the following should work: 
        chars.get(&c);
    // While this won't:
        // nums.get(&c);
    
    // and will return the reference without having to check at runtime. 
}
