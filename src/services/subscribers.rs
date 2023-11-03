pub trait Counter {
    fn quantity(&self) -> usize;

    /// Brings the quantity into a visually beautiful format (like 1000 to 1k)
    fn show_quantity(&self) -> String {
        let (number, add_char) = match self.quantity() {
            // >1 000 show without adding designations
            x @ 0..=999_usize => return format!("{}", x),
            // 1000+ show with adding designations
            x @ 0..=999_999_usize => (x as f64 / 1e+3, 'K'),
            x @ 0..=999_999_999_usize => (x as f64 / 1e+6, 'M'),
            x => (x as f64 / 1e+9, 'B'),
        };
        // remove the fractional part if it is equal to zero after the union
        format!("{:.2}{}", number, add_char).replace(".00", "")
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;

    struct Test {nb: usize}

    impl Counter for Test {
        fn quantity(&self) -> usize { self.nb }
    }

    #[test]
    fn number_556() {
        let input_data = Test {nb: 556};
        let expected_result = String::from("556");
        assert_eq!(expected_result, input_data.show_quantity())
    }

    #[test]
    fn number_4000() {
        let input_data = Test {nb: 4000};
        let expected_result = String::from("4K");
        assert_eq!(expected_result, input_data.show_quantity())
    }

    #[test]
    fn number_3957() {
        let input_data = Test {nb: 3957};
        let expected_result = String::from("3.96K");
        assert_eq!(expected_result, input_data.show_quantity())
    }

    #[test]
    fn number_987005() {
        let input_data = Test {nb: 987005};
        let expected_result = String::from("987K");
        assert_eq!(expected_result, input_data.show_quantity())
    }

    #[test]
    fn number_489999517() {
        let input_data = Test {nb: 489999517};
        let expected_result = String::from("490M");
        assert_eq!(expected_result, input_data.show_quantity())
    }

    #[test]
    fn number_18787871565() {
        let input_data = Test {nb: 18787871565};
        let expected_result = String::from("18.79B");
        assert_eq!(expected_result, input_data.show_quantity())
    }

    #[test]
    fn number_97000087156() {
        let input_data = Test {nb: 97000087156};
        let expected_result = String::from("97B");
        assert_eq!(expected_result, input_data.show_quantity())
    }

}