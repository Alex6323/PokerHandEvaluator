use hand_eval;
use hand_eval::eval::*;
use hand_eval::models::*;

fn main() {
    let hand = Hand::new("KcQsJhTh8s2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    let hand = Hand::new("AcAsAhTh8s2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));
    
}