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
            pub year: u32,
        }
    }
}

pub use library::writers::Writer;

#[inline]
pub fn order_books(writer: &mut Writer) {
    writer
        .books
        .sort_unstable_by_key(|b| b.title.to_lowercase());
}
