use check_user_name::*;

fn main() {
    let user0 = User::new("Didier".to_string(), AccessLevel::Admin);
    println!("{:?}", check_user_name(&user0));

    let user2 = User::new("Mary".to_string(), AccessLevel::Normal);
    println!("{:?}", check_user_name(&user2));

    let user3 = User::new("John".to_string(), AccessLevel::Guest);
    println!("{:?}", check_user_name(&user3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let u = User::new("Michael".to_string(), AccessLevel::Guest);
        let (t, _) = check_user_name(&u);
        assert!(!t);
    }

    #[test]
    fn test_ok() {
        let v = vec![
            User::new("Fatima".to_string(), AccessLevel::Admin),
            User::new("Sara".to_string(), AccessLevel::Normal),
        ];
        for value in v {
            let (t, _) = check_user_name(&value);
            assert!(t);
        }
    }
}
