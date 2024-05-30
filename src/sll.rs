//! A singly linked list in Rust.
//! Implement the collections trait for the list and
//! write unit tests for it.
//!

use crate::Collection;

pub struct SLL {}
type Key = i32;

impl SLL {
    // creates and returns an empty singly linked list
    pub fn new() -> Self {}
}

impl Collection for SLL {
    // TODO: implement collection functions for the SLL struct.
}

#[cfg(test)]
mod tests {
    // TODO: write unit tests for the SLL.
}
