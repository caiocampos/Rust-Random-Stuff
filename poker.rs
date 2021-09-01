use std::cmp::{Ordering, Reverse};

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    match hands.len() {
        0 => return None,
        1 => return Some(Vec::from(hands)),
        _ => (),
    }
    let hand_list: Vec<Hand> = hands.iter().cloned().filter_map(Hand::parse).collect();
    if hand_list.len() != hands.len() {
        return None;
    }
    let (res, _) =
        hand_list
            .iter()
            .enumerate()
            .fold((Vec::new(), 0), |(mut res, mut max), (i, hand)| {
                match hand.rank.cmp(&max) {
                    Ordering::Greater => {
                        res.clear();
                        max = hand.rank;
                        res.push(hands[i]);
                    }
                    Ordering::Equal => res.push(hands[i]),
                    _ => (),
                }
                (res, max)
            });
    Some(res)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
    Unknown,
}

impl Suit {
    pub fn is_unknown(&self) -> bool {
        self == &Self::Unknown
    }
}

impl From<&str> for Suit {
    fn from(suit: &str) -> Self {
        match suit {
            "H" => Self::Hearts,
            "D" => Self::Diamonds,
            "C" => Self::Clubs,
            "S" => Self::Spades,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Card {
    A(Suit),
    K(Suit),
    Q(Suit),
    J(Suit),
    N10(Suit),
    N9(Suit),
    N8(Suit),
    N7(Suit),
    N6(Suit),
    N5(Suit),
    N4(Suit),
    N3(Suit),
    N2(Suit),
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        match (self.get_type(), other.get_type()) {
            (_, (_, Suit::Unknown)) => None,
            ((_, Suit::Unknown), _) => None,
            ((a, _), (b, _)) => Some(a.cmp(&b)),
        }
    }
}

impl Card {
    pub fn parse(card: &str) -> Option<Self> {
        let len = card.len();
        if len < 2 {
            return None;
        }
        let (kind, suit) = card.split_at(len - 1);
        let suit = Suit::from(suit);
        if suit.is_unknown() {
            return None;
        }
        let card = match kind {
            "A" => Self::A(suit),
            "K" => Self::K(suit),
            "Q" => Self::Q(suit),
            "J" => Self::J(suit),
            "10" => Self::N10(suit),
            "9" => Self::N9(suit),
            "8" => Self::N8(suit),
            "7" => Self::N7(suit),
            "6" => Self::N6(suit),
            "5" => Self::N5(suit),
            "4" => Self::N4(suit),
            "3" => Self::N3(suit),
            "2" => Self::N2(suit),
            _ => return None,
        };
        Some(card)
    }

    pub fn get_type(&self) -> (u32, Suit) {
        match self {
            Self::A(a) if !a.is_unknown() => (13, *a),
            Self::K(a) if !a.is_unknown() => (12, *a),
            Self::Q(a) if !a.is_unknown() => (11, *a),
            Self::J(a) if !a.is_unknown() => (10, *a),
            Self::N10(a) if !a.is_unknown() => (9, *a),
            Self::N9(a) if !a.is_unknown() => (8, *a),
            Self::N8(a) if !a.is_unknown() => (7, *a),
            Self::N7(a) if !a.is_unknown() => (6, *a),
            Self::N6(a) if !a.is_unknown() => (5, *a),
            Self::N5(a) if !a.is_unknown() => (4, *a),
            Self::N4(a) if !a.is_unknown() => (3, *a),
            Self::N3(a) if !a.is_unknown() => (2, *a),
            Self::N2(a) if !a.is_unknown() => (1, *a),
            _ => (0, Suit::Unknown),
        }
    }

    pub fn rank(&self) -> u32 {
        self.get_type().0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rank {
    // HighCard(max_card, ..., min_card)
    HighCard(Card, Card, Card, Card, Card),
    // OnePair(pair, max_kicker, ..., min_kicker)
    OnePair(Card, Card, Card, Card),
    // TwoPair(max_pair, min_pair, kicker)
    TwoPair(Card, Card, Card),
    // ThreeOfAKind(triplet, max_kicker, min_kicker)
    ThreeOfAKind(Card, Card, Card),
    // Straight(max_card)
    Straight(Card),
    // Flush(max_card, ..., min_card)
    Flush(Card, Card, Card, Card, Card),
    // FullHouse(triplet, pair)
    FullHouse(Card, Card),
    // FourOfAKind(quads, kicker)
    FourOfAKind(Card, Card),
    // StraightFlush(max_card)
    StraightFlush(Card),
}

impl Ord for Rank {
    fn cmp(&self, other: &Rank) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Rank) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Rank {
    pub fn parse(cards: &[Card]) -> Option<Self> {
        if cards.len() != 5 {
            return None;
        }
        let mut cards = cards.to_vec();
        cards.sort_by_key(|&card| Reverse(card));
        let (ranks, suits): (Vec<u32>, Vec<Suit>) = cards.iter().map(Card::get_type).unzip();
        if let Some(res) = Self::is_straight_flush(&ranks, &suits, &cards) {
            return Some(res);
        }
        if let Some(res) = Self::is_pair(&ranks, &cards) {
            return Some(res);
        }
        Some(Self::HighCard(
            cards[0], cards[1], cards[2], cards[3], cards[4],
        ))
    }

    fn is_straight_flush(ranks: &Vec<u32>, suits: &Vec<Suit>, cards: &Vec<Card>) -> Option<Self> {
        let is_low_straight = [13, 4, 3, 2, 1].iter().eq(ranks);
        let straight = is_low_straight
            || ranks
                .iter()
                .enumerate()
                .all(|(i, &rank)| rank == ranks[0] - (i as u32));
        let straight_max = match is_low_straight {
            true => 1,
            _ => 0,
        };
        let flush = suits.iter().all(|&suit| suit == suits[0]);
        match (straight, flush) {
            (true, true) => Some(Self::StraightFlush(cards[straight_max])),
            (true, false) => Some(Self::Straight(cards[straight_max])),
            (false, true) => Some(Self::Flush(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            _ => None,
        }
    }

    fn is_pair(ranks: &Vec<u32>, cards: &Vec<Card>) -> Option<Self> {
        // FourOfAKind 4 4 4 4 3
        if ranks[0] == ranks[3] {
            return Some(Self::FourOfAKind(cards[0], cards[4]));
        }
        // FourOfAKind 5 4 4 4 4
        if ranks[1] == ranks[4] {
            return Some(Self::FourOfAKind(cards[1], cards[0]));
        }
        // 5 5 5 ? ?
        if ranks[0] == ranks[2] {
            let res = match ranks[3] == ranks[4] {
                // FullHouse 5 5 5 4 4
                true => Self::FullHouse(cards[0], cards[3]),
                // ThreeOfAKind 5 5 5 4 3
                _ => Self::ThreeOfAKind(cards[0], cards[3], cards[4]),
            };
            return Some(res);
        }
        // ? ? 4 4 4
        if ranks[2] == ranks[4] {
            let res = match ranks[0] == ranks[1] {
                // FullHouse 5 5 4 4 4
                true => Self::FullHouse(cards[2], cards[0]),
                // ThreeOfAKind 6 5 4 4 4
                _ => Self::ThreeOfAKind(cards[2], cards[0], cards[1]),
            };
            return Some(res);
        }
        // ThreeOfAKind 5 4 4 4 3
        if ranks[1] == ranks[3] {
            return Some(Self::ThreeOfAKind(cards[1], cards[0], cards[4]));
        }
        let pairs: Vec<usize> = (0..5)
            .filter(|&i| (i < 4 && ranks[i] == ranks[i + 1]) || (i > 0 && ranks[i] == ranks[i - 1]))
            .collect();
        let not_in_pairs = |i: &usize| !pairs.iter().any(|j| j == i);
        match pairs.len() {
            // OnePair
            2 => {
                let kickes: Vec<usize> = (0..5).filter(not_in_pairs).collect();
                return Some(Self::OnePair(
                    cards[pairs[0]],
                    cards[kickes[0]],
                    cards[kickes[1]],
                    cards[kickes[2]],
                ));
            }
            // TwoPair
            4 => {
                if let Some(kicker) = (0..5).step_by(2).find(not_in_pairs) {
                    return Some(Self::TwoPair(
                        cards[pairs[0]],
                        cards[pairs[2]],
                        cards[kicker],
                    ));
                }
                None
            }
            _ => None,
        }
    }

    pub fn rank(&self) -> u32 {
        let rot = |val: u32, cards: u32| val << (cards * 4);
        match self {
            Self::HighCard(c1, c2, c3, c4, c5) => [c1, c2, c3, c4, c5]
                .iter()
                .enumerate()
                .map(|(i, c)| rot(c.rank(), 4 - (i as u32)))
                .sum::<u32>(),
            Self::OnePair(c1, c2, c3, c4) => {
                [c1, c2, c3, c4]
                    .iter()
                    .enumerate()
                    .map(|(i, c)| rot(c.rank(), 3 - (i as u32)))
                    .sum::<u32>()
                    + rot(1, 5)
            }
            Self::TwoPair(c1, c2, c3) => {
                [c1, c2, c3]
                    .iter()
                    .enumerate()
                    .map(|(i, c)| rot(c.rank(), 2 - (i as u32)))
                    .sum::<u32>()
                    + rot(2, 5)
            }
            Self::ThreeOfAKind(c1, c2, c3) => {
                [c1, c2, c3]
                    .iter()
                    .enumerate()
                    .map(|(i, c)| rot(c.rank(), 2 - (i as u32)))
                    .sum::<u32>()
                    + rot(3, 5)
            }
            Self::Straight(c) => rot(4, 5) + c.rank(),
            Self::Flush(c1, c2, c3, c4, c5) => {
                [c1, c2, c3, c4, c5]
                    .iter()
                    .enumerate()
                    .map(|(i, c)| rot(c.rank(), 4 - (i as u32)))
                    .sum::<u32>()
                    + rot(5, 5)
            }
            Self::FullHouse(c1, c2) => {
                [c1, c2]
                    .iter()
                    .enumerate()
                    .map(|(i, c)| rot(c.rank(), 1 - (i as u32)))
                    .sum::<u32>()
                    + rot(6, 5)
            }
            Self::FourOfAKind(c1, c2) => {
                rot(c1.rank(), 4)
                    + [c1, c2]
                        .iter()
                        .enumerate()
                        .map(|(i, c)| rot(c.rank(), 1 - (i as u32)))
                        .sum::<u32>()
                    + rot(7, 5)
            }
            Self::StraightFlush(c) => rot(8, 5) + c.rank(),
        }
    }
}

#[derive(Debug)]
struct Hand {
    pub rank: u32,
    pub hand: String,
}

impl Hand {
    pub fn parse(hand: &str) -> Option<Self> {
        let cards: Vec<Card> = hand
            .split(' ')
            .filter_map(|card| Card::parse(card))
            .collect();
        match Rank::parse(&cards) {
            Some(rank) => {
                let res = Hand {
                    rank: rank.rank(),
                    hand: hand.into(),
                };
                Some(res)
            }
            _ => None,
        }
    }
}
