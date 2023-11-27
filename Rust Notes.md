# INTRODUCTION
1. Rust tries to stand out compared to other languages by having the Rust compiler refuse to compile code that has certain elusive bugs, **including** concurrency bugs.

2. Rust also brings _contemporary_ programming tools to the system programming world, such as:
    a. `cargo`, the included _dependency manager_ and _build tool_ (wowza!), which makes adding, compiling, and managing dependencies painless and _consistent_ across the Rust ecosystem.
    b. The `rustfmt` tool that can enforce a coding style if invoked.
    c. The **Rust Language Server** provides IDE-integration for code completion and inline error messages.

3. From these first two points, Rust developers can more productively work in a team environment while system programming.

4. Companies use Rust in production for a variety of tasks, including:
    a. command line tools
    b. web services
    c. DevOps tooling
    d. embedded devices
    e. audio and video analysis and transcoding
    f. cryptocurrencies
    g. bioinformatics
    h. search engines
    i. Internet of Things applications
    j. machine learning
    k. web browsers (such as major parts of the FireFox web browser)

5. The Rust mascot is a crab named **Ferris**. For code errors, Ferris has the following expressions:
    a. claws down, question mark to the left == code does not compile!
    b. both claws up == this code panics!
    c. Ferris' right claw up - code does not produce the desired behavior


# CHAPTER 1 - GETTING STARTED

## INSTALLATION

1. It looks like Rust makes use of the linker that comes with a C development environment, which could be why you need a C build environment with Rust.

2. `rustup` is the command line tool for managing Rust versions and the associated tools.

3. To check whether you have installed Rust correctly and what version you have: `rustc --version`. You'll in return get (if successfully installed) `rustc x.y.z (abcabcabc yyyy-mm-dd)` (i.e., the commit hash and release date!).

4. To look at the (thorough) documentation that comes with Rust, you can start with `rustup doc`, and if you want to open **The Book**, `rustup doc --book`.

## HELLO WORLD!

1. All Rust source files have the `.rs` extension.

2. To pass a single source file to the Rust compiler, you can just do `rustc <source_name>.rs` and, if successful, will produce an executable with the same name as that source file _along with_ a `.pdb` file on Windows systems (which is a file that contains debug information).

3. Looks like functions declaration/definition blocks start with `fn <function_name>(<arg_list>)`.

4. `main` is the function that is run first with any executable Rust program.

5. Rust style is to indent with _four spaces_, **not** a tab.

6. `println!` calls a Rust **macro**. If it had been a function call instead, there would be no `!` character. Macros will be discussed more in Chapter 19 (so far!), so for now, just assume that the use of an `!` in a function-call-like statement invokes a macro.

7. `"Hello, world!"` is a string.

8. Rust statements almost always end with `;`.

## Hello, Cargo!

1. **Cargo** is Rust's build system and package manager. It is one hell of a power tool.

2. Pretty much any Rust repo makes use of Cargo.

