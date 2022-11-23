use own_and_return::*;

fn main() {
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume() {
        println!(
            "{}",
            take_film_name(Film {
                name: "Matrix".to_string()
            })
        );
    }
    #[test]
    fn test_only_print() {
        println!(
            "{}",
            read_film_name(&Film {
                name: "Matrix".to_string()
            })
        );
    }
}
