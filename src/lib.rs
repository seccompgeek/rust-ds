pub trait Collection {
    // Adds a new key to the collection.
    // If the key already exists, then simply return false.
    // Otherwise, add it to the collection and return true.
    fn add(&mut self, key: i32) -> bool;
    // Removes a key from the collection.
    // If the key exists, remove it and return false.
    // Else, return false.
    fn remove(&mut self, key: i32) -> bool;
    // Traverses the collection to find the given key.
    // Returns true if the key is found.
    // Else returns false.
    fn contains(&self, key: i32) -> bool;
}

mod binary_tree;
mod dll;
mod sll;
