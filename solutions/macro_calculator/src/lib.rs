use std::array;

use json::object;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let [kcals, fats, carbs, proteins] = foods
        .iter()
        .map(|f| {
            let kcals = f.calories.1.split_terminator("kcal").next().unwrap();
            let kcals = kcals.parse().unwrap();

            [kcals, f.fats, f.carbs, f.proteins].map(|v| v * f.nbr_of_portions)
        })
        .reduce(|acc, next| array::from_fn(|i| acc[i] + next[i]))
        .unwrap_or_default()
        .map(|v| (v * 100.0).round() / 100.0);

    object! {
        cals: kcals,
        carbs: carbs,
        proteins: proteins,
        fats: fats
    }
}
