use std::marker::PhantomData;

use crate::tokens::*;

pub struct TokenBuilder<const ID: usize>(
    PhantomData<()>, //prevents users outside the API from creating an instance of TokenBuilder
);

impl<const ID: usize> TokenBuilder<ID> {
    /// Creates a new `TokenBuilder` with this ID. 
    ///
    /// # Safety
    /// Since a `TokenBuilder` can be converted into a [Token](crate::tokens::Token) and `Token`s
    /// represent mutable or immutable access to data, creating multiple `TokenBuilder`s with the
    /// same ID could mutably alias data.
    pub const unsafe fn new() -> Self {
        Self(PhantomData)
    }

    /// Convert this TokenBuilder into a [Token](crate::tokens::Token) as well as the next
    /// TokenBuilder.
    pub const fn token(self) -> (Token<ID>, TokenBuilder<{ID + 1}>) {
        unsafe {(Token::new(()),
                 TokenBuilder::new())}
    }

    /// More generic version of [Self::token()]
    pub const fn token_with<U>(self, u: U) -> (TokenWith<U, ID>, TokenBuilder<{ID + 1}>) {
        unsafe {(TokenWith::new(u),
                 TokenBuilder::new())}
    }
}
