# About
This is a tool to evaluate Poker hands. It has no CLI yet, so you have to be able to compile and run Rust code.

# How to use it

* Clone the repository
* Modify 'main.rs' to a hand of your interest:
    ```Rust
    let hand = Hand::new("KcKdKsKh2c2d2s");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));
    ```
* 'cargo run' the code.