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
            major = if self.major != NULL { format!(" {}", self.get_major()) } else { String::new() },
            minor = if self.minor != NULL { format!(" {}", self.get_minor()) } else { String::new() },
            kickers = if self.kickers != 0 { format!(" {}", self.get_kickers()) } else { String::new() }
        )
    }
}

pub fn evaluate(hand: &Hand) -> Code {
    let bitmask = hand.get_bitmask();

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
    match SIZE_HAND - num_ranks {
        0 => {
            return encode(HIGHCARD, NULL, NULL, MSB5_MASK[ranks]);
        }
        1 => {
            //println!("{:13b}", ranks);
            let pair_mask = ranks ^ (clubs ^ diamonds ^ hearts ^ spades);
            //println!("{:13b}", pair_mask);
            let major = MSB_RANK[pair_mask];
            //println!("{} => {:b}", major, RANK_MASK[major]);
            let kickers = MSB3_MASK[ranks ^ RANK_MASK[major]];
            //println!("{:013b}", kickers);
            //println!("{:032b}", encode(PAIR, major, 0, kickers));
            return encode(PAIR, major, NULL, kickers);
        }
        2 => {
            let two_pair_mask = ranks ^ (clubs ^ diamonds ^ hearts ^ spades);
            if two_pair_mask != 0 {
                let major = MSB_RANK[two_pair_mask];
                let minor = MSB_RANK[two_pair_mask ^ MSB1_MASK[two_pair_mask]];
                let kicker = MSB1_MASK[ranks ^ two_pair_mask];

                return encode(TWO_PAIR, major, minor, kicker);
            } else {
                let trips_mask = ((clubs & diamonds) | (hearts & spades))
                    & ((clubs & hearts) | (diamonds & spades));
                let major = MSB_RANK[trips_mask];
                let kicker1 = MSB1_MASK[ranks ^ trips_mask];
                let kicker2 = MSB1_MASK[(ranks ^ trips_mask) ^ kicker1];

                return encode(TRIPS, major, NULL, kicker1 | kicker2);
            }
        }
        n @ _ => {
            let quads_mask = clubs & diamonds & hearts & spades;

            if quads_mask != 0 {
                //Quads
                let major = MSB_RANK[quads_mask];
                let kicker = MSB1_MASK[ranks ^ quads_mask];

                return encode(QUADS, major, NULL, kicker);
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

                        return encode(FULLHOUSE, major, minor, 0);
                    }
                } else {
                    // Two Pair (with triple pairs)
                    let major = MSB_RANK[two_pair_mask];
                    let minor = MSB_RANK[two_pair_mask ^ RANK_MASK[major]];
                    let kicker = MSB1_MASK[(ranks ^ RANK_MASK[major]) ^ RANK_MASK[minor]];

                    return encode(TWO_PAIR, major, minor, kicker);
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

// TODO: for each passed in hand determines its strength code
fn assign_strength(hands: Vec<&str>) -> HashMap<&str, Code> {
    hands.iter().for_each(|s| {
        //Hand::new(s).
    });
    unimplemented!()
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
