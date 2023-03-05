# Collections as smart pointers

First, we need to know what are smart pointers.
## Smart Pointers
A smart pointer in Rust is a type of **data structure that behaves like a pointer but also has additional capabilities** that help manage memory safely and efficiently. There are 2 types of smart pointers - `Box<T>` and `Rc<T>`.

- `Box<T>` - We use this to allocate heap memory. Rust guarantees that the **memory is deallocated automatically when the Box goes out of scope**.
- `Rc<T>` - It is a smart pointer that allows multiple ownership of data (heap allocated). It keeps track of how many references to a particular piece of data exists at a given time. It keeps track of how many references to a particular piece of data exist at any given time, and it only deallocates the data when all the references have gone out of scope.