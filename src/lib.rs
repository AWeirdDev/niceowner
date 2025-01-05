//! # niceowner
//! A stupid library that allows you to own a value, even if it comes from a reference. No cloning.
//!
//! ```rust
//! use niceowner::NiceOwner;
//!
//! pub struct Dog {
//!     name: String,
//! }
//!
//! fn lend_to_amy(no: &mut NiceOwner<Dog>) {
//!     // The dog is essentially *borrowed*.
//!     // But now Amy is going to own it... somehow.
//!     let mut dog = no.own().unwrap();
//!     dog.name = String::from("Amy's dog");
//!
//!     // Amy remembers to return the dog back to the owner.
//!     no.return_value(dog);
//! }
//!
//! fn main() {
//!     let dog = Dog {
//!         name: String::from("Dee'O G"),
//!     };
//!     let mut owned_dog = NiceOwner::new(dog);
//!
//!     // The owner is going abroad, so let's ask Amy to take care of the dog.
//!     lend_to_amy(&mut owned_dog);
//!
//!     // The owner sees the nametag of the dog.
//!     println!("The name of the dog is {:?}", owned_dog.name); // "Amy's dog"
//!
//!     // The dog now obeys Amy! :O
//! }

//! ```

use std::{
    mem,
    ops::{Deref, DerefMut},
};

use anyhow::{anyhow, Result};

/// An owner who lends the value of their own and allows you to not only borrow it, but also own it.
pub struct NiceOwner<K> {
    data: Option<K>,
}

impl<K> NiceOwner<K> {
    /// Creates a new nice owner.
    ///
    /// ```rust
    /// use niceowner::NiceOwner;
    ///
    /// let dog = String::from("dog");
    /// let mut owned_dog = NiceOwner::new(dog);
    ///
    /// fn pet(no: &mut NiceOwner<String>) {
    ///     let mut dog = no.own().unwrap();
    ///     println!("My dog is called {}", dog);
    ///     dog.push_str("!");
    ///
    ///     // Remember to return the dog back to the owner.
    ///     no.return_value(dog);
    /// }
    ///
    /// pet(&mut owned_dog);
    /// ```
    pub fn new(data: K) -> Self {
        Self { data: Some(data) }
    }

    /// Owns the value, even if this comes from a reference.
    /// If the value is already owned somewhere else and not yet returned, returns an error.
    pub fn own(&mut self) -> Result<K> {
        if let Some(s) = mem::replace(&mut self.data, None) {
            Ok(s)
        } else {
            Err(anyhow!(
                "Expected NiceOwner to own a value, but nothing was returned back."
            ))
        }
    }

    /// Return the value back to the owner.
    pub fn return_value(&mut self, value: K) {
        self.data = Some(value);
    }

    /// Temporarily own the value and do something with it.
    ///
    /// ```rust
    /// use niceowner::NiceOwner;
    ///
    /// let dog = String::from("dog");
    /// let mut owned_dog = NiceOwner::new(dog);
    ///
    /// owned_dog.with(|mut s| {
    ///     s.clear();
    ///     s // **essential**: return the value back
    /// });
    ///
    /// assert_eq!(owned_dog.own().unwrap(), ""); // dog gon
    /// ```
    pub fn with(&mut self, f: fn(K) -> K) {
        let owned = self.own().unwrap();
        self.return_value(f(owned));
    }
}

impl<T> Deref for NiceOwner<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().unwrap()
    }
}

impl<K> DerefMut for NiceOwner<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut().unwrap()
    }
}
