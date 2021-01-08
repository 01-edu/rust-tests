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
    let a = serde_json::json!(
        {
        "name": "light milk",
        "calories": ["148kJ", "35kcal"],
        "protein": 3.5,
        "fats": 0.1,
        "carbs": 5.0,
        "nbr_of_portions": 0.7
    });

    let b = serde_json::json!({
        "name": "oat cookies",
        "calories": ["1996kJ", "477kcal"],
        "protein": 8.2,
        "fats": 21.0,
        "carbs": 60.4,
        "nbr_of_portions": 1.2,
    });

    let macros: Macros = serde_json::from_value(calculate_macros(vec![a, b])).unwrap();
    println!("{:?}", macros);
}
```
*/

extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Macros {
    pub cals: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub fats: f64,
}

pub fn calculate_macros(foods: Vec<serde_json::Value>) -> serde_json::Value {
    let (mut cals, mut prot, mut carbs, mut fats) = (0.0, 0.0, 0.0, 0.0);

    for food in foods {
        let cal = &food["calories"][1]
            .to_string()
            .split("kcal")
            .collect::<Vec<&str>>()[0]
            .split("\"")
            .collect::<Vec<&str>>()[1]
            .to_string()
            .parse::<f64>()
            .unwrap();

        cals += cal * food["nbr_of_portions"].as_f64().unwrap();
        prot += food["protein"].as_f64().unwrap() * food["nbr_of_portions"].as_f64().unwrap();
        carbs += food["carbs"].as_f64().unwrap() * food["nbr_of_portions"].as_f64().unwrap();
        fats += food["fats"].as_f64().unwrap() * food["nbr_of_portions"].as_f64().unwrap();
    }

    serde_json::json!({
        "cals": (cals * 100.0).round() / 100.0,
        "carbs": (carbs * 100.0).round() / 100.0,
        "proteins": (prot * 100.0).round() / 100.0,
        "fats": (fats * 100.0).round() / 100.0,
    })
}
/*
fn main() {
    let a = serde_json::json!(
        {
        "name": "light milk",
        "calories": ["148kJ", "35kcal"],
        "protein": 3.5,
        "fats": 0.1,
        "carbs": 5.0,
        "nbr_of_portions": 0.7
    });

    let b = serde_json::json!({
        "name": "oat cookies",
        "calories": ["1996kJ", "477kcal"],
        "protein": 8.2,
        "fats": 21.0,
        "carbs": 60.4,
        "nbr_of_portions": 1.2,
    });

    let macros: Macros = serde_json::from_value(calculate_macros(vec![a, b])).unwrap();
    println!("{:?}", macros);
}
*/
#[cfg(test)]
mod test {
    use super::*;

    impl PartialEq for Macros {
        fn eq(&self, other: &Macros) -> bool {
            self.carbs == other.carbs
                && self.cals == other.cals
                && self.proteins == other.proteins
                && self.fats == other.fats
        }
    }

    #[test]
    fn testing_macros_values() {
        let a = serde_json::json!(
            {
            "name": "light milk",
            "calories": ["148kJ", "35kcal"],
            "protein": 3.5,
            "fats": 0.1,
            "carbs": 5.0,
            "nbr_of_portions": 0.7
        });
        let b = serde_json::json!({
            "name": "oat cookies",
            "calories": ["1996kJ", "477kcal"],
            "protein": 8.2,
            "fats": 21.0,
            "carbs": 60.4,
            "nbr_of_portions": 1.2,
        });

        let macros: Macros = serde_json::from_value(calculate_macros(vec![a, b])).unwrap();

        assert_eq!(
            Macros {
                cals: 596.9,
                carbs: 75.98,
                proteins: 12.29,
                fats: 25.27
            },
            macros
        );
    }

    #[test]
    fn testing_no_food() {
        let macros: Macros = serde_json::from_value(calculate_macros(vec![])).unwrap();

        assert_eq!(
            Macros {
                cals: 0.0,
                carbs: 0.0,
                proteins: 0.0,
                fats: 0.0
            },
            macros
        );
    }

    #[test]
    fn big_values() {
        let macros: Macros = serde_json::from_value(calculate_macros(vec![
            serde_json::json!({
                "name": "big mac",
                "calories": ["2133.84kJ", "510kcal"],
                "protein": 27,
                "fats": 26,
                "carbs": 41,
                "nbr_of_portions": 2,
            }),
            serde_json::json!({
                "name": "pizza margherita",
                "calories": ["1500.59kJ", "358.65kcal"],
                "protein": 13.89,
                "fats": 11.21,
                "carbs": 49.07,
                "nbr_of_portions": 4.9,
            }),
        ]))
        .unwrap();

        assert_eq!(
            Macros {
                cals: 2777.39,
                carbs: 322.44,
                proteins: 122.06,
                fats: 106.93
            },
            macros
        );
    }
}
