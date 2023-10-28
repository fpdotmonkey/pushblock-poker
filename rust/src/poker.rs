//! Data and methods for evaluating and comparing poker hands
//!
//! This is with a 52-card deck and french-suited cards.  In other
//! words, cards that go from Two to Ace and are suited Spade, Heart,
//! Club, and Diamond.

/// Face value of a playing card, with Ace high and Two low
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// The suits of conventional playing cards
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

/// A representation of a conventional playing card
#[derive(Debug, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Constructs a card with the given rank and suit
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    /// The card's suit
    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// The card's rank, or face value
    pub fn rank(&self) -> Rank {
        self.rank
    }
}

/// Compare based on rank
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rank {
            rank if rank > other.rank => Some(std::cmp::Ordering::Greater),
            rank if rank < other.rank => Some(std::cmp::Ordering::Less),
            _ => Some(std::cmp::Ordering::Equal),
        }
    }
}

/// Compare based on rank
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

/// Every kind of poker hand category
#[derive(Debug, PartialEq, PartialOrd)]
pub enum HandKind {
    /// A hand only evaluated on the ranks of its cards
    HighCard([Rank; 5]),
    /// A hand with two cards ranked `pair` and others `high_cards`
    Pair {
        /// The rank of the pair
        pair: Rank,
        /// The ranks of the remaining cards
        high_cards: [Rank; 3],
    },
    /// A hand with two pairs
    TwoPair {
        /// The rank of the greater pair
        pair_high: Rank,
        /// The rank of the lesser pair
        pair_low: Rank,
        /// The card not in the pair
        high_card: Rank,
    },
    /// A hand with three `Rank`s
    ThreeOfAKind(Rank),
    /// A hand of cards of sequential rank with `Rank` the highest
    Straight(Rank),
    /// A hand of uniform suit with cards of the described ranks
    Flush([Rank; 5]),
    /// A hand with a set of three `Rank`s and any pair
    FullHouse(Rank),
    /// A hand with four `Rank`s
    FourOfAKind(Rank),
    /// A hand of all the same suit that's also a straight
    StraightFlush(Rank),
    /// A hand of all the same suit that's also an Ace-high straight
    RoyalFlush,
}

