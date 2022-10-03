// Create the trait Vehicle with the model and year

// In a border cross you want to keep a list of all the vehicles that
// are waiting to enter the country. You want to keep a waiting list
// of the vehicle but the vehicles can be of two types: Car or Truck
// With the following structure:
#[allow(dead_code)]
pub struct Car<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
}

#[allow(dead_code)]
pub struct Truck<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
    pub load_tons: u32,
}

pub trait Vehicle {
    fn model(&self) -> &str;
    fn year(&self) -> u32;
}

impl Vehicle for Truck<'_> {
    fn model(&self) -> &str {
        self.model
    }

    fn year(&self) -> u32 {
        self.year
    }
}

impl Vehicle for Car<'_> {
    fn model(&self) -> &str {
        self.model
    }
    fn year(&self) -> u32 {
        self.year
    }
}

// create a function that receives a vector of structures that
// implement the Vehicle trait

#[allow(dead_code)]
pub fn all_models(list: Vec<&dyn Vehicle>) -> Vec<&str> {
    let mut models = Vec::new();
    for ve in list {
        models.push(ve.model());
    }
    models
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_cars<'a>() -> Vec<&'a dyn Vehicle> {
        let cars: Vec<&dyn Vehicle> = vec![
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
        cars
    }

    fn setup_trucks<'a>() -> Vec<&'a dyn Vehicle> {
        let trucks: Vec<&dyn Vehicle> = vec![
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
        trucks
    }

    #[test]
    fn all_car_models() {
        let cars = setup_cars();
        assert_eq!(all_models(cars), ["Model 3", "S", "300", "Montana"]);
    }

    #[test]
    fn all_truck_models() {
        let trucks = setup_trucks();
        assert_eq!(
            all_models(trucks),
            ["Ranger", "Silverado", "Sierra", "Cybertruck"]
        );
    }

    #[test]
    fn cars_and_trucks_models() {
        let mut vehicles = setup_cars();
        vehicles.extend(setup_trucks());
        assert_eq!(
            all_models(vehicles),
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
