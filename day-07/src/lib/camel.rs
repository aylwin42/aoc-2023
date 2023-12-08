use core::fmt;
use std::{fmt::Formatter, str::FromStr, cmp::Ordering};

use crate::Error;

/// Camel Cards (Advent of Code 2023 - Day 07)
/// 

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Copy,Clone)]
pub enum Card {
    Joker = 0,
    Num2 = 1,
    Num3 = 2,
    Num4 = 3,
    Num5 = 4,
    Num6 = 5,
    Num7 = 6,
    Num8 = 7,
    Num9 = 8,
    Num10 = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Card::Joker |
            Card::Jack => write!(f, "J"),
            Card::Num2 => write!(f, "2"),
            Card::Num3 => write!(f, "3"),
            Card::Num4 => write!(f, "4"),
            Card::Num5 => write!(f, "5"),
            Card::Num6 => write!(f, "6"),
            Card::Num7 => write!(f, "7"),
            Card::Num8 => write!(f, "8"),
            Card::Num9 => write!(f, "9"),
            Card::Num10 => write!(f, "T"),
            Card::Queen => write!(f, "Q"),
            Card::King => write!(f, "K"),
            Card::Ace => write!(f, "A"),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Num2),
            '3' => Ok(Card::Num3),
            '4' => Ok(Card::Num4),
            '5' => Ok(Card::Num5),
            '6' => Ok(Card::Num6),
            '7' => Ok(Card::Num7),
            '8' => Ok(Card::Num8),
            '9' => Ok(Card::Num9),
            'T' => Ok(Card::Num10),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(Error::ParseError)
        }
    }
}

impl Card {
    #[must_use]
    pub fn with_jokers(&self) -> Self {
        if *self == Card::Jack {
            Card::Joker
        } else {
            *self
        }
    }
}

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd)]
pub enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug,Copy,Clone,Eq)]
pub struct Hand([Card;5]);

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}",self.0[0],self.0[1],self.0[2],self.0[3],self.0[4])
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(hand_str: &str) -> Result<Self, Self::Err> {
        Ok(Hand(
            hand_str.chars()
                .map(Card::try_from)
                .collect::<Result<Vec<_>,_>>()?
                .try_into().or(Err(Error::ParseError))?
        ))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.to_type() == other.to_type()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.to_type().cmp(&other.to_type()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.0.cmp(&other.0),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {

    /// # Panics
    ///     Panics if the HandType is not known (should never happen)
    #[must_use]
    pub fn to_type(&self) -> HandType {
        let mut tally = [0u8;14];

        self.0.iter().for_each(|card| tally[*card as usize] += 1);
        
        let wild_cards = tally[0];

        let mut tally = tally[1..].to_vec();
        tally.sort_unstable();

        match (tally[12]+wild_cards,tally[11]) {
            (5,0) => HandType::FiveOfAKind,
            (4,0|1) => HandType::FourOfAKind,
            (3,2) => HandType::FullHouse,
            (3,1) => HandType::ThreeOfAKind,
            (2,2) => HandType::TwoPair,
            (2,1) => HandType::OnePair,
            (1,1) => HandType::HighCard,
            value => panic!("This combinination should not happen: {value:?} {self} ({tally:?})")
        }
    }

    // Convert the Jacks to Jokers
    #[must_use]
    pub fn with_jokers(&self) -> Self {
        Self([
            self.0[0].with_jokers(),
            self.0[1].with_jokers(),
            self.0[2].with_jokers(),
            self.0[3].with_jokers(),
            self.0[4].with_jokers(),            
        ])
    }
}
