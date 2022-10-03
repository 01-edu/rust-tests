#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            body: Vec::new(),
            headers: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_row(&mut self, row: &[String]) {
        assert_eq!(self.headers.len(), row.len());
        self.body.push(row.to_vec());
    }

    #[allow(dead_code)]
    pub fn add_col(&mut self, col: &[String]) {
        let mut col_it = col.iter();

        self.headers.push(col_it.next().unwrap().to_owned());
        for (i, v) in col_it.enumerate() {
            if self.body.len() <= i {
                self.body.push(Vec::new());
            }
            self.body[i].push(v.to_owned());
        }
    }

    #[allow(dead_code)]
    fn get_col(&self, index: usize) -> Option<Vec<String>> {
        let header = self.headers.get(index)?;
        let mut col = vec![header.to_owned()];
        for row in &self.body {
            let value = row.get(index)?;
            col.push(value.to_owned());
        }
        Some(col)
    }

    #[allow(dead_code)]
    pub fn filter_col<T: Fn(&str) -> bool>(&self, filter: T) -> Option<Self> {
        let mut n_table = Table::new();
        for (i, col) in self.headers.iter().enumerate() {
            if filter(&col) {
                let column = self.get_col(i)?;
                n_table.add_col(&column);
            }
        }
        Some(n_table)
    }

    // the filter is only to choose which elements of the column must
    // be returned
    #[allow(dead_code)]
    pub fn filter_row<T: Fn(&str) -> bool>(&self, col_name: &str, filter: T) -> Option<Self> {
        let mut table = Table::new();
        // the first argument of the closure is the header and the
        // second is the value of the column
        // to select the column of a table with
        table.headers = self.headers.clone();
        let mut col = 0;
        for (i, header) in self.headers.iter().enumerate() {
            if header == col_name {
                col = i;
                break;
            }
        }

        for row in &self.body {
            let val = row.get(col)?;
            if filter(val) {
                table.add_row(&row.clone());
            }
        }
        Some(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtering_columns() {
        let mut table = Table::new();
        table.headers = vec![
            "name".to_string(),
            "lastname".to_string(),
            "id number".to_string(),
        ];
        table.add_row(&[
            "Ackerley".to_string(),
            "Philips".to_string(),
            "123456789".to_string(),
        ]);
        table.add_row(&[
            "Adamaris".to_string(),
            "Philips".to_string(),
            "1111123456789".to_string(),
        ]);
        table.add_row(&[
            "Ackerley".to_string(),
            "Philips".to_string(),
            "123456789".to_string(),
        ]);

        let filter = |col: &str| col == "name";

        let new_table = Table {
            headers: vec!["name".to_string()],
            body: vec![
                vec!["Ackerley".to_string()],
                vec!["Adamaris".to_string()],
                vec!["Ackerley".to_string()],
            ],
        };
        assert_eq!(new_table, table.filter_col(filter).unwrap());
    }

    #[test]
    fn filtering_rows() {
        let tab = Table {
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
                    "Thomas".to_string(),
                    "Shelby".to_string(),
                    "123456789".to_string(),
                ],
                vec![
                    "Ackerley".to_string(),
                    "Philips".to_string(),
                    "123456789".to_string(),
                ],
            ],
        };

        let get_fillips = |s: &str| s == "Philips";
        // filter the elements with last name Philips
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
            tab.filter_row("Last Name", get_fillips).unwrap(),
            expected_table
        );
    }
}
