pub mod mall;

use itertools::Itertools;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    mall.floors
        .values()
        .flat_map(|f| &f.stores)
        .max_by_key(|(_, s)| s.square_meters)
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    mall.floors
        .values()
        .flat_map(|f| f.stores.values().flat_map(|s| &s.employees))
        .max_set_by(|(_, a), (_, b)| a.salary.total_cmp(&b.salary))
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.floors
        .values()
        .flat_map(|f| f.stores.values().flat_map(|s| &s.employees))
        .count()
        + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, available_sec: HashMap<String, Guard>) {
    let total_size = mall.floors.values().map(|f| f.size_limit).sum::<u64>();

    let total_areas = total_size / 200;
    let unguarded_areas = total_areas as usize - mall.guards.len();

    available_sec
        .into_iter()
        .take(unguarded_areas)
        .for_each(|(name, guard)| {
            mall.hire_guard(name, guard);
        });
}

pub fn cut_or_raise(mall: &mut Mall) {
    mall.floors
        .values_mut()
        .flat_map(|f| f.stores.values_mut().flat_map(|s| s.employees.values_mut()))
        .for_each(|e| {
            let shift_hours = e.working_hours.1 - e.working_hours.0;
            if shift_hours >= 10 {
                e.raise(e.salary * 0.1);
            } else {
                e.cut(e.salary * 0.1);
            }
        });
}
