#[cfg(test)]
mod tests {
    use sales::*;

    const STORE: Store<12> = Store::new([
        Item("product A", 1.23),
        Item("product B", 23.1),
        Item("product C", 3.12),
        Item("product D", 9.75),
        Item("product E", 1.75),
        Item("product F", 23.75),
        Item("product G", 2.75),
        Item("product H", 1.64),
        Item("product I", 15.23),
        Item("product J", 2.10),
        Item("product K", 54.91),
        Item("product L", 43.99),
    ]);

    #[test]
    fn test_invalid_product() {
        let mut cart = Cart::new();

        assert_eq!(Err(()), cart.insert_item_by_name(&STORE, "product Z"));
    }

    #[test]
    fn test_empty_cart() {
        let cart = Cart::new();

        assert_eq!(cart.generate_prices(), []);
    }

    #[test]
    fn test_cart_one() {
        let mut cart = Cart::new();

        for p in ["product A", "product B", "product C"] {
            cart.insert_item_by_name(&STORE, p).unwrap();
        }

        assert_eq!(cart.generate_prices(), [1.17, 2.98, 22.06]);
    }

    #[test]
    fn test_cart_two() {
        let mut cart = Cart::new();

        for p in [
            "product A",
            "product B",
            "product C",
            "product D",
            "product E",
            "product F",
            "product G",
        ] {
            cart.insert_item_by_name(&STORE, p).unwrap();
        }

        assert_eq!(
            cart.generate_prices(),
            [1.17, 1.67, 2.62, 2.98, 9.31, 22.05, 22.67]
        );
    }

    #[test]
    fn test_cart_three() {
        let mut cart = Cart::new();

        for p in [
            "product A",
            "product B",
            "product C",
            "product D",
            "product E",
            "product F",
            "product G",
            "product H",
            "product I",
        ] {
            cart.insert_item_by_name(&STORE, p).unwrap();
        }

        assert_eq!(
            cart.generate_prices(),
            [1.16, 1.55, 1.65, 2.6, 2.94, 9.2, 14.38, 21.8, 22.42]
        );
    }

    #[test]
    fn test_cart_four() {
        let mut cart = Cart::new();

        for p in [
            "product A",
            "product B",
            "product C",
            "product D",
            "product E",
            "product F",
            "product G",
            "product H",
            "product I",
            "product J",
            "product K",
            "product L",
        ] {
            cart.insert_item_by_name(&STORE, p).unwrap();
        }

        assert_eq!(
            cart.generate_prices(),
            [
                1.18, 1.58, 1.69, 2.02, 2.65, 3.01, 9.39, 14.67, 22.25, 22.88, 42.38, 52.9,
            ]
        );
    }
}