/// A construct for evaluating and comparing sets of cards
#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// This creates a hand from a set of five or more cards
    ///
    /// This will panic if fewer than five cards are passed in
    pub fn new(cards: Vec<Card>) -> Hand {
        assert!(cards.len() >= 5, "there must be 5 or more cards in a hand");
        let mut sorted_cards: Vec<Card> = cards;
        sorted_cards.sort_by(|card0, card1| card1.rank().partial_cmp(&card0.rank()).unwrap());
        Hand {
            cards: sorted_cards,
        }
    }

    /// This compute the category of hand that represents the cards
    ///
    /// It will find the kind of hand that most favorably describes
    /// the given card.  For example, the hand "3♠ 3♥ 3♣ 2♥ 2♠"
    /// would be described as a full house instead of as a pair or
    /// three of a kind since that's the highest ranked option.
    pub fn kind(&self) -> HandKind {
        if self.is_flush() {
            match self.straight_high_card() {
                Some(Rank::Ace) => return HandKind::RoyalFlush,
                Some(rank) => return HandKind::StraightFlush(rank),
                None => {}
            }
            return HandKind::Flush(
                self.cards
                    .iter()
                    .map(|card| card.rank())
                    .collect::<Vec<Rank>>()
                    .try_into()
                    .unwrap(),
            );
        }

        if let Some(straight_high_card) = self.straight_high_card() {
            return HandKind::Straight(straight_high_card);
        }

        if let Some(set_hand) = self.set_hand() {
            return set_hand;
        }

        return HandKind::HighCard(
            self.cards
                .iter()
                .map(|card| card.rank())
                .collect::<Vec<Rank>>()
                .try_into()
                .unwrap(),
        );
    }

    fn straight_high_card(&self) -> Option<Rank> {
        // handle the Ace-low case
        let mut straight_sorted_cards: Vec<Card> = self.cards.clone();
        if straight_sorted_cards[0].rank() == Rank::Ace
            && straight_sorted_cards[straight_sorted_cards.len() - 1].rank() == Rank::Two
        {
            straight_sorted_cards.rotate_left(1);
        }

        let mut previous_card: &Card = &straight_sorted_cards[0];
        for card in straight_sorted_cards.iter().skip(1) {
            if card.rank()
                != match previous_card.rank() {
                    Rank::Ace => Rank::King,
                    Rank::Two => Rank::Ace,
                    Rank::Three => Rank::Two,
                    Rank::Four => Rank::Three,
                    Rank::Five => Rank::Four,
                    Rank::Six => Rank::Five,
                    Rank::Seven => Rank::Six,
                    Rank::Eight => Rank::Seven,
                    Rank::Nine => Rank::Eight,
                    Rank::Ten => Rank::Nine,
                    Rank::Jack => Rank::Ten,
                    Rank::Queen => Rank::Jack,
                    Rank::King => Rank::Queen,
                }
            {
                return None;
            }
            previous_card = card;
        }
        Some(straight_sorted_cards[0].rank())
    }

    fn is_flush(&self) -> bool {
        let first_card_suit: Suit = self.cards[0].suit();
        return self.cards.iter().all(|card| card.suit() == first_card_suit);
    }

    fn set_hand(&self) -> Option<HandKind> {
        let sets: std::collections::HashMap<Rank, usize> =
            self.cards
                .iter()
                .fold(std::collections::HashMap::new(), |mut acc, card| {
                    let counter = acc.entry(card.rank()).or_insert(0);
                    *counter += 1;
                    acc
                });

        let mut three_of_a_kind: Option<Rank> = None;
        let mut pairs: Vec<Rank> = vec![];
        let mut high_cards: Vec<Rank> = vec![];
        for (&rank, &count) in sets.iter() {
            if count == 4 {
                return Some(HandKind::FourOfAKind(rank));
            }
            if count == 3 {
                three_of_a_kind = Some(rank);
            } else if count == 2 {
                pairs.push(rank);
            } else {
                high_cards.push(rank);
            }
        }

        if let Some(three_of_a_kind) = three_of_a_kind {
            if pairs.is_empty() {
                return Some(HandKind::ThreeOfAKind(three_of_a_kind));
            }
            return Some(HandKind::FullHouse(three_of_a_kind));
        }
        if pairs.len() == 2 {
            debug_assert_eq!(high_cards.len(), 1);

            return Some(HandKind::TwoPair {
                pair_high: *pairs.iter().max().unwrap(),
                pair_low: *pairs.iter().min().unwrap(),
                high_card: high_cards[0],
            });
        }
        if pairs.len() == 1 {
            debug_assert_eq!(high_cards.len(), 3);

            high_cards.sort_by(|card0, card1| card1.partial_cmp(card0).unwrap());
            return Some(HandKind::Pair {
                pair: pairs[0],
                high_cards: high_cards.try_into().unwrap(),
            });
        }

        None
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.kind().partial_cmp(&other.kind())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card_from_str(card: &str) -> Card {
        assert_eq!(card.len(), 2);

        let rank: Rank = match card.chars().nth(0) {
            Some('2') => Rank::Two,
            Some('3') => Rank::Three,
            Some('4') => Rank::Four,
            Some('5') => Rank::Five,
            Some('6') => Rank::Six,
            Some('7') => Rank::Seven,
            Some('8') => Rank::Eight,
            Some('9') => Rank::Nine,
            // T for 10 to make it so only one character needs to be matched
            Some('T') => Rank::Ten,
            Some('J') => Rank::Jack,
            Some('Q') => Rank::Queen,
            Some('K') => Rank::King,
            Some('A') => Rank::Ace,
            _ => panic!("invalid card rank"),
        };

        let suit: Suit = match card.chars().nth(1) {
            Some('s') => Suit::Spade,
            Some('h') => Suit::Heart,
            Some('c') => Suit::Club,
            Some('d') => Suit::Diamond,
            _ => panic!("invalid card suit"),
        };

        Card::new(rank, suit)
    }

    fn cards_from_str(cards: &str) -> Vec<Card> {
        // hand ought to be formatted /(Rs )*(Rs)/ where R is a rank and
        // s a suit
        assert!(cards.len() % 3 != 1);

        cards.rsplit(" ").map(|card| card_from_str(card)).collect()
    }

    mod card {
        use super::*;

        #[test]
        fn rank_ord() {
            let cards: Vec<Card> = vec![
                card_from_str("2s"),
                card_from_str("3d"),
                card_from_str("4c"),
                card_from_str("5h"),
                card_from_str("6s"),
                card_from_str("7d"),
                card_from_str("8c"),
                card_from_str("9h"),
                card_from_str("Ts"),
                card_from_str("Jd"),
                card_from_str("Qc"),
                card_from_str("Kh"),
                card_from_str("As"),
            ];

            for (i, card0) in cards.iter().enumerate() {
                for card1 in cards.iter().skip(i + 1) {
                    assert!(card1 > card0);
                }
            }
        }

        #[test]
        fn rank_eq_neq() {
            // identical cards are equal
            assert_eq!(card_from_str("2s"), card_from_str("2s"));
            // cards with different rank are unequal
            assert!(card_from_str("3h") != card_from_str("4d"));
            // cards of the same rank and different suit are equal
            assert_eq!(card_from_str("4d"), card_from_str("4h"));
        }

        fn rank_getter() {
            assert_eq!(card_from_str("As").rank(), Rank::Ace);
            assert_eq!(card_from_str("Kh").rank(), Rank::King);
            assert_eq!(card_from_str("Qc").rank(), Rank::Queen);
            assert_eq!(card_from_str("Jd").rank(), Rank::Jack);
        }

        #[test]
        fn suit_getter() {
            assert_eq!(card_from_str("As").suit(), Suit::Spade);
            assert_eq!(card_from_str("Kh").suit(), Suit::Heart);
            assert_eq!(card_from_str("Qc").suit(), Suit::Club);
            assert_eq!(card_from_str("Jd").suit(), Suit::Diamond);
        }
    }

    #[test]
    fn hands_evaluate_and_compare_correctly() {
        // This is a big list of hands and what kind of hand they
        // should evaluate to.  They're in order of highest value to
        // lowest.
        let hands: Vec<(&str, HandKind)> = vec![
            ("Ts Js Qs Ks As", HandKind::RoyalFlush),
            ("Qh Th Kh 9h Jh", HandKind::StraightFlush(Rank::King)),
            ("7c 8c 9c Tc Jc", HandKind::StraightFlush(Rank::Jack)),
            ("Tc 9c 8c 7c 6c", HandKind::StraightFlush(Rank::Ten)),
            ("9s 5s 6s 7s 8s", HandKind::StraightFlush(Rank::Nine)),
            ("Ad 2d 3d 4d 5d", HandKind::StraightFlush(Rank::Five)),
            ("9d As Ah Ac Ad", HandKind::FourOfAKind(Rank::Ace)),
            ("8s 8h Kc 8c 8d", HandKind::FourOfAKind(Rank::Eight)),
            ("5s 5h 5c 5d 2d", HandKind::FourOfAKind(Rank::Five)),
            ("2s 2h 2c 2d 8h", HandKind::FourOfAKind(Rank::Two)),
            ("Ac Kc Kd Kh Ad", HandKind::FullHouse(Rank::King)),
            ("7h 7d 7c 6s 6c", HandKind::FullHouse(Rank::Seven)),
            ("6h 7h 6d 7d 6c", HandKind::FullHouse(Rank::Six)),
            (
                "As 3s 5s 7s Ks",
                HandKind::Flush([Rank::Ace, Rank::King, Rank::Seven, Rank::Five, Rank::Three]),
            ),
            (
                "Jd 9d 8d 4d 3d",
                HandKind::Flush([Rank::Jack, Rank::Nine, Rank::Eight, Rank::Four, Rank::Three]),
            ),
            (
                "4c 9c Tc 7c 3c",
                HandKind::Flush([Rank::Ten, Rank::Nine, Rank::Seven, Rank::Four, Rank::Three]),
            ),
            (
                "2h 4h 5h 6h 7h",
                HandKind::Flush([Rank::Seven, Rank::Six, Rank::Five, Rank::Four, Rank::Two]),
            ),
            ("Th Kh Jh Qh Ad", HandKind::Straight(Rank::Ace)),
            ("Td 9s 8h 7d 6c", HandKind::Straight(Rank::Ten)),
            ("7c 5c 9c 6h 8c", HandKind::Straight(Rank::Nine)),
            ("As 2c 3d 4h 5s", HandKind::Straight(Rank::Five)),
            ("Kh Kd Ks 4c 2h", HandKind::ThreeOfAKind(Rank::King)),
            ("7d Qd 6h Qc Qs", HandKind::ThreeOfAKind(Rank::Queen)),
            ("3h 2s 2d 2c 7h", HandKind::ThreeOfAKind(Rank::Two)),
            (
                "As Ah Ks Kc Qd",
                HandKind::TwoPair {
                    pair_high: Rank::Ace,
                    pair_low: Rank::King,
                    high_card: Rank::Queen,
                },
            ),
            (
                "Ac Ad Kh Kd Js",
                HandKind::TwoPair {
                    pair_high: Rank::Ace,
                    pair_low: Rank::King,
                    high_card: Rank::Jack,
                },
            ),
            (
                "As Ah Qs Qh Jh",
                HandKind::TwoPair {
                    pair_high: Rank::Ace,
                    pair_low: Rank::Queen,
                    high_card: Rank::Jack,
                },
            ),
            (
                "Kh Kd Qd Qc Jc",
                HandKind::TwoPair {
                    pair_high: Rank::King,
                    pair_low: Rank::Queen,
                    high_card: Rank::Jack,
                },
            ),
            (
                "Jh Js 3c 3s 2h",
                HandKind::TwoPair {
                    pair_high: Rank::Jack,
                    pair_low: Rank::Three,
                    high_card: Rank::Two,
                },
            ),
            (
                "Ts Th 8s 7h 4c",
                HandKind::Pair {
                    pair: Rank::Ten,
                    high_cards: [Rank::Eight, Rank::Seven, Rank::Four],
                },
            ),
            (
                "3h 3d 4h 5h 6h",
                HandKind::Pair {
                    pair: Rank::Three,
                    high_cards: [Rank::Six, Rank::Five, Rank::Four],
                },
            ),
            (
                "2s 2c 4d 5d 6d",
                HandKind::Pair {
                    pair: Rank::Two,
                    high_cards: [Rank::Six, Rank::Five, Rank::Four],
                },
            ),
            (
                "2h 2d 3c 5c 6c",
                HandKind::Pair {
                    pair: Rank::Two,
                    high_cards: [Rank::Six, Rank::Five, Rank::Three],
                },
            ),
            (
                "2s 2c 3h 4h 6h",
                HandKind::Pair {
                    pair: Rank::Two,
                    high_cards: [Rank::Six, Rank::Four, Rank::Three],
                },
            ),
            (
                "2h 2d 3s 4s 5s",
                HandKind::Pair {
                    pair: Rank::Two,
                    high_cards: [Rank::Five, Rank::Four, Rank::Three],
                },
            ),
            (
                // This isn't a straight because an Ace can't turn corners
                "Kh Ad 2s 3c 4h",
                HandKind::HighCard([Rank::Ace, Rank::King, Rank::Four, Rank::Three, Rank::Two]),
            ),
            (
                "Kd Qd 7s 4s 3h",
                HandKind::HighCard([
                    Rank::King,
                    Rank::Queen,
                    Rank::Seven,
                    Rank::Four,
                    Rank::Three,
                ]),
            ),
            (
                "Td 8c 7c 6c 5c",
                HandKind::HighCard([Rank::Ten, Rank::Eight, Rank::Seven, Rank::Six, Rank::Five]),
            ),
            (
                "Tc 8d 7d 6d 4d",
                HandKind::HighCard([Rank::Ten, Rank::Eight, Rank::Seven, Rank::Six, Rank::Four]),
            ),
            (
                "Th 8s 7s 5s 4s",
                HandKind::HighCard([Rank::Ten, Rank::Eight, Rank::Seven, Rank::Five, Rank::Four]),
            ),
            (
                "Ts 8h 6h 5h 4h",
                HandKind::HighCard([Rank::Ten, Rank::Eight, Rank::Six, Rank::Five, Rank::Four]),
            ),
            (
                "Td 7c 6c 5c 4c",
                HandKind::HighCard([Rank::Ten, Rank::Seven, Rank::Six, Rank::Five, Rank::Four]),
            ),
            (
                "9c 7d 6d 5d 4d",
                HandKind::HighCard([Rank::Nine, Rank::Seven, Rank::Six, Rank::Five, Rank::Four]),
            ),
        ];

        // the hands evaluate the the correct kind
        assert!(hands
            .iter()
            .all(|(hand_str, hand_kind)| Hand::new(cards_from_str(hand_str)).kind() == *hand_kind));
        // the hands compare correctly
        assert!(hands.iter().enumerate().all(|(i, (hand_str, _))| hands
            .iter()
            .skip(i + 1)
            .all(|(other_str, _)| Hand::new(cards_from_str(hand_str))
                > Hand::new(cards_from_str(other_str)))));
    }
}
