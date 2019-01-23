# About
This is a tool to evaluate Poker hands very quickly and is based off of _PokerStove_. I made a few small adaptations though. It has no CLI yet, so you have to be able to modify and recompile the Rust code each time you want to evaluate a different hand. That's shitty UX, I know, but come on ... :)

# How to use it

* Clone the repository.
* Modify 'main.rs' to a hand of your interest:
    ```Rust
    let hand = Hand::new("KcKdKsKh2c2d2s");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));
    ```
* 'cargo run' the code.