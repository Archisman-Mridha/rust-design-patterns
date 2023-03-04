# Use borrowed types for arguments

First we need to know what is `deref coercion`.

## Deref Coercion

>In Rust, deref coercion is a mechanism where the compiler **automatically converts a reference to a type into a reference to another type that it points to**.

Using a `target of a deref coercion` means that we specify the type that we want the reference to be converted to, when we pass the reference as an argument to a function.

For example - We have a function that takes a reference to a string as an argument. We can use a target of a deref coercion to allow the function to accept `references` to both **String** and **&str** types. The compiler **automatically converts the String reference into &str reference**.

> The benefit of using a target of deref coercion here, is that we can avoid writing separate versions of the function for each type.

Here is the code -
```rust
fn printForDebugging(content: &str) {
    println!("{}", content);}

fn main( ) {
    let content= String::from("passing reference to `String` type");
    printForDebugging(content);

    let content= "passing reference to `&str` type";
    printForDebugging(&content);

    /*
        deref coercion has a major advantage over here.
        The `&String` type has 2 layers of indirection. Due to deref coercion, the argument type gets converted to
        `&str` which has 1 layer of indirection here.
    */
    let content= String::from("passing reference to `&String` type");
    printForDebugging(&content);
}
```
In the above example, we can see, how in some cases we can avoid layers of indirection by using a target of deref coercion.