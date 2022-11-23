use print_and_consume::*;

fn main() {
    println!(
        "{}",
        own_and_return(Film {
            name: "Casablanca".to_string()
        })
    );
    println!(
        "{}",
        only_return(&Film {
            name: "Casablanca".to_string()
        })
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume() {
        println!(
            "{}",
            own_and_return(Film {
                name: "Matrix".to_string()
            })
        );
    }
    #[test]
    fn test_only_print() {
        println!(
            "{}",
            only_return(&Film {
                name: "Matrix".to_string()
            })
        );
    }
}
