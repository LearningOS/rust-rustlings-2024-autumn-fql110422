fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    if time_of_day > 23 {
        None
    } else if time_of_day >= 22 {
        Some(0)
    } else {
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // Fix this test to unwrap the Option and get the value
        let icecreams = maybe_icecream(12).unwrap();
        assert_eq!(icecreams, 5);
    }
}
