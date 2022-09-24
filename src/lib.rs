pub struct Solution { }

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_one() {
        let result = Solution::restore_ip_addresses(String::from("25525511135"));

        assert_eq!(result, vec![
            String::from("255.255.11.135"),
            String::from("255.255.111.35")
        ])
    }

    #[test]
    fn acceptance_two() {
        let result = Solution::restore_ip_addresses(String::from("0000"));

        assert_eq!(result, vec![
            String::from("0.0.0.0")
        ])
    }

    #[test]
    fn acceptance_three() {
        let result = Solution::restore_ip_addresses(String::from("101023"));

        assert_eq!(result, vec![
            String::from("1.0.10.23"),
            String::from("1.0.102.3"),
            String::from("10.1.0.23"),
            String::from("10.10.2.3"),
            String::from("101.0.2.3"),
        ])
    }
}
