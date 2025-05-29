use box_recursion::*;

#[test]
fn test_new() {
    let list = WorkEnvironment::new();
    assert!(list.grade.is_none());
}

#[test]
fn test_one_worker() {
    let mut list = WorkEnvironment::new();
    list.add_worker("Marie", "CEO");
    list.remove_worker();
    assert!(list.grade.is_none());
}

#[test]
fn test_two_workers() {
    let mut list = WorkEnvironment::new();
    list.add_worker("Marie", "CEO");
    list.add_worker("Monica", "Manager");
    list.remove_worker();

    assert_eq!(list.grade.as_ref().unwrap().role, Role::CEO);
    assert_eq!(list.grade.as_ref().unwrap().name, "Marie".to_owned());
}

#[test]
fn test_more_workers() {
    let mut list = WorkEnvironment::new();
    list.add_worker("Marie", "CEO");
    list.add_worker("Monica", "Manager");
    list.add_worker("Ana", "Normal Worker");
    list.add_worker("Alice", "Normal Worker");
    list.remove_worker();

    assert_eq!(list.grade.as_ref().unwrap().role, Role::Worker);
    assert_eq!(list.grade.as_ref().unwrap().name, "Ana".to_owned());

    list.remove_worker();
    list.remove_worker();

    assert_eq!(list.grade.as_ref().unwrap().role, Role::CEO);
    assert_eq!(list.grade.as_ref().unwrap().name, "Marie".to_owned());
}

#[test]
fn test_last_worker() {
    let mut list = WorkEnvironment::new();
    list.add_worker("Marie", "CEO");
    list.add_worker("Monica", "Manager");
    list.add_worker("Ana", "Normal Worker");
    list.add_worker("Alice", "Normal Worker");

    assert_eq!(
        list.last_worker(),
        Some(("Alice".to_owned(), Role::Worker))
    );

    list.remove_worker();
    assert_eq!(
        list.last_worker(),
        Some(("Ana".to_owned(), Role::Worker))
    );

    list.remove_worker();
    assert_eq!(
        list.last_worker(),
        Some(("Monica".to_owned(), Role::Manager))
    );

    list.remove_worker();
    assert_eq!(
        list.last_worker(),
        Some(("Marie".to_owned(), Role::CEO))
    );
}
