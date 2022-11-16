use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Car {
    pub color: String,
    pub plate: String,
}

#[derive(Debug)]
pub struct RentalBusiness {
    pub car: RefCell<Car>,
}

impl RentalBusiness {
    pub fn rent_car(&self) -> Ref<Car> {
        self.car.borrow()
    }

    pub fn sell_car(&self) -> Car {
        self.car.take()
    }

    pub fn repair_car(&self) -> RefMut<Car> {
        self.car.borrow_mut()
    }

    pub fn change_car(&self, new_car: Car) {
        self.car.replace(new_car);
    }
}
