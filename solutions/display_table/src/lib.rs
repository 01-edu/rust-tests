use std::fmt;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.is_empty() {
            return Ok(());
        }

        let cols_len = self.columns_len();

        let print_row = |row: &Vec<String>, f: &mut fmt::Formatter| -> fmt::Result {
            write!(f, "|")?;
            for (i, col) in row.iter().enumerate() {
                write!(f, " {:^1$} |", col, cols_len[i])?;
            }
            writeln!(f)?;
            Ok(())
        };

        print_row(&self.headers, f)?;
        write!(f, "|")?;
        for v in &cols_len[..(cols_len.len() - 1)] {
            write!(f, "{:->1$}", "+", v + 3)?;
        }

        write!(f, "{:->1$}", "|", cols_len[cols_len.len() - 1] + 3)?;
        writeln!(f)?;

        for row in &self.body {
            print_row(row, f)?;
        }

        Ok(())
    }
}

impl Table {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    fn max_col(&self, col: usize) -> usize {
        let header_len = self.headers[col].len();

        self.body
            .iter()
            .map(|r| r[col].len())
            .max()
            .unwrap_or_default()
            .max(header_len)
    }

    #[inline]
    fn columns_len(&self) -> Vec<usize> {
        (0..self.headers.len()).map(|i| self.max_col(i)).collect()
    }

    #[inline]
    pub fn add_row(&mut self, row: &[String]) {
        assert_eq!(self.headers.len(), row.len());
        self.body.push(row.to_vec());
    }
}
