# CHAPTER 3 - COMMON PROGRAMMING CONCEPTS

## 3.1 Variables and Mutability

1. As mentioned before, variables are _immutable by default_. To make a variable _mutable_, you'd use the `mut` keyword in the variable's declaration.

2. Rust _does also_ support **constants** and the keyword `const`. This may easily be confused with _immutability_ but there are important distinctions.

   a. As you know, `const` in C is really meant to be "read-only", not that the variable is _actually_ constant (i.e., immutable).

   b. In Rust, `const` variables are _always_ immutable and **must** have an explicit **type annotation**, like: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`.

   c. Constant variables are declared using _just_ the `const` keyword instead of the `let` keyword.

   d. Constant variables can be declared in _any_ scope, including the global scope.

   e. Constant variables _must_ be assigned (bound to) a constant expression - i.e., one that can be fully computed at compile-time and not rely on any run-time computation (_just like in C_).

   f. Similar to C, the Rust compiler is able to resolve certain operations / expressions at compile-time. So you can write out `60 * 60 * 3` for the number of seconds in 3 hours instead of `10800`.

   g. Constants have _static duration_, scoped to whatever scoped they were declared in (_just like C_).

3. As you saw in the guessing game of Chapter 2, Rust supports **shadowing**.

   a. A new variable overshadows a previous one _until_ it is overshadowed itself or _the scope of this new variable ends_.

   b. Shadowing can make it seem like you are mutating a variable but in reality, you are not. For example:

```
// -- some code --
let x = 5;  // immutable variable named x bound to the value of 5.
let x = x + 1; // new variable, also named x, that shadows the previous value of x. This new variable will be bound to the value of 6.
{
   let x = x * 2; // new variable again, within an _inner_ scope, holding the value of 12.
   println!("The value of x in this inner scope is: {x}");  // will print 12
}  // scope here ends so the second shadow variable will become out-of-scope and the name "x" will return to the first shadow variable

println!("The value of x is: {x}"); // will print 6
```