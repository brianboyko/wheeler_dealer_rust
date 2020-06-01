fn combine(n: u8, k: u8, current_number: u8, results: &mut Vec<Vec<u8>>, stack: &mut Vec<u8>) {
    if stack.len() == k as usize {
        results.push(stack.clone());
        return;
    }
    if current_number > n {
        return;
    }
    stack.push(current_number);
    combine(n, k, current_number + 1, results, stack);
    stack.pop();
    combine(n, k, current_number + 1, results, stack);
}

pub fn list_combos(n: u8, k: u8) -> Vec<Vec<u8>> {
    let mut results: Vec<Vec<u8>> = vec![];
    let mut stack: Vec<u8> = vec![];
    combine(n, k, 0, &mut results, &mut stack);
    results
}

#[test]
fn it_enumerates_combos() {
    let test_results = list_combos(4, 2);
    assert_eq!(
        test_results,
        vec![
            [0, 1],
            [0, 2],
            [0, 3],
            [0, 4],
            [1, 2],
            [1, 3],
            [1, 4],
            [2, 3],
            [2, 4],
            [3, 4]
        ]
    );
    let all_poker_hands = list_combos(52, 5);
    assert_eq!(all_poker_hands.len(), 2869685)
}
