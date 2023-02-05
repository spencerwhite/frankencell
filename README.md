# Purpose
This crate is another attempt at the `ghost-cell` / `qcell` saga of cell crates. This provides
an alternative to `std::cell::RefCell` that can allow interior mutability checked at compile
time, rather than runtime. Because Rust doesn't allow for unlimited creation of invariant
generics, this always comes with a rather large complexity cost. Whereas `ghost-cell` uses
invariant lifetimes and `qcell` can use either invariant lifetimes or newtypes, this crate
instead uses const generic `usize`s.

# Pros
As with other `*cell` crates, this model provides interior mutability checked at compile time.
Unlike `ghost-cell`'s model,  this crate doesn't require all of your borrows to exist in a
closure and, unlike `qcell::TCell`, this crate allows for more than three simultaneous borrows.

# Cons

First, any item that contains a `Cell` or `Token` must be generic over `const ID: usize`. You
may choose to get rid of this if you are **sure** that, for example, a certain instance of a
struct will always have `ID = 3`

Second, in order to provide a safe API, `Token`s must be built using a `TokenBuilder` struct
that ensures `ID`s are unique. There may in the future be a way around this, but don't hold
your breath!

# Example
```
use cell::*;
let (token1, next) = first().unwrap().token();
let (token2, _) = next.token();

let a = token1.cell('a');
let b = token2.cell('b');

println!("{}", a.borrow(&token1));
println!("{}", b.borrow(&token2));

// The following fail to compile:
println!("{}", a.borrow(&token2));
println!("{}", b.borrow(&token1));
```

# Future improvements
Currently because of how `const` works, it is impossible for a `const fn` to return different
values on different calls. In order to generate unique IDs however, the following would have to
be possible:

```rust
const fn inc() -> usize {
    // Insert magic here
}

#[test]
fn test_inc() {
    assert_eq!(inc(), 0);
    assert_eq!(inc(), 1);
    assert_eq!(inc(), 2);

    // user-facing API is now significantly better
    let token_3: Token<3> = Token::next();
    let token_4: Token<4> = Token::next();
}
```

This *may* become possible when/if heap allocations are allowed in `const` contexts, but even
then this pattern will likely never be officially endorsed by the Rust compiler.

# Should I use this? 
Probably not. At the moment this is really more of a proof-of-concept. There's still a lot of
work that needs to go into the compiler and, even then, this may not be a viable solution.

If you're simply looking for something that's more ergonomic than `ghost-cell` and `qcell`, the
`cell-family` crate seems to have a good approach.
