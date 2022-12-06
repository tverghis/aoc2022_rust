use std::collections::HashSet;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", find_first_marker(INPUT, 4));
    println!("Part 2: {}", find_first_marker(INPUT, 14));
}

fn find_first_marker(input: &str, window_size: usize) -> usize {
    let (window_start_idx, _) = input
        .chars()
        .collect::<Vec<_>>()
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| slice_is_unique(window))
        .unwrap();

    window_start_idx + window_size
}

fn slice_is_unique(s: &[char]) -> bool {
    let mut set = HashSet::new();
    s.iter().all(|c| set.insert(c))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_first_start_of_packet_marker() {
        assert_eq!(find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_find_first_start_of_message_marker() {
        assert_eq!(find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
