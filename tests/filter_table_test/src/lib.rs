use filter_table::*;

#[test]
fn filtering_columns() {
    let mut table = Table::new();
    table.headers = vec![
        "name".to_owned(),
        "lastname".to_owned(),
        "id number".to_owned(),
    ];
    table.add_row(&[
        "Ackerley".to_owned(),
        "Philips".to_owned(),
        "123456789".to_owned(),
    ]);
    table.add_row(&[
        "Adamaris".to_owned(),
        "Philips".to_owned(),
        "1111123456789".to_owned(),
    ]);
    table.add_row(&[
        "Ackerley".to_owned(),
        "Philips".to_owned(),
        "123456789".to_owned(),
    ]);

    let new_table = Table {
        headers: vec!["name".to_owned()],
        body: vec![
            vec!["Ackerley".to_owned()],
            vec!["Adamaris".to_owned()],
            vec!["Ackerley".to_owned()],
        ],
    };
    assert_eq!(new_table, table.filter_col(|c| c == "name").unwrap());
}

#[test]
fn filtering_rows() {
    let tab = Table {
        headers: vec![
            "Name".to_owned(),
            "Last Name".to_owned(),
            "ID Number".to_owned(),
        ],
        body: vec![
            vec![
                "Adamaris".to_owned(),
                "Philips".to_owned(),
                "1111123456789".to_owned(),
            ],
            vec![
                "Thomas".to_owned(),
                "Shelby".to_owned(),
                "123456789".to_owned(),
            ],
            vec![
                "Ackerley".to_owned(),
                "Philips".to_owned(),
                "123456789".to_owned(),
            ],
        ],
    };

    let expected_table = Table {
        headers: vec![
            "Name".to_string(),
            "Last Name".to_string(),
            "ID Number".to_string(),
        ],
        body: vec![
            vec![
                "Adamaris".to_string(),
                "Philips".to_string(),
                "1111123456789".to_string(),
            ],
            vec![
                "Ackerley".to_string(),
                "Philips".to_string(),
                "123456789".to_string(),
            ],
        ],
    };
    assert_eq!(
        tab.filter_row("Last Name", |s| s == "Philips").unwrap(),
        expected_table
    );
}
