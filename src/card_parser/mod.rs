// mod cards;
// use cards::Card;

// pub fn parse_card_from_string(card_text: String) -> Card {
//   let chars = tech.chars();
//   let rank_char: char = chars.next().unwrap();
//   let suit_char: char = chars.next().unwrap();
//   let rank: u8 = match rank_char {
//     'A' => 14,
//     'K' => 13,
//     'Q' => 12,
//     'J' => 11,
//     'T' => 10,
//     _ => |c| c.to_digit(10),
//   };
//   let suit: u8 = match suit_char {
//     's' => 1,
//     'h' => 2,
//     'd' => 4,
//     'c' => 8,
//   }
//   Card(rank, suit)
// }

#[derive(Debug)]
struct Card {
    rank: u8,
    suit: u8,
}

impl From<String> for Card {
    fn from(s: String) -> Card {
        let mut chars = s.chars();
        let (rank, suit) = (chars.next().unwrap(), chars.next().unwrap());

        let rank: u8 = match rank {
            '2'..'9' => rank.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!("{} is not a valid rank", rank),
        };

        let suit: u8 = match suit {
            's' => 1,
            'h' => 2,
            'd' => 4,
            'c' => 8,
            _ => unreachable!("{} is not a valid suit", suit),
        };

        Self { rank, suit }
    }
}