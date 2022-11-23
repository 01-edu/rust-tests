pub struct Film {
    pub name: String,
}

pub fn own_and_return(to_consume: Film) -> String {
    to_consume.name
}

pub fn only_return(only_to_return: &Film) -> String {
    only_to_return.name.clone()
}
