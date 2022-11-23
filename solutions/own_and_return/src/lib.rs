pub struct Film {
    pub name: String,
}

pub fn take_film_name(to_consume: Film) -> String {
    to_consume.name
}

pub fn read_film_name(only_to_return: &Film) -> String {
    only_to_return.name.clone()
}
