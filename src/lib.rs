pub struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = String::new();
        //frontier Vec<(String, String, usize)>

        for (i, char) in s.chars().enumerate() {
            //if the current node is not valid, return nothing
            //If there are no more characters from `s` to add, add this to the result
            //Generate the next generation

            if i > 0 {
                result.push('.');
            }
            result.push(char);
        }

        vec![result]
    }

    fn get_next_generation(
        s: String,
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
            Solution::get_next_generation(String::from("11"), 1, String::from("1"));

        assert_eq!(result1, String::from("11"));
        assert_eq!(result2, String::from("1.1"));
        assert_eq!(index, 2);
    }

    #[test]
    fn branch_single_character_generation_twos() {
        let (result1, result2, index) =
            Solution::get_next_generation(String::from("22"), 1, String::from("2"));

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
