use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::{big_luts::*, constants::*, models::*};
pub type Code = usize;

pub struct Evaluation {
    comb: usize,
    major: usize,
    minor: usize,
    kickers: usize,
}

impl Evaluation {
    pub fn decode(code: Code) -> Self {
        Evaluation {
            comb: (code >> OFFSET_COMB) & 0xF,
            major: (code >> OFFSET_MAJOR) & 0xF,
            minor: (code >> OFFSET_MINOR) & 0xF,
            kickers: code & 0x1FFF,
        }
    }

    pub fn get_comb(&self) -> &'static str {
        COMBS[self.comb]
    }

    pub fn get_major(&self) -> &'static str {
        if self.major != NULL {
            RANKS[self.major]
        } else {
            ""
        }
    }

    pub fn get_minor(&self) -> &'static str {
        if self.minor != NULL {
            RANKS[self.minor]
        } else {
            ""
        }
    }

    pub fn get_kickers(&self) -> String {
        let mut s = String::new();
        for i in 0..=12 {
            if self.kickers & RANK_MASK[12 - i] != 0 {
                s.push_str(&RANKS[12 - i]);
            }
        }
        s
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{comb}{major}{minor}{kickers}",
            comb = self.get_comb(),
            major = if self.major != NULL {
                format!(" {}", self.get_major())
            } else {
                String::new()
            },
            minor = if self.minor != NULL {
                format!(" {}", self.get_minor())
            } else {
                String::new()
            },
            kickers = if self.kickers != 0 {
                format!(" {}", self.get_kickers())
            } else {
                String::new()
            }
        )
    }
}

pub fn evaluate(bitmask: u64) -> Code {
    let clubs = get_ranks(bitmask, OFFSET_CLUBS);
    let diamonds = get_ranks(bitmask, OFFSET_DIAMONDS);
    let hearts = get_ranks(bitmask, OFFSET_HEARTS);
    let spades = get_ranks(bitmask, OFFSET_SPADES);

    let ranks = clubs | diamonds | hearts | spades;
    let num_ranks = NUM_ONBITS[ranks];

    if num_ranks >= 5 {
        let suit = if NUM_ONBITS[clubs] >= 5 {
            clubs
        } else if NUM_ONBITS[diamonds] >= 5 {
            diamonds
        } else if NUM_ONBITS[hearts] >= 5 {
            hearts
        } else if NUM_ONBITS[spades] >= 5 {
            spades
        } else {
            0
        };

        if suit != 0 {
            let major = STRAIGHT_TYPE[suit];

            if major != 0 {
                if major == ACE {
                    return encode(ROYAL_FLUSH, NULL, NULL, 0);
                } else {
                    return encode(STRAIGHT_FLUSH, major, NULL, 0);
                }
            } else {
                return encode(FLUSH, NULL, NULL, MSB5_MASK[suit]);
            }
        } else {
            let major = STRAIGHT_TYPE[ranks];

            if major != 0 {
                return encode(STRAIGHT, major, NULL, 0);
            };
        };
    };

    // match against number of duplicate ranks
    return match SIZE_HAND - num_ranks {
        0 => encode(HIGHCARD, NULL, NULL, MSB5_MASK[ranks]),
        1 => {
            let pair_mask = ranks ^ (clubs ^ diamonds ^ hearts ^ spades);
            let major = MSB_RANK[pair_mask];
            let kickers = MSB3_MASK[ranks ^ RANK_MASK[major]];
            encode(PAIR, major, NULL, kickers)
        }
        2 => {
            let two_pair_mask = ranks ^ (clubs ^ diamonds ^ hearts ^ spades);
            if two_pair_mask != 0 {
                let major = MSB_RANK[two_pair_mask];
                let minor = MSB_RANK[two_pair_mask ^ MSB1_MASK[two_pair_mask]];
                let kicker = MSB1_MASK[ranks ^ two_pair_mask];

                encode(TWO_PAIR, major, minor, kicker)
            } else {
                let trips_mask = ((clubs & diamonds) | (hearts & spades))
                    & ((clubs & hearts) | (diamonds & spades));
                let major = MSB_RANK[trips_mask];
                let kicker1 = MSB1_MASK[ranks ^ trips_mask];
                let kicker2 = MSB1_MASK[(ranks ^ trips_mask) ^ kicker1];

                encode(TRIPS, major, NULL, kicker1 | kicker2)
            }
        }
        n => {
            let quads_mask = clubs & diamonds & hearts & spades;

            if quads_mask != 0 {
                //Quads
                let major = MSB_RANK[quads_mask];
                let kicker = MSB1_MASK[ranks ^ quads_mask];

                encode(QUADS, major, NULL, kicker)
            } else {
                let two_pair_mask = ranks ^ (clubs ^ diamonds ^ hearts ^ spades);

                if NUM_ONBITS[two_pair_mask] != n {
                    let trips_mask = ((clubs & diamonds) | (hearts & spades))
                        & ((clubs & hearts) | (diamonds & spades));
                    let major = MSB_RANK[trips_mask];

                    if two_pair_mask != 0 {
                        //Fullhouse (with 1 triple and 1 pair)
                        let minor = MSB_RANK[two_pair_mask];

                        return encode(FULLHOUSE, major, minor, 0);
                    } else {
                        //Fullhouse (with 2 triples)
                        let minor = MSB_RANK[trips_mask ^ RANK_MASK[major]];

                        encode(FULLHOUSE, major, minor, 0)
                    }
                } else {
                    // Two Pair (with triple pairs)
                    let major = MSB_RANK[two_pair_mask];
                    let minor = MSB_RANK[two_pair_mask ^ RANK_MASK[major]];
                    let kicker = MSB1_MASK[(ranks ^ RANK_MASK[major]) ^ RANK_MASK[minor]];

                    encode(TWO_PAIR, major, minor, kicker)
                }
            }
        }
    };
}

