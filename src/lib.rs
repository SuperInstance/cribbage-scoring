/// Cribbage hand scoring: fifteens, pairs, runs, flushes, knobs.
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank { Ace=1, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit { Hearts, Diamonds, Clubs, Spades }

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self { Self { rank, suit } }
    pub fn value(&self) -> u8 { std::cmp::min(self.rank as u8, 10) }
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self { Self { cards } }
    pub fn score(&self, starter: &Card) -> u16 {
        let mut pts = 0u16;
        let all = {
            let mut v = self.cards.clone();
            v.push(*starter);
            v
        };
        pts += fifteens(&all);
        pts += pairs(&all);
        pts += runs(&all);
        pts += flush(&self.cards, starter);
        pts += knobs(&self.cards, starter);
        pts
    }
}

fn fifteens(cards: &[Card]) -> u16 {
    let vals: Vec<u8> = cards.iter().map(|c| c.value()).collect();
    let n = vals.len();
    let mut count = 0u16;
    for mask in 1u32..(1 << n) {
        let s: u8 = (0..n).filter(|&i| mask & (1 << i) != 0).map(|i| vals[i]).sum();
        if s == 15 { count += 2; }
    }
    count
}

fn pairs(cards: &[Card]) -> u16 {
    let mut pts = 0u16;
    for i in 0..cards.len() {
        for j in (i+1)..cards.len() {
            if cards[i].rank == cards[j].rank { pts += 2; }
        }
    }
    pts
}

fn runs(cards: &[Card]) -> u16 {
    let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank as u8).collect();
    ranks.sort();
    let mut best = 0u16;
    let n = ranks.len();
    for start in 0..n {
        let mut len = 1u16;
        let mut mult = 1u16;
        let mut i = start + 1;
        while i < n {
            if ranks[i] == ranks[i-1] + 1 { len += 1; mult = 1; }
            else if ranks[i] == ranks[i-1] { mult += 1; }
            else { break; }
            i += 1;
        }
        if len >= 3 && len * mult > best { best = len * mult; }
    }
    best
}

fn flush(hand: &[Card], starter: &Card) -> u16 {
    if hand.iter().all(|c| c.suit == hand[0].suit) {
        if starter.suit == hand[0].suit { 5 } else { 4 }
    } else { 0 }
}

fn knobs(hand: &[Card], starter: &Card) -> u16 {
    if hand.iter().any(|c| c.rank == Rank::Jack && c.suit == starter.suit) { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_perfect_hand() {
        let hand = Hand::new(vec![
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Five, Suit::Diamonds),
        ]);
        let starter = Card::new(Rank::Five, Suit::Spades);
        assert!(hand.score(&starter) >= 29);
    }
}
