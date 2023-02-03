pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        match (s.len(), p.len()) {
            (l, _) if l < 1 => panic!("s length must be 1 character or greater!"),
            (_, l) if l < 1 => panic!("p length must be 1 character or greater!"),
            (l, _) if l > 20 => panic!("s length must be 20 characters or fewer!"),
            (_, l) if l > 30 => panic!("p length must be 30 characters or fewer!"),
            _ => ()
        }
        let (c_s, c_p) = (s.chars().next(), p.chars().next());
        c_s == c_p || c_p == Some('.')
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_a() {
        assert_eq!(Solution::is_match(String::from("a"), String::from("a")), true);
    }

    #[test]
    fn a_is_not_b() {
        assert_eq!(Solution::is_match(String::from("b"), String::from("a")), false);
    }

    #[test]
    fn dot_matches_any_character() {
        assert_eq!(Solution::is_match(String::from("a"), String::from(".")), true);
        assert_eq!(Solution::is_match(String::from("b"), String::from(".")), true);
        assert_eq!(Solution::is_match(String::from("x"), String::from(".")), true);
    }

    #[test]
    #[should_panic(expected = "s length must be 1 character or greater!")]
    fn validate_s_must_be_length_1_or_more() {
        Solution::is_match(String::from(""), String::from("a"));
    }

    #[test]
    #[should_panic(expected = "p length must be 1 character or greater!")]
    fn validate_p_must_be_length_1_or_more() {
        Solution::is_match(String::from("b"), String::from(""));
    }

    #[test]
    #[should_panic(expected = "s length must be 20 characters or fewer!")]
    fn validate_s_must_be_length_20_or_less() {
        Solution::is_match(String::from("b".repeat(21)), String::from("a"));
    }

    #[test]
    #[should_panic(expected = "p length must be 30 characters or fewer!")]
    fn validate_p_must_be_length_30_or_less() {
        Solution::is_match(String::from("b"), String::from("a".repeat(31)));
    }
}
