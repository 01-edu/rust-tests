#[cfg(test)]
mod tests {
    use border_cross::*;

    const CARS: [&dyn Vehicle; 4] = [
        &Car {
            plate_nbr: "A3D5C7",
            model: "Model 3",
            horse_power: 325,
            year: 2010,
        },
        &Car {
            plate_nbr: "A785P7",
            model: "S",
            horse_power: 500,
            year: 1980,
        },
        &Car {
            plate_nbr: "D325C7",
            model: "300",
            horse_power: 300,
            year: 2000,
        },
        &Car {
            plate_nbr: "Z3KCH4",
            model: "Montana",
            horse_power: 325,
            year: 2020,
        },
    ];

    const TRUCKS: [&dyn Vehicle; 4] = [
        &Truck {
            plate_nbr: "V3D5CT",
            model: "Ranger",
            horse_power: 325,
            year: 2010,
            load_tons: 40,
        },
        &Truck {
            plate_nbr: "V785PT",
            model: "Silverado",
            horse_power: 500,
            year: 1980,
            load_tons: 40,
        },
        &Truck {
            plate_nbr: "DE2SC7",
            model: "Sierra",
            horse_power: 300,
            year: 2000,
            load_tons: 40,
        },
        &Truck {
            plate_nbr: "3DH432",
            model: "Cybertruck",
            horse_power: 325,
            year: 2020,
            load_tons: 40,
        },
    ];

    #[test]
    fn all_car_models() {
        assert_eq!(all_models(CARS), ["Model 3", "S", "300", "Montana"]);
    }

    #[test]
    fn all_truck_models() {
        assert_eq!(
            all_models(TRUCKS),
            ["Ranger", "Silverado", "Sierra", "Cybertruck"]
        );
    }

    // because `generic_const_exprs` isn't stable yet we need to give the specific case
    fn flatten_3x2<T: Copy>(input: [[T; 4]; 2]) -> [T; 8] {
        let mut out = [input[0][0]; 8];
        let mut i = 0;
        for row in &input {
            for &item in row {
                out[i] = item;
                i += 1;
            }
        }
        out
    }

    #[test]
    fn cars_and_trucks_models() {
        assert_eq!(
            all_models(flatten_3x2([CARS, TRUCKS])),
            [
                "Model 3",
                "S",
                "300",
                "Montana",
                "Ranger",
                "Silverado",
                "Sierra",
                "Cybertruck"
            ]
        );
    }
}
