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
///
/// When comparing the value of the cards with `<`, `>`, `==`, and
/// friends, the rank is the only value that matters.
///
/// ```
/// assert_eq!(Card::new(Rank::Two, Suit::Diamond), Card::new(Rank::Two, Suit::Spade));
/// ```
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

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank > other.rank {
            Some(std::cmp::Ordering::Greater)
        } else if self.rank < other.rank {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.rank == other.rank;
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
        if self.is_flush() && self.straight_high_card().is_some() {
            if self.straight_high_card().unwrap() == Rank::Ace {
                return HandKind::RoyalFlush;
            }
            return HandKind::StraightFlush(self.straight_high_card().unwrap());
        }

        if self.is_flush() {
            return HandKind::Flush(
                self.cards
                    .iter()
                    .map(|card| card.rank())
                    .collect::<Vec<Rank>>()
                    .try_into()
                    .unwrap(),
            );
        }

        if self.straight_high_card().is_some() {
            return HandKind::Straight(self.straight_high_card().unwrap());
        }

        if self.set_hand().is_some() {
            return self.set_hand().unwrap();
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
        return Some(straight_sorted_cards[0].rank().clone());
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
                    return acc;
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

        if three_of_a_kind.is_some() {
            if pairs.len() > 0 {
                return Some(HandKind::FullHouse(three_of_a_kind.unwrap()));
            }
            return Some(HandKind::ThreeOfAKind(three_of_a_kind.unwrap()));
        }
        if pairs.len() == 2 {
            assert_eq!(pairs.len(), 2);
            assert_eq!(high_cards.len(), 1);

            return Some(HandKind::TwoPair {
                pair_high: *pairs.iter().max().unwrap(),
                pair_low: *pairs.iter().min().unwrap(),
                high_card: high_cards[0],
            });
        }
        if pairs.len() == 1 {
            assert_eq!(pairs.len(), 1);
            assert_eq!(high_cards.len(), 3);

            high_cards.sort_by(|card0, card1| card1.partial_cmp(&card0).unwrap());
            return Some(HandKind::Pair {
                pair: pairs[0],
                high_cards: high_cards.try_into().unwrap(),
            });
        }

        return None;
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
        // hand ought to be formatted /(Rs )+/ where R is the rank and
        // s the suit
        assert!(cards.len() % 3 != 1);

        cards.rsplit(" ").map(|card| card_from_str(card)).collect()
    }

    mod card {
        use super::*;

        #[test]
        fn rank_gt() {
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
        fn rank_not_gt() {
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

            for (i, card0) in cards.iter().rev().enumerate() {
                for card1 in cards.iter().rev().skip(i + 1) {
                    assert!(!(card1 > card0));
                }
            }
        }

        #[test]
        fn rank_lt() {
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

            for (i, card0) in cards.iter().rev().enumerate() {
                for card1 in cards.iter().rev().skip(i + 1) {
                    assert!(card1 < card0);
                }
            }
        }

        #[test]
        fn rank_not_lt() {
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
                    assert!(!(card1 < card0));
                }
            }
        }

        #[test]
        fn rank_eq_neq() {
            // identical cards are equal
            assert_eq!(card_from_str("2s"), card_from_str("2s"));
            assert!(!(card_from_str("2s") != card_from_str("2s")));

            // cards with different rank are unequal
            assert!(card_from_str("3h") != card_from_str("4d"));
            assert!(!(card_from_str("3h") == card_from_str("4d")));

            // cards of the same rank and different suit are equal
            assert_eq!(card_from_str("4d"), card_from_str("4h"));
            assert!(!(card_from_str("4d") != card_from_str("4h")));
        }

        #[test]
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

    mod hand {
        use super::*;

        #[test]
        fn compares_correctly() {
            let sorted_hands: Vec<Hand> = vec![
                Hand::new(cards_from_str("Ts Js Qs Ks As")), // Royal flush
                Hand::new(cards_from_str("7c 8c 9c Tc Jc")), // Straight flush
                Hand::new(cards_from_str("5s 5h 5c 5d 2d")), // Four of a kind
                Hand::new(cards_from_str("6s 6h 6c Kc Kd")), // Full house
                Hand::new(cards_from_str("Jd 9d 8d 4d 3d")), // Flush
                Hand::new(cards_from_str("Td 9s 8h 7d 6c")), // Straight
                Hand::new(cards_from_str("Qc Qs Qh 9h 2s")), // Three of a kind
                Hand::new(cards_from_str("Jh Js 3c 3s 2h")), // Two pair
                Hand::new(cards_from_str("Ts Th 8s 7h 4c")), // Pair
                Hand::new(cards_from_str("Kd Qd 7s 4s 3h")), // High card
            ];

            for (i, hand0) in sorted_hands.iter().enumerate() {
                for hand1 in sorted_hands.iter().skip(i + 1) {
                    assert!(hand1 != hand0);
                    assert!(hand1 < hand0, "{:?} < {:?}", hand1.kind(), hand0.kind());
                }
            }

            for (i, hand0) in sorted_hands.iter().rev().enumerate() {
                for hand1 in sorted_hands.iter().rev().skip(i + 1) {
                    assert!(hand1 > hand0);
                }
            }
        }

        #[test]
        fn royal_flush() {
            let spade_royal_flush: Vec<Card> = cards_from_str("Ts Js Qs Ks As");
            let heart_royal_flush: Vec<Card> = cards_from_str("Ah Kh Qh Jh Th");
            let club_royal_flush: Vec<Card> = cards_from_str("Tc Ac Jc Kc Qc");
            let diamond_royal_flush: Vec<Card> = cards_from_str("Qd Kd Td Ad Jd");

            let spade_hand: Hand = Hand::new(spade_royal_flush);
            let heart_hand: Hand = Hand::new(heart_royal_flush);
            let club_hand: Hand = Hand::new(club_royal_flush);
            let diamond_hand: Hand = Hand::new(diamond_royal_flush);

            assert_eq!(spade_hand.kind(), HandKind::RoyalFlush);
            assert_eq!(heart_hand.kind(), HandKind::RoyalFlush);
            assert_eq!(club_hand.kind(), HandKind::RoyalFlush);
            assert_eq!(diamond_hand.kind(), HandKind::RoyalFlush);

            assert_eq!(spade_hand, heart_hand);
            assert_eq!(heart_hand, club_hand);
            assert_eq!(club_hand, diamond_hand);
            assert_eq!(diamond_hand, club_hand);
        }

        #[test]
        fn straight_flush() {
            let ten_high_straight_flush: Hand = Hand::new(cards_from_str("Tc 9c 8c 7c 6c"));
            // Ace is low here
            let five_high_straight_flush: Hand = Hand::new(cards_from_str("Ad 2d 3d 4d 5d"));
            let king_high_straight_flush: Hand = Hand::new(cards_from_str("Qh Th Kh 9h Jh"));
            let ten_high_spade_straight_flush: Hand = Hand::new(cards_from_str("9s Ts 6s 7s 8s"));

            assert_eq!(
                ten_high_straight_flush.kind(),
                HandKind::StraightFlush(Rank::Ten)
            );
            assert_eq!(
                five_high_straight_flush.kind(),
                HandKind::StraightFlush(Rank::Five)
            );
            assert_eq!(
                king_high_straight_flush.kind(),
                HandKind::StraightFlush(Rank::King)
            );
            assert_eq!(
                ten_high_spade_straight_flush.kind(),
                HandKind::StraightFlush(Rank::Ten)
            );

            assert!(ten_high_straight_flush > five_high_straight_flush);
            assert!(five_high_straight_flush < king_high_straight_flush);
            assert!(king_high_straight_flush > ten_high_straight_flush);
            assert_eq!(ten_high_straight_flush, ten_high_spade_straight_flush);
        }

        #[test]
        fn four_of_a_kind() {
            let four_twos: Hand = Hand::new(cards_from_str("2s 2h 2c 2d 8h"));
            let four_aces: Hand = Hand::new(cards_from_str("9d As Ah Ac Ad"));
            let four_eights: Hand = Hand::new(cards_from_str("8s 8h Kc 8c 8d"));

            assert_eq!(four_twos.kind(), HandKind::FourOfAKind(Rank::Two));
            assert_eq!(four_aces.kind(), HandKind::FourOfAKind(Rank::Ace));
            assert_eq!(four_eights.kind(), HandKind::FourOfAKind(Rank::Eight));

            assert!(four_twos < four_aces);
            assert!(four_aces > four_eights);
            assert!(four_eights > four_twos);
        }

        #[test]
        fn full_house() {
            let sevens_over_sixes: Hand = Hand::new(cards_from_str("7h 7d 7c 6s 6c"));
            let sixes_over_sevens: Hand = Hand::new(cards_from_str("6h 7h 6d 7d 6c"));
            let kings_over_aces: Hand = Hand::new(cards_from_str("Ac Kc Kd Kh Ad"));

            assert_eq!(sevens_over_sixes.kind(), HandKind::FullHouse(Rank::Seven));
            assert_eq!(sixes_over_sevens.kind(), HandKind::FullHouse(Rank::Six));
            assert_eq!(kings_over_aces.kind(), HandKind::FullHouse(Rank::King));

            assert!(sevens_over_sixes > sixes_over_sevens);
            assert!(sixes_over_sevens < kings_over_aces);
            assert!(kings_over_aces > sevens_over_sixes);
        }

        #[test]
        fn flush() {
            let seven_high_flush: Hand = Hand::new(cards_from_str("2h 4h 5h 6h 7h"));
            let ace_high_flush: Hand = Hand::new(cards_from_str("As 3s 5s 7s Ks"));
            let ten_high_flush: Hand = Hand::new(cards_from_str("4c 9c Tc 7c 3c"));

            assert_eq!(
                seven_high_flush.kind(),
                HandKind::Flush([Rank::Seven, Rank::Six, Rank::Five, Rank::Four, Rank::Two])
            );
            assert_eq!(
                ace_high_flush.kind(),
                HandKind::Flush([Rank::Ace, Rank::King, Rank::Seven, Rank::Five, Rank::Three])
            );
            assert_eq!(
                ten_high_flush.kind(),
                HandKind::Flush([Rank::Ten, Rank::Nine, Rank::Seven, Rank::Four, Rank::Three])
            );

            assert!(seven_high_flush < ace_high_flush);
            assert!(ace_high_flush > ten_high_flush);
            assert!(ten_high_flush > seven_high_flush);
        }

        #[test]
        fn straight() {
            let nine_high_straight: Hand = Hand::new(cards_from_str("7c 5c 9c 6h 8c"));
            let five_high_straight: Hand = Hand::new(cards_from_str("As 2c 3d 4h 5s"));
            let ace_high_straight: Hand = Hand::new(cards_from_str("Th Kh Jh Qh Ad"));
            let wraparound_isnt_straight: Hand = Hand::new(cards_from_str("Kh Ad 2s 3c 4h"));

            assert_eq!(nine_high_straight.kind(), HandKind::Straight(Rank::Nine));
            assert_eq!(five_high_straight.kind(), HandKind::Straight(Rank::Five));
            assert_eq!(ace_high_straight.kind(), HandKind::Straight(Rank::Ace));
            assert_eq!(
                wraparound_isnt_straight.kind(),
                HandKind::HighCard([Rank::Ace, Rank::King, Rank::Four, Rank::Three, Rank::Two])
            );

            assert!(five_high_straight < ace_high_straight);
            assert!(ace_high_straight > nine_high_straight);
            assert!(nine_high_straight > five_high_straight);
        }

        #[test]
        fn three_of_a_kind() {
            let three_kings: Hand = Hand::new(cards_from_str("Kh Kd Ks 4c 2h"));
            let three_queens: Hand = Hand::new(cards_from_str("7d Qd 6h Qc Qs"));
            let three_twos: Hand = Hand::new(cards_from_str("3h 2s 2d 2c 7h"));

            assert_eq!(three_kings.kind(), HandKind::ThreeOfAKind(Rank::King));
            assert_eq!(three_queens.kind(), HandKind::ThreeOfAKind(Rank::Queen));
            assert_eq!(three_twos.kind(), HandKind::ThreeOfAKind(Rank::Two));

            assert!(three_kings > three_queens);
            assert!(three_queens > three_twos);
            assert!(three_twos < three_kings);
        }

        #[test]
        fn two_pair() {
            let ace_king_queen: Hand = Hand::new(cards_from_str("As Ah Ks Kc Qd"));
            let ace_king_jack: Hand = Hand::new(cards_from_str("Ac Ad Kh Kd Js"));
            let ace_queen_jack: Hand = Hand::new(cards_from_str("As Ah Qs Qh Jh"));
            let king_queen_jack: Hand = Hand::new(cards_from_str("Kh Kd Qd Qc Jc"));

            assert_eq!(
                ace_king_queen.kind(),
                HandKind::TwoPair {
                    pair_high: Rank::Ace,
                    pair_low: Rank::King,
                    high_card: Rank::Queen
                }
            );
            assert_eq!(
                ace_king_jack.kind(),
                HandKind::TwoPair {
                    pair_high: Rank::Ace,
                    pair_low: Rank::King,
                    high_card: Rank::Jack
                }
            );
            assert_eq!(
                ace_queen_jack.kind(),
                HandKind::TwoPair {
                    pair_high: Rank::Ace,
                    pair_low: Rank::Queen,
                    high_card: Rank::Jack
                }
            );
            assert_eq!(
                king_queen_jack.kind(),
                HandKind::TwoPair {
                    pair_high: Rank::King,
                    pair_low: Rank::Queen,
                    high_card: Rank::Jack
                }
            );

            assert!(ace_king_queen > ace_king_jack);
            assert!(ace_king_jack > ace_queen_jack);
            assert!(ace_queen_jack > king_queen_jack);
            assert!(king_queen_jack < ace_king_queen);
        }

        #[test]
        fn pair() {
            let two_three_four_five: Hand = Hand::new(cards_from_str("2h 2d 3s 4s 5s"));
            let two_three_four_six: Hand = Hand::new(cards_from_str("2s 2c 3h 4h 6h"));
            let two_three_five_six: Hand = Hand::new(cards_from_str("2h 2d 3c 5c 6c"));
            let two_four_five_six: Hand = Hand::new(cards_from_str("2s 2c 4d 5d 6d"));
            let three_four_five_six: Hand = Hand::new(cards_from_str("3h 3d 4h 5h 6h"));

            assert_eq!(
                two_three_four_five.kind(),
                HandKind::Pair {
                    pair: Rank::Two,
                    high_cards: [Rank::Five, Rank::Four, Rank::Three]
                }
            );
            assert_eq!(
                two_three_four_six.kind(),
                HandKind::Pair {
                    pair: Rank::Two,
                    high_cards: [Rank::Six, Rank::Four, Rank::Three]
                }
            );
            assert_eq!(
                two_three_five_six.kind(),
                HandKind::Pair {
                    pair: Rank::Two,
                    high_cards: [Rank::Six, Rank::Five, Rank::Three]
                }
            );
            assert_eq!(
                two_four_five_six.kind(),
                HandKind::Pair {
                    pair: Rank::Two,
                    high_cards: [Rank::Six, Rank::Five, Rank::Four]
                }
            );
            assert_eq!(
                three_four_five_six.kind(),
                HandKind::Pair {
                    pair: Rank::Three,
                    high_cards: [Rank::Six, Rank::Five, Rank::Four]
                }
            );

            assert!(two_three_four_five < two_three_four_six);
            assert!(two_three_four_six < two_three_five_six);
            assert!(two_three_five_six < two_four_five_six);
            assert!(two_four_five_six < three_four_five_six);
            assert!(three_four_five_six > two_three_four_five);
        }

        #[test]
        fn high_card() {
            let ten_eight_seven_six_five: Hand = Hand::new(cards_from_str("Td 8c 7c 6c 5c"));
            let ten_eight_seven_six_four: Hand = Hand::new(cards_from_str("Tc 8d 7d 6d 4d"));
            let ten_eight_seven_five_four: Hand = Hand::new(cards_from_str("Th 8s 7s 5s 4s"));
            let ten_eight_six_five_four: Hand = Hand::new(cards_from_str("Ts 8h 6h 5h 4h"));
            let ten_seven_six_five_four: Hand = Hand::new(cards_from_str("Td 7c 6c 5c 4c"));
            let nine_seven_six_five_four: Hand = Hand::new(cards_from_str("9c 7d 6d 5d 4d"));

            assert_eq!(
                ten_eight_seven_six_five.kind(),
                HandKind::HighCard([Rank::Ten, Rank::Eight, Rank::Seven, Rank::Six, Rank::Five])
            );
            assert_eq!(
                ten_eight_seven_six_four.kind(),
                HandKind::HighCard([Rank::Ten, Rank::Eight, Rank::Seven, Rank::Six, Rank::Four])
            );
            assert_eq!(
                ten_eight_seven_five_four.kind(),
                HandKind::HighCard([Rank::Ten, Rank::Eight, Rank::Seven, Rank::Five, Rank::Four])
            );
            assert_eq!(
                ten_eight_six_five_four.kind(),
                HandKind::HighCard([Rank::Ten, Rank::Eight, Rank::Six, Rank::Five, Rank::Four])
            );
            assert_eq!(
                ten_seven_six_five_four.kind(),
                HandKind::HighCard([Rank::Ten, Rank::Seven, Rank::Six, Rank::Five, Rank::Four])
            );
            assert_eq!(
                nine_seven_six_five_four.kind(),
                HandKind::HighCard([Rank::Nine, Rank::Seven, Rank::Six, Rank::Five, Rank::Four])
            );

            assert!(ten_eight_seven_six_five > ten_eight_seven_six_four);
            assert!(ten_eight_seven_six_four > ten_eight_seven_five_four);
            assert!(ten_eight_seven_five_four > ten_eight_six_five_four);
            assert!(ten_eight_six_five_four > ten_seven_six_five_four);
            assert!(ten_seven_six_five_four > nine_seven_six_five_four);
            assert!(nine_seven_six_five_four < ten_eight_seven_six_five);
        }
    }
}
