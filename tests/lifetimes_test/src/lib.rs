#[cfg(test)]
mod tests {
    use lifetimes::*;

    #[test]
    fn create_person() {
        let person = Person::new("Leo");

        assert_eq!(
            person,
            Person {
                age: 0,
                name: "Leo"
            }
        );
    }
}
