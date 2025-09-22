use std::cell::{Ref, RefCell, RefMut};

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
    #[inline]
    pub fn rent_car(&self) -> Ref<'_, Car> {
        self.car.borrow()
    }

    #[inline]
    pub fn sell_car(&self) -> Car {
        self.car.take()
    }

    #[inline]
    pub fn repair_car(&self) -> RefMut<'_, Car> {
        self.car.borrow_mut()
    }

    #[inline]
    pub fn change_car(&self, new_car: Car) {
        self.car.replace(new_car);
    }
}
