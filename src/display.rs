use crate::eval::*;

pub fn print_all_evals(hands: &Vec<String>) {
    let evals = evaluate_hands(hands);
    evals.iter().for_each(|(hand, &code)| {
        println!("{} => {}", hand, Evaluation::decode(code));
    })
}

pub fn print_all_hands_ranked(hands: &Vec<String>) {
    let ranked_hands = rank_hands(evaluate_hands(hands));
    ranked_hands.into_iter().enumerate().for_each(|(i, hands)| {
        print!("{:02}: ", i);
        hands.iter().enumerate().for_each(|(j, h)| {
            print!("{}{}", if j != 0 { ", " } else { "" }, h);
        });
        println!();
    });
}
