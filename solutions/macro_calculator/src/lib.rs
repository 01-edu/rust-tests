/*
## macro_calculator

### Instructions

Create a function `calculate_macros` that receives a vector of `serde_json::Value` and returns a `serde_json::Value`.

The vector you will receive will contain jsons in the following format:

```json
{
    "name": <name>,
    "calories": [<value_in_kJ>, <value_in_kcal>],
    "fats": <fats_in_g>,
    "carbs": <carbs_in_g>,
    "proteins": <proteins_in_g>,
    "nbr_of_portions": <portions>
}
```

Besides the name and the content of the calories array, that are strings, everything else are floats.

As the result of the function you should return a json with the following format (Macros struct):

```json
    "cals": <calories>,
    "carbs": <carbs>,
    "proteins": <proteins>,
    "fats": <fats>,
```

The number of portions should be taken into account. The values of the macros refer to one portion.
All values should be floats (f64) and should be the addition of every macronutrient in the provided array (cals is the addition of every calories, fats is the addition of every fats, and so on...).
Every value should be rounded to two decimal places (ex: 12.29) or one decimal place if it ends in 0 (ex: 18.90 -> 18.9).

Hint: You will need the `serde`, `serde_json` and `serde_derive` crates.

### Example

```rust
fn main() {
    let a = vec![
        Food {
            name: String::from("big mac"),
            calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
            protein: 27.0,
            fats: 26.0,
            carbs: 41.0,
            nbr_of_portions: 2.0,
        },
        Food {
            name: "pizza margherita".to_string(),
            calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
            protein: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
    ];

    println!("{:#}", calculate_macros(a));
}
```

*/

extern crate json;

use json::object;

pub struct Food {
    #[allow(dead_code)]
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let (mut cals, mut prot, mut carbs, mut fats) = (0.0, 0.0, 0.0, 0.0);

    for food in foods {
        let cal = &food.calories[1]
            .to_string()
            .split("kcal")
            .collect::<Vec<&str>>()[0]
            .to_string()
            .parse::<f64>()
            .unwrap();

        cals += cal * food.nbr_of_portions;
        prot += food.proteins * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    object! {
        "cals": (cals * 100.0).round() / 100.0,
        "carbs": (carbs * 100.0).round() / 100.0,
        "proteins": (prot * 100.0).round() / 100.0,
        "fats": (fats * 100.0).round() / 100.0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing_macros_values() {
        let a = Food {
            name: "light milk".to_string(),
            calories: ["148kJ".to_string(), "35kcal".to_string()],
            proteins: 3.5,
            fats: 0.1,
            carbs: 5.0,
            nbr_of_portions: 0.7,
        };
        let b = Food {
            name: "oat cookies".to_string(),
            calories: ["1996kJ".to_string(), "477kcal".to_string()],
            proteins: 8.2,
            fats: 21.0,
            carbs: 60.4,
            nbr_of_portions: 1.2,
        };

        let macros = calculate_macros(vec![a, b]);

        assert_eq!(macros["cals"], 596.9);
        assert_eq!(macros["carbs"], 75.98);
        assert_eq!(macros["proteins"], 12.29);
        assert_eq!(macros["fats"], 25.27);
    }

    #[test]
    fn testing_no_food() {
        let macros = calculate_macros(vec![]);

        assert_eq!(macros["cals"], 0.0);
        assert_eq!(macros["carbs"], 0.0);
        assert_eq!(macros["proteins"], 0.0);
        assert_eq!(macros["fats"], 0.0);
    }

    #[test]
    fn big_values() {
        let macros = calculate_macros(vec![
            Food {
                name: "big mac".to_string(),
                calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
                proteins: 27.0,
                fats: 26.0,
                carbs: 41.0,
                nbr_of_portions: 2.0,
            },
            Food {
                name: "pizza margherita".to_string(),
                calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
                proteins: 13.89,
                fats: 11.21,
                carbs: 49.07,
                nbr_of_portions: 4.9,
            },
        ]);

        assert_eq!(macros["cals"].as_f64().unwrap(), 2777.39);
        assert_eq!(macros["carbs"].as_f64().unwrap(), 322.44);
        assert_eq!(macros["proteins"].as_f64().unwrap(), 122.06);
        assert_eq!(macros["fats"].as_f64().unwrap(), 106.93);
    }
}
