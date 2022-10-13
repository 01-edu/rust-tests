use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

// I want to print a table well formatted:
// Each column must adjust automatically depending on the maximum
// length of the string inside
// Ex:
// |  Name   | Last Name |
// |---------+-----------|
// | augusto |  ornelas  |
// here augusto is the longest string in the name column and therefore
// the col is adjusted in response
// in the column last name "last name" is the longest string so the
// length of the column is adjusted and "ornelas" stays in the center
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.len() == 0 {
            return Ok(());
        }
        // get the maximum length of each column
        let cols_len = self.columns_len();

        // print just one row without the newline
        let print_row = |row: &Vec<String>, f: &mut fmt::Formatter| -> fmt::Result {
            write!(f, "|")?;
            for (i, col) in row.iter().enumerate() {
                write!(f, " {:^1$} |", col, cols_len[i])?;
            }
            Ok(())
        };

        print_row(&self.headers, f)?;
        write!(f, "\n")?;
        // write the separator between the headers and the rest of the list
        write!(f, "|")?;
        for v in &cols_len[..(cols_len.len() - 1)] {
            write!(f, "{:->1$}", "+", v + 3)?;
        }

        write!(f, "{:->1$}", "|", cols_len[cols_len.len() - 1] + 3)?;
        write!(f, "\n")?;

        // write the rest of the list
        for row in self.body.iter() {
            print_row(&row, f)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
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
    fn max_col(&self, col: usize) -> usize {
        let col_header_length = self.headers[col].len();
        let mut max: usize = 0;
        for row in &self.body {
            for (i, v) in row.iter().enumerate() {
                if i == col && v.len() > max {
                    max = v.len()
                }
            }
        }
        if max > col_header_length {
            max
        } else {
            col_header_length
        }
    }

    #[allow(dead_code)]
    fn columns_len(&self) -> Vec<usize> {
        let mut cols_len = Vec::with_capacity(self.headers.len());
        for i in 0..self.headers.len() {
            cols_len.push(self.max_col(i));
        }
        cols_len
    }
    #[allow(dead_code)]
    pub fn add_header(&mut self, new_header: &str) {
        self.headers.push(new_header.to_owned());
    }

    #[allow(dead_code)]
    pub fn add_row(&mut self, row: &[String]) {
        assert_eq!(self.headers.len(), row.len());
        self.body.push(row.to_vec());
    }
}
