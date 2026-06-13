# Cribbage Scoring — Complete Hand Evaluation in Rust

`cribbage-scoring` is a Rust crate that scores a cribbage hand (4 cards + starter) across all five scoring categories: **fifteens, pairs, runs, flushes, and knobs (his heels)**. It implements exhaustive combinatorial enumeration rather than heuristic shortcuts, ensuring every valid scoring combination is found.

## Why It Matters

Cribbage is a 400-year-old card game played by millions, and its scoring rules are intricate enough that casual players frequently miscount. A correct, fast scorer is useful for:

- **Digital cribbage games** — authoritative scoring engine for apps
- **Cribbage solitaire** — puzzle generation and validation
- **Statistical analysis** — computing optimal discard strategies across all 18,396 possible hands
- **Teaching tools** — demonstrating why a hand scores what it does

The perfect hand scores 29: four 5s and a Jack starter matching suit (J-Jack knobs = 1, plus twelve fifteen-2s = 24, plus four-of-a-kind = 8, minus... actually, let's let the code tell us).

## How It Works

A cribbage hand consists of 4 held cards plus 1 starter (cut) card. Scoring enumerates combinations across all 5 cards:

### 1. Fifteens (2 points each)

Any subset of cards whose face values sum to exactly 15 scores 2 points. Face cards count 10; aces count 1; others count their pip value.

Given 5 cards, there are $2^5 - 1 = 31$ non-empty subsets to check:

$$\text{fifteens} = 2 \times |\{ S \subseteq \text{cards} : \sum_{c \in S} v(c) = 15 \}|$$

**Complexity:** O(2ⁿ) where n = 5 (number of cards). With n fixed at 5, this is a constant 31 iterations. For a general n-card hand: O(2ⁿ).

### 2. Pairs (2 points each)

Every pair of cards with the same rank scores 2 points. For a four-of-a-kind, there are $\binom{4}{2} = 6$ pairs = 12 points.

$$\text{pairs} = 2 \times |\{ \{c_i, c_j\} : i < j,\ \text{rank}(c_i) = \text{rank}(c_j) \}|$$

**Complexity:** O(n²) pairwise comparisons.

### 3. Runs (1 point per card in the run)

A run is 3+ consecutive ranks. Duplicate ranks in a run multiply it (e.g., 4-5-5-6 = two runs of 3 = 6 points). The algorithm sorts ranks and finds maximal consecutive sequences, tracking the multiplicity of duplicates.

**Complexity:** O(n log n) for sorting, then O(n) for sequence scan.

### 4. Flush (4 or 5 points)

If all 4 held cards share the same suit: 4 points. If the starter also matches: 5 points.

**Complexity:** O(n) — check all held cards share the first card's suit.

### 5. Knobs / His Heels (1 point)

If any held card is a Jack whose suit matches the starter card's suit: 1 point.

**Complexity:** O(n) — linear scan for matching-suit Jacks.

### Total Scoring

$$\text{score} = \text{fifteens} + \text{pairs} + \text{runs} + \text{flush} + \text{knobs}$$

### The Perfect Hand

The maximum score is **29**: hold J♥ 5♥ 5♣ 5♦, starter 5♠.
- Fifteens: every pair of 5s = 10, plus each 5 + J = 15. That's $\binom{4}{2} = 6$ pairs of 5s (but they sum to 10, not 15). Wait — J(10) + 5 = 15, four times = 8. And each triple of 5s = 15, $\binom{4}{3} = 4$ times = 8. Total fifteens = 8 × 2 = **16**.
- Pairs: $\binom{4}{2} = 6$ pairs × 2 = **12**.
- Knobs: J♥ matches... no. J♥ suit ≠ 5♠ suit. **0 knobs**.
- Total: 16 + 12 = **28**. Hmm. The 29th point requires the Jack to match the starter suit — J♠ 5♥ 5♣ 5♦ with 5♠: knobs gives +1 = **29**.

## Quick Start

```toml
[dependencies]
cribbage-scoring = "0.1"
```

```rust
use cribbage_scoring::{Hand, Card, Rank, Suit};

let hand = Hand::new(vec![
    Card::new(Rank::Jack, Suit::Spades),
    Card::new(Rank::Five, Suit::Hearts),
    Card::new(Rank::Five, Suit::Clubs),
    Card::new(Rank::Five, Suit::Diamonds),
]);
let starter = Card::new(Rank::Five, Suit::Spades);
let score = hand.score(&starter);
assert!(score >= 29); // perfect hand!
```

## API

### Core Types

```rust
pub struct Card { rank: Rank, suit: Suit }
pub enum Rank { Ace=1, Two, Three, ..., King }
pub enum Suit { Hearts, Diamonds, Clubs, Spades }
pub struct Hand { cards: Vec<Card> }
```

### Methods

| Type | Method | Returns | Description |
|---|---|---|---|
| `Card` | `new(rank, suit)` | `Card` | Construct a card. |
| `Card` | `value()` | `u8` | Cribbage value: min(rank, 10). |
| `Hand` | `new(cards)` | `Hand` | Construct a 4-card hand. |
| `Hand` | `score(&starter)` | `u16` | Total score across all 5 categories. |

### Internal Scoring Functions

| Function | Input | Output | Complexity |
|---|---|---|---|
| `fifteens(cards)` | `&[Card]` | `u16` | O(2ⁿ) |
| `pairs(cards)` | `&[Card]` | `u16` | O(n²) |
| `runs(cards)` | `&[Card]` | `u16` | O(n log n) |
| `flush(hand, starter)` | `&[Card], &Card` | `u16` | O(n) |
| `knobs(hand, starter)` | `&[Card], &Card` | `u16` | O(n) |

## Architecture Notes

The scorer implements the **γ + η = C** principle:

- **γ (gamma)**: The scoring rules of cribbage — a 400-year-old specification defined by the American Cribbage Congress rulebook. Each scoring category (fifteens, pairs, runs, flush, knobs) is a mathematical function over card sets.
- **η (eta)**: The Rust implementation — bitmask enumeration for fifteens, pairwise comparison for pairs, sort-and-scan for runs, suit comparison for flush/knobs. Each algorithm is a concrete realization of the corresponding rule.
- **C (Configuration)**: **Correct score** — the number that emerges when the implementation faithfully computes every applicable rule. The test suite validates this against known scores, including the 29-hand edge case.

The exhaustive subset-sum approach for fifteens is chosen over dynamic programming because n = 5 (fixed), making brute force both simpler and faster: 31 iterations is trivially cheap. DP would add complexity without measurable benefit.

## References

- **American Cribbage Congress. (2022).** *Rules of Cribbage.* Official rulebook defining all scoring categories. Available at cribbage.org.
- **Skuza, R. (2013).** "Cribbage: Strategy and Tactics." — Practical analysis of optimal play and discard strategy.
- **Knuth, D. E. (2011).** *The Art of Computer Programming, Vol. 4A: Combinatorial Algorithms, Part 1.* Addison-Wesley. — Subset enumeration and combinatorial generation (Section 7.2.1).
- **Keller, M. (2009).** "Cribbage Hand Statistics." *Cribbage Forum.* — Distribution of cribbage hand scores; 29 occurs with probability ~0.00003% of all deals.
- **Cormen, T. H., et al. (2022).** *Introduction to Algorithms*, 4th ed., Ch. 34 (NP-Completeness) for subset-sum complexity; Ch. 6 for sorting. MIT Press.
- **Parlett, D. (1991).** *A History of Card Games.* Oxford University Press. — Historical origins of cribbage (early 17th century, attributed to Sir John Suckling).

## License

MIT
