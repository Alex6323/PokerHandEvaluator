use std::fmt::{self, Display, Formatter};

use crate::constants::*;

pub type Bitmask = u64;

pub trait CardSet {
    fn get_bitmask(&self) -> Bitmask;
    fn reset(&mut self);
    fn count(&self) -> usize;
}

#[derive(Default)]
pub struct Deck {
    bitmask: Bitmask,
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            bitmask: 0x000F_FFFF_FFFF_FFFF,
        }
    }

    pub fn remove(&mut self, bitmask: Bitmask) {
        self.bitmask &= self.bitmask ^ bitmask;
    }

    pub fn draw(&mut self) {
        // TODO: randomly remove a card
        unimplemented!();
    }
}

impl CardSet for Deck {
    fn get_bitmask(&self) -> Bitmask {
        self.bitmask
    }

    fn reset(&mut self) {
        self.bitmask = 0x000F_FFFF_FFFF_FFFF;
    }

    fn count(&self) -> usize {
        //TODO: use lut for each suit and add results
        unimplemented!();
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "deck={:052b}", self.bitmask)
    }
}

pub struct Hand {
    bitmask: Bitmask,
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(s: &str) -> Self {
        assert!(s.len() == 14);

        // easier way to achieve this?
        let chars = s.chars().collect::<Vec<char>>();
        let cards = chars
            .chunks(2)
            .map(|chunk| Card::new(&chunk.iter().collect::<String>()))
            .collect::<Vec<Card>>();

        let mut bitmask = 0_u64;
        for card in &cards {
            bitmask |= card.get_bitmask();
        }

        Hand { bitmask, cards }
    }
}

impl CardSet for Hand {
    fn get_bitmask(&self) -> Bitmask {
        self.bitmask
    }

    fn reset(&mut self) {
        self.bitmask = 0x0;
        self.cards.clear();
    }

    fn count(&self) -> usize {
        self.cards.len()
    }
}

// Possible to generalize?
impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        //write!(f, "{:052b}", self.bitmask)
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}

pub struct Card {
    rank: usize,
    suit: usize,
}

impl Card {
    pub fn new(id: &str) -> Self {
        assert!(id.len() == 2);

        let (rank, suit) = id.split_at(1);

        assert!(RANKS.contains(&rank));
        assert!(SUITS.contains(&suit));

        let rank = RANKS.iter().position(|&r| r == rank).unwrap();
        let suit = SUITS.iter().position(|&s| s == suit).unwrap();

        Card { rank, suit }
    }

    pub fn get_rank(&self) -> usize {
        self.rank
    }

    pub fn get_suit(&self) -> usize {
        self.suit
    }

    pub fn get_index(&self) -> usize {
        NUM_RANKS * self.suit + self.rank
    }

    pub fn get_bitmask(&self) -> Bitmask {
        0x1 << self.get_index()
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        /*
        write!(
            f,
            "rank={:>n$}, suit={}, index={:>n$}",
            self.rank,
            self.suit,
            self.get_index(),
            n = 2
        )
        */
        write! {f, "{}{}", RANKS[self.rank], SUITS[self.suit]}
    }
}
