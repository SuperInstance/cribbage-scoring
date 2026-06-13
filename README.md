# Cribbage Scoring

**A Rust library for scoring Cribbage hands** that implements the complete official scoring rules — fifteens, pairs, runs, flushes, and "his knobs" — using bitmask enumeration and combinatorial analysis.

## Why It Matters

Cribbage is a 400-year-old card game whose scoring system is surprisingly algorithmic. Scoring a hand requires checking every subset of cards for combinations that sum to exactly 15, detecting runs of 3+ consecutive ranks, identifying pairs, checking suit uniformity, and finding Jacks matching the starter card's suit. This makes it an excellent exercise in combinatorial enumeration.

**The algorithmic challenge:** "Fifteens" requires checking all 2⁵ − 1 = 31 non-empty subsets of a 5-card hand (4 hand cards + 1 starter). The implementation uses a bitmask approach, iterating through all subset masks and summing the card values for each. This is O(2ⁿ) where n = 5, which is trivially fast.

Beyond the game itself, this library demonstrates bitmask enumeration, a technique used in NP-hard problems like set cover, the traveling salesman problem (via Held-Karp), and bitmask dynamic programming.

## How It Works

The scoring function evaluates five categories, each implemented as a separate function:

**Fifteens (`fifteens`):** Uses bitmask enumeration to check all 31 non-empty subsets of the 5-card combination. For each subset, sum the card values (face cards = 10, Aces = 1, others = face value). Each subset summing to exactly 15 scores 2 points. Time complexity: O(2ⁿ · n) where n = 5.

**Pairs (`pairs`):** Compares all C(5,2) = 10 pairs of cards. Each pair of equal rank scores 2 points. Triplets score as 3 pairs (6 points), four-of-a-kind as 6 pairs (12 points). O(n²).

**Runs (`runs`):** Sorts the ranks, then scans for the longest consecutive sequence of 3+ cards. Handles duplicate ranks in runs (e.g., 3-3-4-5 scores as two runs of 3 = 6 points) by tracking run length and a multiplier for duplicate cards. O(n log n) for sorting, O(n) for the scan.

**Flush (`flush`):** If all 4 hand cards share the same suit, scores 4 points. If the starter card also matches that suit, scores 5 points. O(n).

**His Knobs (`knobs`):** If any hand card is a Jack whose suit matches the starter card's suit, scores 1 point. O(n).

The maximum possible score is **29** — achieved by holding J♥, 5♥, 5♣, 5♦ with 5♠ as starter: 8 points from fifteens (the Jack combines with each 5 for 15, four times × 2), plus 8 more from the four 5s making 15 in C(4,2)=6 pairings... totaling 29 with runs contributing. The test suite validates this perfect hand.

## Quick Start

```rust
use cribbage_scoring::{Hand, Card, Rank, Suit};

// Score a hand
let hand = Hand::new(vec![
    Card::new(Rank::Seven, Suit::Hearts),
    Card::new(Rank::Seven, Suit::Diamonds),
    Card::new(Rank::Eight, Suit::Hearts),
    Card::new(Rank::Nine, Suit::Hearts),
]);
let starter = Card::new(Rank::Six, Suit::Clubs);

let score = hand.score(&starter);
println!("Hand scores {} points", score);
// Fifteens: 7+8=15 (×2 for two 7s) = 4pts
// Run: 6-7-8-9 (×2 for two 7s) = 8pts
// Pair: two 7s = 2pts
// Total: 14 points
```

## API

### `Card`
- `new(rank: Rank, suit: Suit) -> Self` — Create a card
- `value(&self) -> u8` — Cribbage value (min(rank, 10))

### `Rank` (enum)
`Ace=1, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King`

### `Suit` (enum)
`Hearts, Diamonds, Clubs, Spades`

### `Hand`
- `new(cards: Vec<Card>) -> Self` — Create a 4-card hand
- `score(&self, starter: &Card) -> u16` — Calculate total score with starter card. O(2ⁿ + n²)

## Architecture Notes

This is a standalone scoring library used in SuperInstance's game-theory toolkit, demonstrating combinatorial enumeration and exact-scoring algorithms. It's used in game AI evaluation and statistical hand analysis.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## License

MIT
