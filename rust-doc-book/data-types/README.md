# Common Programming Concepts

## Data Types

- Rust is a statically typed language (~cries in spanish~)
- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
- Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.
- Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
- A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- In addition to destructuring through pattern matching, we can access a tuple element directly by using a period (.) followed by the index of the value we want to access.
- Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type.
- Arrays are useful when you want your data allocated on the stack rather than the heap.