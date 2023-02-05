use crate::cells::Cell;

/// A generic token that can store any data. See examples/arena.rs for an example of how this could
/// be used.
pub struct TokenWith<T, const ID: usize>(T);

impl<T, const ID: usize> TokenWith<T, ID> {
    /// Creates a new token with this ID.
    ///
    /// # Safety:
    /// Because tokens represent access (mutable or immutable) to a memory location, creating >1
    /// tokens is equivalent to creating >1 mutable references to data.
    pub const unsafe fn new(t: T) -> Self {
        Self(t)
    }

    pub const fn cell(&self, t: T) -> Cell<T, ID> {
        Cell::new(t)
    }

    /// Getters and setters are necessary so an API user can't manually create a Token and must
    /// instead use [Self::new()]
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Getters and setters are necessary so an API user can't manually create a Token and must
    /// instead use [Self::new()]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}


/// A Token that represents access to one or more memory locations, each containing the same or
/// different data types.
///
/// Currently, this crate only provides [Cell](crate::cells::Cell), but you may create your own
/// ownership primitives. See examples/arena.rs for an example.
pub type Token<const ID: usize> = TokenWith<(), ID>;
