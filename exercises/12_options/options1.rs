fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day > 23 {
        None
    } else if hour_of_day < 22 {
        Some(5)
    } else {
        Some(0)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        let ice_creams = maybe_ice_cream(12).unwrap();
        assert_eq!(ice_creams, 5);
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}