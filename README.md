My TRPL memo (The Rust Programming Language)
====

# Table of Contents
these are TRPL demos with descriptions.
some projects have my original code, but mainly the same as TRPL.

to run these implementations,
```
git clone git@github.com:FukudaTaiga/rust_tutorials.git
```
and
```
cd [target_directory] && cargo run
```
some repository need to add option "+nightly"

only for hello_world directory,
```
cd hello_world && rustc hello_world.rs && ./hello_world
```

## Section 1
[Hello World](https://github.com/FukudaTaiga/rust_tutorials/tree/main/hello_world)

[Hello cargo](https://github.com/FukudaTaiga/rust_tutorials/tree/main/hello_cargo)

Cargo commands
- new
- check
- build
- run

## Section 2
[Guessing Game](https://github.com/FukudaTaiga/rust_tutorials/tree/main/guessing_game)

Example of Rust code and give elementary explanation like
- println!
- String
- use
- match
- loop

## Section 3
[Variables and Type](https://github.com/FukudaTaiga/rust_tutorials/tree/main/var_and_type)

Variables
- declaration
- immutability and mutability
- shadowing

Basic Types
- scalar type integer, float, bool, char
- multiple type tuple, array

[Function and Loop](https://github.com/FukudaTaiga/rust_tutorials/tree/main/functions)

Syntax
- Statement
- Expression

Function
- declaration
- basic knowledge

Loops
- loop
- while
- for

## Section 4
[Ownership](https://github.com/FukudaTaiga/rust_tutorials/tree/main/ownership)

Concept of Ownership
- What is ownership?
- memory management heap or stack
- value substitution clone(if Copy) or move
- reference rules
- slice

## Section 5
[Struct](https://github.com/FukudaTaiga/rust_tutorials/tree/main/structure)

Structure (like class)
- declaration
- tuple structure (not mention new type pattern in this section, see Sec19)
- unit structure
- method implementation

## Section 6
[Enum and Pattern matching](https://github.com/FukudaTaiga/rust_tutorials/tree/main/enum_pattern)

Enum
- declaration
- implementation
- pattern match demo

## Section 7
[Package, Crate, Module](https://github.com/FukudaTaiga/rust_tutorials/tree/main/package_module)

Project module separation
- main, lib, mod.rs
- public and private controll
- import modules using keyword of mod, use, extern
- base of package, crate, module, path

## Section 8
[Collection - Vector, String, HashMap](https://github.com/FukudaTaiga/rust_tutorials/tree/main/collections)

Vector
- declaration, access
- push, pop
- how is multi typed vector implemented

String(&str slice)
- Why it is difficlt
- declaration, access
- push, push_str, format!, chars, as_bytes, bytes

HashMap
- declaration, access
- insert, get, entry, or_insert

Above collections Exercise

## Section 9
[Handling Error](https://github.com/FukudaTaiga/rust_tutorials/tree/main/handle_error)

Errors
- unrecoverable, with panic! macro
- recoverable, with Result<T, E>
- unwrap, expect, operator ?

## Section 10
[Generics, Trait, Lifetime](https://github.com/FukudaTaiga/rust_tutorials/tree/main/generics_trait_lifetime)

Generics
- for structure as example
- impl of generic typed
- trait bound
- performance, monomorphization

Trait
- trait, defining common behavior
- default impl and how to impl it for concrete class
- coherence, orphan rule
- syntax suger impl (traitname)
- where statement

Concept of Lifetime
- What is lifetime?
- life time annotation
- static life time

## Section 11
[Tests](https://github.com/FukudaTaiga/rust_tutorials/tree/main/rust_test)

Rust Test
- unit test
- cfg(test), should_panic, Result case, ignore
- associative test
- helpers best? practice

## Section 12
[File I/O and review so far](https://github.com/FukudaTaiga/rust_tutorials/tree/main/minigrep)

Summary of former half and a bit of new
- std::env, std::process
- TDD

## Section 13
[Iterator, Closure](https://github.com/FukudaTaiga/rust_tutorials/tree/main/iterator_closure)

Iterator
- its trait, lazy eval
- creation with iter(), next()
- consumer adoptor like sum, fold
- iterator adoptor like map, flat_map, filter,...

Closure
- notation
- difference to function
- its trait Fn, FnMut, FnOnce
- Casher case example

## Section 14
**skipping, make it later**

[cargo, Crate.io](https://github.com/FukudaTaiga/rust_tutorials/tree/main/)

## Section 15
[Smart Pointer](https://github.com/FukudaTaiga/rust_tutorials/tree/main/smart_pointer)

**need "cargo +nightly ..." to compile**

Box
- What is Box?
- usage
- recursive strucure with Box, List Example
- Deref, Drop trait

Rc (Reference Count)
- What is Rc?
- multiple ownership
- usage, Rc::clone, strong_count

RefCell + (Rc, Weak)
- internal mutability
- borrow_mut
- cyclic reference, Weak
- Rc::downgrade, upgrade, weak_count
- Tree implmentation

## Section 16
[Concurrency](https://github.com/FukudaTaiga/rust_tutorials/tree/main/concurrency)

Multi thread execution
- usage of thread::spawn(), join()
- Channel pattern mpsc::channel(), transmitter, receiver
- Mutex pattern Mutex/Arc and analogy for RefCell/Rc
- Send/Sync trait

## Section 17
**WIP**

[Object Oriented Programming](https://github.com/FukudaTaiga/rust_tutorials/tree/main/rust_oop)

Object Oriented Programming
- Rust's OOP aspect
- duck typing
- polymorphism with trait object and its overhead
- state pattern WIP

## Section 18
**WIP**

[Pattern matching](https://github.com/FukudaTaiga/rust_tutorials/tree/main/pattern_matching)

Pattern
- all expression and refutability
- destruction
- match this section have deprecated expression, so it may not be runnable (at 2021/9/30 OK)
- ref WIP
- guard WIP
- binding WIP

## Section 19
**WIP? procedual macro is very complicated and its exercise will be other repo?**

[Advanced feature](https://github.com/FukudaTaiga/rust_tutorials/tree/main/advanced_feature)

Unsafe
- raw pointer
- use unsafe function
- mutable static and difference to const
- unsafe trait
- Foreign Function Interface(FFI)

Advanced Trait
- associated type
- default type argument
- full path syntax
- new type pattern

Advanced type
- type alias
- never type and diverging function
- dynamic sized type(DST) and Sized trait, ?Sized syntax

Macro
- declarative macro
- procedual macro (very complex) WIP

## Section 20
**WIP**

[Multi thread Web Server](https://github.com/FukudaTaiga/rust_tutorials/tree/main/multi_thread_server)

this section refers to a Rust Compiler's problem
and complex life timeimplementation and so on
Read more carefully, its not enough

TCP, HTTP with std library (single thread)
- TcpListner, TcpStream
- bind, incoming, read and write with io and buffer, flush

refact to Multi thread
- Mutex implementation
- Compiler's problem
- CDD

Shutdown and Cleanup
- Drop trait implementing use case
- dead lock example
- refact codes with CDD

## Resourse
[TRPL/ja](https://doc.rust-jp.rs/book-ja/title-page.html)

[TRPL](https://github.com/rust-lang/book/tree/main/src)

[The Rust Reference](https://doc.rust-lang.org/reference/macros-by-example.html) - code examples