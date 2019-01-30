pub const RANKS: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];
pub const SUITS: [&str; 4] = ["c", "d", "h", "s"];
pub const TYPES: [&str; 10] = [
    "Highcard",
    "Pair",
    "TwoPair",
    "Trips",
    "Straight",
    "Flush",
    "Fullhouse",
    "Quads",
    "StraightFlush",
    "RoyalFlush",
];

pub const RANK_MASK: [usize; 13] = [
    0x0001, 0x0002, 0x0004, 0x0008, 0x0010, 0x0020, 0x0040, 0x0080, 0x0100, 0x0200, 0x0400, 0x0800,
    0x1000,
];

// game related
pub const NUM_CARDS: usize = 52;
pub const NUM_RANKS: usize = 13;
pub const NUM_SUITS: usize = 4;
pub const SIZE_HAND: u8 = 7;

// bitmask related
pub const MASK_SUIT: u64 = 0x1FFF;
pub const OFFSET_CLUBS: u8 = 0;
pub const OFFSET_DIAMONDS: u8 = 13;
pub const OFFSET_HEARTS: u8 = 26;
pub const OFFSET_SPADES: u8 = 39;
pub const OFFSET_TYPE: usize = 24;
pub const OFFSET_MAJOR: usize = 20;
pub const OFFSET_MINOR: usize = 16;
pub const OFFSET_KICKER: usize = 0;

// hand types
pub const HIGHCARD: usize = 0;
pub const PAIR: usize = 1;
pub const TWO_PAIR: usize = 2;
pub const TRIPS: usize = 3;
pub const STRAIGHT: usize = 4;
pub const FLUSH: usize = 5;
pub const FULLHOUSE: usize = 6;
pub const QUADS: usize = 7;
pub const STRAIGHT_FLUSH: usize = 8;
pub const ROYAL_FLUSH: usize = 9;

// ranks
pub const TWO: usize = 0;
pub const THREE: usize = 1;
pub const FOUR: usize = 2;
pub const FIVE: usize = 3;
pub const SIX: usize = 4;
pub const SEVEN: usize = 5;
pub const EIGHT: usize = 6;
pub const NINE: usize = 7;
pub const TEN: usize = 8;
pub const JACK: usize = 9;
pub const QUEEN: usize = 10;
pub const KING: usize = 11;
pub const ACE: usize = 12;
// Represents an empty major/minor rank slot in the evaluation code. Although it sets all bits to 1
// it doesn't effect the correct order of hands regarding their strength, because for any specific
// combination type (Pair, TwoPair, ...) either NULL is set or the rank corresponding bit pattern.
pub const NULL: usize = 0xF;
