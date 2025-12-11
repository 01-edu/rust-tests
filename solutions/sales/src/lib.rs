#![feature(sort_floats)]

const ONE_FREE_EVERY: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item<'a>(pub &'a str, pub f64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Store<'a, const N: usize> {
    pub products: [Item<'a>; N],
}

impl<'a, const N: usize> Store<'a, N> {
    pub const fn new(products: [Item<'a>; N]) -> Self {
        Self { products }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Cart<'a> {
    pub items: Vec<Item<'a>>,
}

impl<'a> Cart<'a> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_item_by_name<const N: usize>(
        &mut self,
        s: &Store<'a, N>,
        name: &str,
    ) -> Result<(), ()> {
        let item = s.products.iter().copied().find(|p| p.0 == name).ok_or(())?;
        self.items.push(item);

        Ok(())
    }

    pub fn generate_prices(&self) -> Vec<f64> {
        let mut prices = self
            .items
            .iter()
            .copied()
            .map(|Item(_, v)| v)
            .collect::<Vec<_>>();
        prices.sort_floats();

        let all_total = prices.iter().sum::<f64>();
        let non_discount_total = prices
            .iter()
            .skip(prices.len() / ONE_FREE_EVERY)
            .sum::<f64>();

        let percentage = (non_discount_total * 100.) / all_total;

        prices
            .into_iter()
            .map(|p| (p * percentage).round() / 100.)
            .collect()
    }
}
