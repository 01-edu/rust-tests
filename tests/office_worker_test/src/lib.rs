use office_worker::*;

#[test]
fn test_office_worker() {
    assert_eq!(
        OfficeWorker::from("Louise,25,admin"),
        OfficeWorker {
            name: "Louise".to_owned(),
            age: 25,
            role: WorkerRole::Admin,
        }
    );
    assert_eq!(
        OfficeWorker::from("Rob,11,guest"),
        OfficeWorker {
            name: "Rob".to_owned(),
            age: 11,
            role: WorkerRole::Guest,
        }
    );
    assert_eq!(
        OfficeWorker::from("Maria Agata,44,user"),
        OfficeWorker {
            name: "Maria Agata".to_owned(),
            age: 44,
            role: WorkerRole::User,
        }
    );
}

#[test]
fn test_worker_role() {
    assert_eq!(WorkerRole::from("guest"), WorkerRole::Guest);
    assert_eq!(WorkerRole::from("admin"), WorkerRole::Admin);
    assert_eq!(WorkerRole::from("user"), WorkerRole::User);
}
