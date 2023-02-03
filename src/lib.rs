pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_is_a() {
        assert_eq!(Solution::is_match(String::from("a"), String::from("a")), true);
    }
}
