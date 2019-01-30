pub mod big_luts;
pub mod constants;
pub mod display;
pub mod eval;
pub mod models;

//use std::time::Instant;

pub fn test() {
    println!("It works");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    /*
    // Royal Flush
    let hand = Hand::new("KcQcJcTcAc2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Straight Flush
    let hand = Hand::new("KcQcJcTc9c2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Quads (1 quad, 3 singles)
    let hand = Hand::new("KcKdKsKh9c2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Quads (1 quad, 1 trip)
    let hand = Hand::new("KcKdKsKh2c2d2s");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Quads (1 quad, 1 pair, 1 single)
    let hand = Hand::new("KcKdKsKh9c3d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Fullhouse (1 trip - 1 pair)
    let hand = Hand::new("KcKdKsQhQc2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Fullhouse (2 trips)
    let hand = Hand::new("KcKdKsQh3c3c3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Flush
    let hand = Hand::new("KcQcJcTc8c2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Straight
    let hand = Hand::new("KcQhJcTs9c2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Straight (Ace low)
    let hand = Hand::new("Ac2h5cTs3cQd4d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Trips
    let hand = Hand::new("AcAsAhTh8s2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // TwoPair (2 pairs, 3 singles)
    let hand = Hand::new("AcAsThTc8s2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // TwoPair (3 pairs, 1 single)
    let hand = Hand::new("AcAsThTc8s8d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Pair
    let hand = Hand::new("AcAsJhTc8s2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    // Highcard
    let hand = Hand::new("KcQsJhTh8s2d3d");
    println!(
        "{} => {}",
        hand,
        Evaluation::decode(evaluate(hand.get_bitmask()))
    );

    let mut hands = vec![];
    hands.push("KcQsJhTh8s2d3d");
    hands.push("AcAsJhTc8s2d3d");
    hands.push("AcAsJcTh8s5d3d");

    println!();
    print_all_evals(&hands);

    println!();
    print_all_hands_ordered(&hands);

    let hand_bitmask = Hand::new("KcQsJhTh8s2d3d").get_bitmask();
    let start = Instant::now();
    evaluate(hand_bitmask);
    let stop = start.elapsed();
    println!(
        "{:.9} seconds",
        f64::from(stop.subsec_nanos()) / 1_000_000_000.
    );
    */
}