#[inline]
fn get_ranks(bitmask: Bitmask, offset_suit: u8) -> usize {
    ((bitmask >> offset_suit) & MASK_SUIT) as usize
}

#[inline]
fn encode(value: usize, major: usize, minor: usize, kicker: usize) -> Code {
    (value << OFFSET_COMB)
        ^ (major << OFFSET_MAJOR)
        ^ (minor << OFFSET_MINOR)
        ^ (kicker << OFFSET_KICKER)
}

pub fn evaluate_hands(hands: &Vec<String>) -> HashMap<&String, Code> {
    assert!(!hands.is_empty());

    let mut evals = HashMap::with_capacity(hands.len());

    hands.iter().for_each(|hand| {
        let eval = evaluate(Hand::new(&hand).get_bitmask());
        evals.insert(hand, eval);
    });

    evals
}

pub fn rank_hands(evals: HashMap<&String, Code>) -> Vec<Vec<&String>> {
    assert!(!evals.is_empty());

    let mut sorted_by_code = vec![];
    evals.iter().for_each(|(&hand, &code)| {
        sorted_by_code.push((hand, code));
    });

    // create a list of sorted hands, where the strongest comes first
    sorted_by_code.sort_unstable_by_key(|&eval| eval.1);
    sorted_by_code.reverse();

    let mut outer = vec![];
    outer.push(vec![sorted_by_code[0].0]);

    for i in 1..sorted_by_code.len() {
        if sorted_by_code[i].1 == sorted_by_code[i - 1].1 {
            let count = outer.len();
            let inner = outer.get_mut(count - 1).unwrap();
            inner.push(sorted_by_code[i].0);
        } else {
            let inner = vec![sorted_by_code[i].0];
            outer.push(inner);
        }
    }

    outer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ace_low_straight() {
        let hand = Hand::new("Ad3s2dKhJs5h4d");
        assert_eq!(
            "Straight 5",
            Evaluation::decode(evaluate(&hand)).to_string()
        );
    }

    #[test]
    fn test_two_pair_with_triples() {
        let hand = Hand::new("AdAsKdKhJsJh4d");
        assert_eq!(
            "TwoPair A K J",
            Evaluation::decode(evaluate(&hand)).to_string()
        );
    }
}
