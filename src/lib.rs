pub struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = String::new();

        for (i, char) in s.chars().enumerate() {
            if i > 0 {
                result.push('.');
            }
            result.push(char);
        }

        vec![result]
    }

    pub fn get_next_generation(
        s: String,
        current_index: usize,
        branch_state: String,
    ) -> (Vec<String>, usize) {
        (vec![String::new()], current_index)
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
    fn branch_single_character_generation() {
        let (result, index) = Solution::get_next_generation(
            String::from("11"),
            1,
            String::from("1"));

        assert_eq!(result, vec![String::from("11"), String::from("1.1")]);
        assert_eq!(index, 1);
    }
}
