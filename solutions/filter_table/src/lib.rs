#[derive(Clone, Debug, PartialEq, Default)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_row(&mut self, row: &[String]) {
        assert_eq!(self.headers.len(), row.len());
        self.body.push(row.to_vec());
    }

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

    fn get_col(&self, index: usize) -> Option<Vec<String>> {
        let header = self.headers.get(index)?;
        let mut col = vec![header.to_owned()];
        for row in &self.body {
            let value = row.get(index)?;
            col.push(value.to_owned());
        }
        Some(col)
    }

    pub fn filter_col(&self, filter: impl Fn(&str) -> bool) -> Option<Self> {
        let mut n_table = Table::new();
        for (i, col) in self.headers.iter().enumerate() {
            if filter(col) {
                let column = self.get_col(i)?;
                n_table.add_col(&column);
            }
        }
        Some(n_table)
    }

    pub fn filter_row(&self, col_name: &str, filter: impl Fn(&str) -> bool) -> Option<Self> {
        let mut table = Table::new();
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
