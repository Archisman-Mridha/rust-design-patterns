# Constructor

Rust doesn't provide the constructor feature by default. We can use an `associated function named new`, to **create a new object** for a given struct. This associated fuction, will not take in the reference to self as an argument.

Here is an example -
```rust
struct Circle {
    radius: f64
}

impl Circle {

    // acts as the constructor for the `Circle` struct
    pub fn new(radius: f64) -> Self {
        return Circle {
            radius
        };
    }
}
```

## Default constructors
Rust supports `default constructors` using the `Default trait`.

The struct can implement the Default trait like this -
```rust
struct Circle {
    radius: f64
}

impl Default for Circle {
    fn default( ) -> Self {
        return Self {
            radius: 10.0
        };
    }
}
```

Or, if all the stuct fields implement Default, then the struct can also Derive the Default trait.
Here is the example -
```rust
#[derive(Default)]
struct Circle {
    radius: f64
}
```

> The advantage of implementing or deriving Default is that your type can now be used where a Default implementation is required