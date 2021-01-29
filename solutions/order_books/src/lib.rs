pub mod library {
    pub mod writers {
        use super::books::*;
        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }
    pub mod books {
        pub struct Book {
            pub title: String,
            pub year: usize,
        }
    }
}
pub use library::writers::Writer;

pub fn order_books(writer: &mut Writer) {
    writer
        .books
        .sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
}