3. To create a new project using Cargo: `cargo new <project_name>`. This command

    a. creates a new directory with the name of the project

    b. generates a `Cargo.toml` file (the equivalent of a `package.json`) which is a file written in **TOML** - Tom's Obvious, Minimal Language - that is how you specify dependencies, your package version, and so on.

    c. a `src/` directory with a `main.rs` file inside. NOTE! Cargo expects **all** your source files to be in this directory.
    
    d. initializes a Git directory (if there wasn't one already contained) along with a `.gitignore` file. Note you can change what Version Control System (VCS) is used by using the `--vcs=<version_control_system>` flag.

4. The `Cargo.toml` file contains sections that are delineated by the section header, which is some name in square brackets, like the `[package]` section present in every `Cargo.toml` file which includes package information like the package name, version, edition, and so on.

5. There is also the `[dependencies]` section in the `Cargo.toml` file that is where you list out all your dependencies.

6. The top-level project directory is just for README files, license information, configuration files, and anything else **not** related to your code.

7. If you started a project that doesn't use Cargo, simply move your source files into a `src/` directory and create an appropriate `Cargo.toml` file. Voila!

8. To build a project with Cargo, simply run `cargo build`.

    a. If you'd like to simply check if your code compiles and not have to build it fully, you can do `cargo check`, which will run much faster than `cargo build`. Cool unique feature of Rust's ecosystem!

    b. Cargo will place the executable output in the `target/debug/` directory by default, since the default build configuration will be **Debug** while you are developing. Once you're ready for release, you can use the `--release` flag with the build command to place the executable instead in `target/release/`.

9. You can build and run your project simply with `cargo run`. Awesome!

10. When you build for the first time ever, you'll get a `Cargo.lock` file (the equivalent of a `package-lock.json`). **Never** directly edit this file - Cargo should be responsible for this instead.

11. Cargo, like any good build system, is able to detect what files have changed whenever we re-run a build and _only_ build those files, thereby more efficiently building.

12. Another awesome part of Cargo is its commands are the _same regardless of OS_!

# CHAPTER 2 - PROGRAMMING A GUESSING GAME

## SETTING UP A NEW PROJECT

1. This project impmlements a guessing game where the program will internally generate a random integer between 1 and 100 (inclusive) and prompt the player to guess it. Wrong guesses will be met with a "too high" or "too low" reply and a correct guess will be met with a congratulatory message and an exit of the program.

2. Go ahead and `cargo new` this project.

## PROCESSING A GUESS

```
use std::io;

fn main()
{
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {guess}");
}
```

1. We'll first look at getting user input. To do this, we'll want to `use std::io` to _bring the `io` library into scope_. The `io` library _comes from the standard library known as `std:`_.

2. By default, Rust has a set of items defined in the standard library that _it brings into scope of every program_. This set is called the **prelude**, and you can see everything in it in the standard library documentation.

3. If something you want is not in the prelude, you have to bring it into scope explicitly with a `use` statement.

4. To create a new _variable_ for storing that user input, `let mut guess = String::new();`. There's a lot going on in this line. Let's break it down:

    a. `let` indicates a statement for creating a new variable.

    b. In Rust, variables are **immutable** by default, so if you'd like the variable to be mutable (_I read as "mutate-able" to help myself recall what it means), then you use the keyword `mut` with it.

    c. For some reason, instead of "assign", the book uses the term "bind".

    d. `String::new()` returns a new **instance** of a `String` type, which is a type provided by the standard library that is "a growable, UTF-8 encoded bit of text."

    e. The `::` syntax in the `::new` line indicates that `new` is an **associated function** (_i.e., method_) of the `String` type. An associated function is a function implemented on a type. You'll find the `new` function on many types because it's a common action.

    f. So overall, the `let mut guess = String::new()` line creates a new, mutable empty String variable.

5. Now that we have a variable to store the user's guess, let's actually read it in! To do that, we invoke the `stdin` function of the `io` module in the following way: `io::stdin().read_line(&mut guess).expect("Failed to read line".)`. Once again, lots going on. Let's break it down:

    a. The `stdin` function actually returns an instance of the `std::io::Stdin` type that "represents a **handle** to the standard input for your terminal."

    b. The _associated function_ `read_line` of the `Stdin` type gets the input from the user. We're also passing in `&mut guess` as the argument to `read_line` to tell it where to store the user input.

    c. The `&` indicates that we are passing the argument _by reference_. Rust has a lot to say about references, and boasts its safety and ease-of-use with references, and we'll learn more about those later in Chapter 4.

    d. In Rust, references are actually _immutable by default_, so to indicate we are passing the reference to the `guess` variable in a mutable way, we need to write `&mut guess`. `&guess` would mean the reference to guess is passed but the _underlying_ data is _not to be modified_.

    e. The full job of `read_line` is to take whatever the user types into standard input and _append it_ to the end of the string that was passed in **without** overwriting any of the string's prior contents. The string argument needs to be mutable in order for the function to do its job.

    f. Now `read_line` puts the user input into the string we pass but it _also_ returns a `Result` value. `Result` is an **enumeration** which is a type that can be one of multiple possible states and may hold a value as well. Each possible state is called a **variant**. Chapter 6 will cover enums in more details.

    g. The purpose of the `Result` type is to encode error-handling information. `Results` variants are `Ok` and `Err`.

    h. The `Ok` variant indicates the operation was successful and inside `Ok` is the successfully generated value.

    i. The `Err` variant indicates the operation failed and it will hold information about how or why the operation failed.

    j. The `Result` type has methods defined for it. One of those methods is the `expect` method. If the `Result` is `Err`, then `expect` will cause the program to crash and display the message that you passed in. An `Ok` will cause `expect` to take the value stored in `Ok` and return just that value. For `read_line`, it stores the _number of bytes in the string it appended_.

    k. An `Err` would happen with `read_line` most likely from an error in the underlying OS (e.g., the user doing a `Ctrl+C`).

    l. If you don't call `expect`, the program _will_ compile _but_ you'll get a warning regarding an `unused Result`.

    m. We'll learn more about error-handling in Chapter 9.

6. Inside of a `println!` statement, you can include placeholders for printing variables, hence the `println!("You guessed: {guess});`.
    
    a. `{<var_name>}` placeholders inside of the format string are for variables. If you'd like to print the result of an expression, place empty curly-braces inside the format string and then _after_ the format string, a comma-separate list with the expression(s) in the order you want, like in `println!("y + 2 = {}", y + 2);`.

## GENERATING A SECRET NUMBER

Now to generate a secret number. Rust does not include random number functionality in the standard library (yet) but the Rust team does provide a `rand` **crate** that provides random functionality.

### Using a Crate to Get More Functionality

1. A **crate** is a collection of Rust source code files packaged together.

2. The project we've been building is a **binary crate** - i.e., one that produces an executable.

3. The `rand` crate is a **library crate**, which is a crate that contains code that is intended to be used in other programs and can't be executed on its own.

4. Before we can use the `rand` crate, it must be specified as a dependency. We specify that in the `Cargo.toml` beneath the `[dependencies]` section. We'd specifically like version `^0.8.5` (i.e., non-breaking versions of `rand` starting from `0.8.5` ==> `0.8.5` to `0.9.0` [recall that when the major version is 0, minor version bumps constitute breaks]), so we put:
```
[dependencies]
rand = "0.8.5"
```

5. After changing the `Cargo.toml` file, if you `cargo build`, you'll find Cargo downloads the crate you specified _along with any of its dependencies (recursively)_ and then begins compiling them.

6. Crates are fetched by Cargo from the **registry**, which is a copy of data from `Crates.io`.

7. At this point, you'll find the `Cargo.lock` file has been modified as well (the analog of a `package-lock.json`). This file is only modified right at the first build or when you change the `Cargo.toml` file's dependency list or package info _or you run the `cargo update` command_.

    a. The `cargo update` command will modify the `Cargo.lock` file to dependency versions that are _compatible_ with your `Cargo.toml`. It won't modify the `Cargo.toml` file.

    b. More information regarding Cargo and its ecosystem will be provided in Chapter 14.

### Generating a Random Number

Let's use the `rand` crate already!

```
use std:io;
use rand::Rng;

fn main()
{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {guess});
}
```
1. So really, all we added was `use rand::Rng` and `let secret_number = rand::thread_rng().gen_range(1..=100)`.

2. The `use rand::Rng` brings the `rand` library into scope and also specified the `Rng` **trait**. The `Rng` trait "defines methods that random number generators implement, and this trait _must be in scope_ for us to use those methods." Chapter 10 will cover traits in more detail.

3. The `rand::thread_rng()` returns a generator type that _is also local to the current thread_ and _seeded by the OS_.

4. The `gen_range` method of the random number generator is _defined by the_ `Rng` trait that we brought into scope and it takes a **range expression** as an argument.

5. The range expression used here is of the form `start..=end` which is an _inclusive_ range defining upper and lower bounds. To learn more about the `rand` crate, every crate comes with documentation that you can access using `cargo doc --open` and clicking on the crate name on the left pane.

## COMPARING THE GUESS TO THE SECRET NUMBER
```
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main()
{
    println!("Guess the name!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number});

    println!("Please input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number)
    {
        Ordering::Less      ==> println!("Too small!");
        Ordering::Greater   ==> println!("Too big!");
        Ordering::Equal     ==> println!("You win!");
    }
}
```
1. First we add another `use` statement, bringing in a _type_ called `std::cmp::Ordering` into scope from the standard library. The `Ordering` type is another enum and has variants `Less`, `Greater`, and `Equal`. These are the three outcomes possible from a comparison.

2. Then we add the `match` statement to do the comparison we wanted.

    a. The `cmp` method (_also a library??_) of the string object `guess` does the comparison. It is a method of most types and you pass to it a reference of the other object being compared - in this case the secret number. It returns an `Ordering` type.

    b. We use the `match` statement to decide what to do based on what is returned from `guess.cmp(&secret_number)` (_kind of like a `switch` statement?_).

    c. A `match` expression is made up of **arms**. An arm consists of a _pattern_ to match against, and the code that should be run if the value given to `match` fits that arm's pattern. Rust will look through each arm from top to bottom until it finds a _match_ and then executes that arm's code and exits (it does not check any further arms).

3. Then, in recognizing the `cmp` method cannot be done on a string vs an integer, we convert the `guess` string into an integer. That's where the line `let guess: u32 = guess.trim().parse().expect("Please input a number!");` comes in.

    a. We create a _new_ variable, also called `guess`. This new variable actually **shadows** (i.e., overrides) the previous value and type of `guess` with a new one.

    b. **Shadowing** lets us reuse a variable name rather than creating two unique variables - something like `guess_str` and `guess_num`. More about this is described in Chapter 3 but just know that this is commonly used when converting a value from one type to another.

    c. The `: u32` **annotation** tells Rust what number type we want this new `guess` to be.

    d. The `trim` method on the `guess` string eliminates any _whitespace_ at the beginning and end of the string (which we want to make sure isn't there before we call `parse`). This is always necessary here because the user presses the Enter key to indicate they are done giving input and that Enter key press adds a `\n` to that input buffer. In fact, on windows, a carriage return `\r` is also added. So `trim` removes those extra characters at the end.

    e. The `parse` method does the conversion from string to integer.
    
    f. This can easily fail with non-numerical inputs which is why `parse` returns a `Result` type just like `read_line` did. In this case, the `parse` method returns the parsed-value with an `Ok` and something else with an `Err`.

## ALLOWING MULTIPLE GUESSES WITH LOOPING

```
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main()
{
    println!("Guess the name!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_numer}");

    loop
    {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");
        let guess: u32 = guess.trim().parse().expect("Please input a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number)
        {
            Ordering::Less      ==> println!("Too small!");
            Ordering::Greater   ==> println!("Too big!");
            Ordering::Equal     ==>
            {
                println!("You win! :D");
                break;
            }
        }
    }
}
```

1. The `loop` statement there brings in an infinite loop, just like `while(1)` in C.

2. The `break` statement exits the infinite loop.

## HANDLING INVALID INPUT
Instead of crashing the program when the user inputs a non-number, let's make the game ignore that input and `continue`. We can do that by replacing the `expect` method call with the following:
```
let guess: u32 = match guess.trim().parse()
{
    Ok(num) => num,
    Err(_)  => continue,
};
```
1. We switch the `expect` call to a `match` expression! A demonstration of the creativity available with a `match` expression.

2. The `Ok(num)` pattern says "the value held with the `Ok` state". The `Ok(num) ==> num,` statement says "return the number stored inside the `Ok` state.

3. The `Err(_)` is a catchall pattern that says "any `Err` value".