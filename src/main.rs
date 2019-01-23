use hand_eval;
use hand_eval::eval::*;
use hand_eval::models::*;

fn main() {
    // Royal Flush
    let hand = Hand::new("KcQcJcTcAc2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Straight Flush
    let hand = Hand::new("KcQcJcTc9c2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Quads (1 quad, 3 singles)
    let hand = Hand::new("KcKdKsKh9c2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Quads (1 quad, 1 trip)
    let hand = Hand::new("KcKdKsKh2c2d2s");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Quads (1 quad, 1 pair, 1 single)
    let hand = Hand::new("KcKdKsKh9c3d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Fullhouse (1 trip - 1 pair)
    let hand = Hand::new("KcKdKsQhQc2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Fullhouse (2 trips)
    let hand = Hand::new("KcKdKsQh3c3c3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Flush
    let hand = Hand::new("KcQcJcTc8c2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Straight
    let hand = Hand::new("KcQhJcTs9c2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Straight (Ace low)
    let hand = Hand::new("Ac2h5cTs3cQd4d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Trips
    let hand = Hand::new("AcAsAhTh8s2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // TwoPair (2 pairs, 3 singles)
    let hand = Hand::new("AcAsThTc8s2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));
        
    // TwoPair (3 pairs, 1 single)
    let hand = Hand::new("AcAsThTc8s8d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));

    // Pair
    let hand = Hand::new("AcAsJhTc8s2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));
    
    // Highcard
    let hand = Hand::new("KcQsJhTh8s2d3d");
    println!("{} => {}", hand, Evaluation::decode(evaluate(&hand)));
}