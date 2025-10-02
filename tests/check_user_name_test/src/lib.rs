#[cfg(test)]
mod tests {
    use check_user_name::*;

    #[test]
    fn test_error() {
        let guest_case = User::new("Michael".to_owned(), AccessLevel::Guest);
        assert_eq!(
            check_user_name(&guest_case),
            (false, "ERROR: User is guest")
        );
    }

    #[test]
    fn test_ok() {
        let admin_case = User::new("Fatima".to_owned(), AccessLevel::Admin);
        let normal_case = User::new("Sara".to_owned(), AccessLevel::Normal);
        assert_eq!(check_user_name(&admin_case), (true, "Fatima"));
        assert_eq!(check_user_name(&normal_case), (true, "Sara"));
    }

    #[test]
    fn test_send_name() {
        let admin_case = User::new("Fatima".to_owned(), AccessLevel::Admin);
        let normal_case = User::new("Sara".to_owned(), AccessLevel::Normal);
        let guest_case = User::new("Michael".to_owned(), AccessLevel::Guest);
        assert_eq!(admin_case.send_name(), Some("Fatima"));
        assert_eq!(normal_case.send_name(), Some("Sara"));
        assert_eq!(guest_case.send_name(), None);
    }
}
