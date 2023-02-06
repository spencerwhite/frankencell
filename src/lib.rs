#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

//! # Purpose
//! This crate is another attempt at the `ghost-cell` / `qcell` saga of cell crates. This provides
//! an alternative to [std::cell::RefCell] that can allow interior mutability checked at compile
//! time, rather than runtime. Because Rust doesn't allow for unlimited creation of invariant
//! generics, this always comes with a rather large complexity cost. Whereas `ghost-cell` uses
//! invariant lifetimes and `qcell` can use either invariant lifetimes or newtypes, this crate
//! instead uses const generic `usize`s.
//!
//! # Pros
//! As with other `*cell` crates, this model provides interior mutability checked at compile time.
//! Unlike `ghost-cell`'s model,  this crate doesn't require all of your borrows to exist in a
//! closure and, unlike `qcell::TCell`, this crate allows for more than three simultaneous borrows.
//!
//! # Cons
//!
//! First, any item that contains a `Cell` or `Token` must be generic over `const ID: usize`. You
//! may choose to get rid of this if you are **sure** that, for example, a certain instance of a
//! struct will always have `ID = 3`
//!
//! Second, in order to provide a safe API, `Token`s must be built using a `TokenBuilder` struct
//! that ensures `ID`s are unique. There may in the future be a way around this, but don't hold
//! your breath!
//!
//! # Example
//! ```compile_fail
//! use frankencell::*;
//! let (token1, next) = first().unwrap().token();
//! let (token2, _) = next.token();
//!
//! let a = Cell::new('a');
//! let b = Cell::new('b');
//!
//! println!("{}", a.borrow(&token1));
//! println!("{}", b.borrow(&token2));
//!
//! // The following fails to compile:
//! println!("{}", a.borrow(&token2));
//! println!("{}", b.borrow(&token1));
//! ```
//!
//! # Future improvements
//! Currently because of how `const` works, it is impossible for a `const fn` to return different
//! values on different calls. In order to generate unique IDs however, the following would have to
//! be possible:
//!
//! ```compile_fail
//! const fn inc() -> usize {
//!     // Insert magic here
//! }
//!
//! #[test]
//! fn test_inc() {
//!     assert_eq!(inc(), 0);
//!     assert_eq!(inc(), 1);
//!     assert_eq!(inc(), 2);
//!
//!     // user-facing API is now significantly better
//!     let token_3: Token<3> = Token::next();
//!     let token_4: Token<4> = Token::next();
//! }
//! ```
//!
//! This *may* become possible when/if heap allocations are allowed in `const` contexts, but even
//! then this pattern will likely never be officially endorsed by the Rust compiler.
//!
//! It may also be possible with macros when/if macros are allowed to keep a local state
//! (rust-lang/rust issue 44034).
//!
//! # Should I use this? 
//! Probably not. At the moment this is really more of a proof-of-concept. There's still a lot of
//! work that needs to go into the compiler and, even then, this may not be a viable solution.
//!
//! If you're simply looking for something that's more ergonomic than `ghost-cell` and `qcell`, the
//! `cell-family` crate seems to have a good approach.

mod builder;
pub mod cells;
pub mod tokens;

use std::sync::Once;

pub use crate::builder::TokenBuilder;
pub use crate::cells::*;
pub use crate::tokens::*;

static FIRST: Once = Once::new();

/// Entry-point into the API that allows for safe creation of unique `Token`s.
///
/// ```rust
/// # use frankencell::first;
/// assert!(first().is_some());
/// assert!(first().is_none());
/// ```
// Implementation stolen lovingly from LegionMammal978
pub fn first() -> Option<TokenBuilder<0>> {
    let mut builder = None;
    FIRST.call_once(|| {
        builder = Some(unsafe { TokenBuilder::new() });
    });

    builder
}

/// Slightly more convenient way to initialize multiple tokens. Note that this currently only
/// supports the basic [Token](crate::tokens::Token) type, and a [TokenWith] must be built manually
/// 
/// # Example
/// init_tokens! { after first().unwrap();
///     t1, t2, t3 then next
/// }
///
/// let (with_usize, next) = next.token_with(0usize);
#[macro_export]
macro_rules! init_tokens {
    (after $first:expr; $($name:ident),* then $next:ident) => {
        let $next = $first;
        $(
            let ($name, $next) = $next.token();
        )*
    }
}

#[test]
fn init_tokens_test() {
    use crate::{TokenBuilder, init_tokens, Cell};

    let first = unsafe {TokenBuilder::<0>::new()};
    init_tokens! { after first;
        t1,t2,t3 then _next
    };

    let cell1 = Cell::new(1);
    let cell2 = Cell::new(2);
    let cell3 = Cell::new(3);

    println!("{}", cell1.borrow(&t1));
    println!("{}", cell2.borrow(&t2));
    println!("{}", cell3.borrow(&t3));
}

#[test]
fn test_first() {
    use crate::first;

    assert!(first().is_some());
    assert!(first().is_none());
}
