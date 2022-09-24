pub struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = String::new();
        let mut frontier = vec![(1, String::from(s.chars().nth(0).unwrap()))];

        while !frontier.is_empty() {
            let (index, candidate) = frontier.pop().unwrap();

            let last_block = Solution::extract_block(&candidate);
            if Solution::is_valid_block(last_block) {
                // Extract into method and test
                let (candidate1, candidate2, new_index) =
                    Solution::get_next_generation(&s, index, candidate);
                // check if end of string. If candidate valid then add to results
                frontier.push((new_index, candidate1));
                frontier.push((new_index, candidate2));
            }
        }

        vec![result]
    }

    fn extract_block(candidate: &String) -> &str {
        ""
    }

    fn get_next_generation(
        s: &String,
        current_index: usize,
        branch_state: String,
    ) -> (String, String, usize) {
        let next_char = &s.as_str()[(current_index..current_index + 1)];
        (
            format!("{branch_state}{next_char}"),
            format!("{branch_state}.{next_char}"),
            current_index + 1,
        )
    }

    fn is_valid_block(block: &str) -> bool {
        let first_digit = &block[0..1];

        if first_digit == "0" && block.len() > 1 {
            return false;
        }

        let number = usize::from_str_radix(block, 10).unwrap();
        number < 255
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn acceptance_one() {
        let result = Solution::restore_ip_addresses(String::from("25525511135"));

        assert_eq!(
            result,
            vec![
                String::from("255.255.11.135"),
                String::from("255.255.111.35")
            ]
        )
    }

    #[test]
    fn acceptance_two() {
        let result = Solution::restore_ip_addresses(String::from("0000"));

        assert_eq!(result, vec![String::from("0.0.0.0")])
    }

    #[test]
    #[ignore]
    fn acceptance_three() {
        let result = Solution::restore_ip_addresses(String::from("101023"));

        assert_eq!(
            result,
            vec![
                String::from("1.0.10.23"),
                String::from("1.0.102.3"),
                String::from("10.1.0.23"),
                String::from("10.10.2.3"),
                String::from("101.0.2.3"),
            ]
        )
    }

    // #[test]
    // fn five_characters() {
    //     let result = Solution::restore_ip_addresses(String::from("10000"));

    //     assert_eq!(result, vec![
    //         String::from("10.0.0.0")
    //     ])
    // }

    // #[test]
    // fn get_all_branches() {
    //     let result = Solution::restore_ip_addresses(String::from("11111"));

    //     assert_eq!(
    //         result,
    //         vec![
    //             String::from("11.1.1.1"),
    //             String::from("1.11.1.1"),
    //             String::from("1.1.11.1"),
    //             String::from("1.1.1.11"),
    //         ]
    //     )
    // }

    #[test]
    fn branch_single_character_generation_ones() {
        let (result1, result2, index) =
            Solution::get_next_generation(&String::from("11"), 1, String::from("1"));

        assert_eq!(result1, String::from("11"));
        assert_eq!(result2, String::from("1.1"));
        assert_eq!(index, 2);
    }

    #[test]
    fn branch_single_character_generation_twos() {
        let (result1, result2, index) =
            Solution::get_next_generation(&String::from("22"), 1, String::from("2"));

        assert_eq!(result1, String::from("22"));
        assert_eq!(result2, String::from("2.2"));
        assert_eq!(index, 2);
    }

    #[test]
    fn validate_block_in_range() {
        let result = Solution::is_valid_block("11");

        assert_eq!(true, result);
    }

    #[test]
    fn validate_block_outside_range() {
        let result = Solution::is_valid_block("256");

        assert_eq!(false, result);
    }

    #[test]
    fn validate_block_leading_zero() {
        let result = Solution::is_valid_block("01");

        assert_eq!(false, result);
    }

    #[test]
    fn validate_block_zero() {
        let result = Solution::is_valid_block("0");

        assert_eq!(true, result);
    }
}
