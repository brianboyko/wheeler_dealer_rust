#[derive(Debug)]
pub struct Card(u8, u8); // rank, suit;

pub fn make_deck () -> Vec<Card> {
  let ranks: [u8; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]; // 2-9TJQKA
  let suits: [u8; 4] = [1, 2, 4, 8]; // shdc
  let mut deck: Vec<Card> = vec![];
  for suit in &suits {
    for rank in &ranks {
      deck.push(Card(*rank, *suit));
    }
  }
  deck
}

#[test]
fn it_makes_a_deck() {
  let test_results = make_deck();
  assert_eq!(
      test_results.len(),
      52
  );
  assert_eq!(test_results[0].0, 2);
  assert_eq!(test_results[0].1, 1);
  assert_eq!(test_results[51].0, 14);
  assert_eq!(test_results[51].1, 8);
}
