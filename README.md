# niceowner

A stupid library that allows you to own a value, even if it comes from a reference. No cloning.

You can use this with an `Rc` or `Arc`, if you're interested in cloning the reference of `NiceOwner`.

## Usage

```rust
use niceowner::NiceOwner;

struct Dog {
    name: String,
}

fn lend_to_amy(no: &mut NiceOwner<Dog>) {
    // The dog is essentially *borrowed*.
    // But now Amy is going to own it... somehow.
    let mut dog = no.own().unwrap();
    dog.name = String::from("Amy's dog");

    // Amy remembers to return the dog back to the owner.
    no.return_value(dog);
}

fn main() {
    let dog = Dog { name: String::from("Dee'O G") };
    let mut owned_dog = NiceOwner::new(dog);

    // The owner is going abroad, so let's ask Amy to take care of the dog.
    lend_to_amy(&mut owned_dog);

    // The owner sees the nametag of the dog.
    println!("The name of the dog is {:?}", owned_dog.name); // "Amy's dog"

    // The dog now obeys Amy! :O
}
```

***

(c) 2025 AWeirdDev
