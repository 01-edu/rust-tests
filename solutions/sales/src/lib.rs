/*
## sales

### Instructions

You will have to create a shopping system, where you will have a :

- Store that will save all the products in it
- Cart that will have `items`, that the client will buy, and a `receipt`

This store is having a promotion, "Buy three and get one for free" (the free item must be the cheapest). The receipt must not present
any value as 0, so you will have to apply the promotion to all items instead.

### Example

`[1.23, 3.12, 23.1]` -> receipt will be `[1.17, 2.98, 22.07]`

So `1.17 + 2.98 + 22.07 == 3.12 + 23.1 + 0`

This is a percentage calculation, and it can be applied to a set of three items.
If the client purchase 9 items it will be applied the promotion, three for free, to all items

|--------------|  |---------------|  |---------------|
`[1.23, 23.1, 3.12, 9.75, 1.75, 23.75, 2.75, 1.64, 15.23]` -> receipt will be `[1.16, 1.55, 1.65, 2.6, 2.94, 9.2, 14.38, 21.8, 22.42]`

|--------|  |--------|   |--------|
`[3.12, 9.75, 1.75, 23.75, 2.75, 1.64, 15.23]` -> receipt will be `[1.54, 1.65, 2.59, 2.94, 9.18, 14.34, 22.36]`

and so on... (hint: Closures is the way)

You will have to implement for the Cart structure the following function:

- `new`, that will initialize the cart
- `insert_item`, that will receive a reference to `Store` and a `String`. Just like the name says you will
  have to insert the item to the cart
- `generate_receipt`, that returns a vector of sorted floats. This function must generate the receipt just
like the example above, using the promotion. Also saving the result in the filed `receipt`.

```rust
fn main() {
    let store = Store::new(vec![
        (String::from("product A"), 1.23),
        (String::from("product B"), 23.1),
        (String::from("product C"), 3.12)]);

    println!("{:?}", store);
    // output:
    // Store { products: [("product A", 1.23), ("product B", 23.1), ("product C", 3.12)] }

    let mut cart = Cart::new();
    cart.insert_item(&store, String::from("product A"));
    cart.insert_item(&store, String::from("product B"));
    cart.insert_item(&store, String::from("product C"));
    println!("{:?}", cart.generate_receipt());
    // output:
    // [1.17, 2.98, 22.07]

    println!("{:?}", cart);
    // output:
    // Cart { items: [("product A", 1.23), ("product B", 23.1), ("product C", 3.12)], receipt: [1.17, 2.98, 22.07] }
}
```

### Notions

- https://doc.rust-lang.org/rust-by-example/fn/closures.html

*/

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let pos = s.products.iter().position(|(x, _)| *x == ele).unwrap();
        self.items.push((ele, s.products[pos].1));
    }

    pub fn get_prices(&self) -> Vec<f32> {
        self.items.iter().map(|(_, v)| *v).collect::<Vec<f32>>()
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices = self.get_prices();
        let cal = self.items.len() / 3;
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let v: Vec<f32> = prices[cal..].to_vec();

        let percentage: f32 =
            (v.iter().sum::<f32>() * 100.0) as f32 / prices.iter().sum::<f32>() as f32;

        self.receipt = prices
            .iter()
            .map(|price| round_two(price * percentage / 100.0))
            .collect::<Vec<f32>>();

        self.receipt.clone()
    }
}

fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Tests {
        store: Store,
        carts: Vec<(Cart, Vec<f32>)>,
    }

    fn add_items(s: &Store, items: Vec<&str>, c: &mut Cart) {
        for item in items.iter() {
            c.insert_item(s, item.to_string());
        }
    }

    impl Tests {
        fn new() -> Tests {
            let store = Store::new(vec![
                (String::from("product A"), 1.23),
                (String::from("product B"), 23.1),
                (String::from("product C"), 3.12),
                (String::from("product D"), 9.75),
                (String::from("product E"), 1.75),
                (String::from("product F"), 23.75),
                (String::from("product G"), 2.75),
                (String::from("product H"), 1.64),
                (String::from("product I"), 15.23),
                (String::from("product J"), 2.10),
                (String::from("product K"), 54.91),
                (String::from("product L"), 43.99),
            ]);

            let mut c = Cart::new();
            let mut c1 = Cart::new();
            let mut c2 = Cart::new();
            let mut c3 = Cart::new();
            add_items(&store, vec!["product A", "product B", "product C"], &mut c);
            let sol = vec![1.17, 2.98, 22.06];
            add_items(
                &store,
                vec![
                    "product A",
                    "product B",
                    "product C",
                    "product D",
                    "product E",
                    "product F",
                    "product G",
                ],
                &mut c1,
            );
            let sol1 = vec![1.17, 1.67, 2.62, 2.98, 9.31, 22.05, 22.67];
            add_items(
                &store,
                vec![
                    "product A",
                    "product B",
                    "product C",
                    "product D",
                    "product E",
                    "product F",
                    "product G",
                    "product H",
                    "product I",
                ],
                &mut c2,
            );
            let sol2 = vec![1.16, 1.55, 1.65, 2.6, 2.94, 9.2, 14.38, 21.8, 22.42];
            add_items(
                &store,
                vec![
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
                ],
                &mut c3,
            );
            let sol3 = vec![
                1.18, 1.58, 1.69, 2.02, 2.65, 3.01, 9.39, 14.67, 22.25, 22.88, 42.38, 52.9,
            ];

            Tests {
                store,
                carts: vec![(c, sol), (c1, sol1), (c2, sol2), (c3, sol3)],
            }
        }
    }

    #[test]
    fn test_generate_receipt() {
        let cases = Tests::new();

        for (mut c, sol) in cases.carts.into_iter() {
            assert_eq!(c.generate_receipt(), sol);
            assert_eq!(c.receipt, sol);
        }
    }
}
