#[cfg(test)]
mod tests {
    use order_books::{
        library::{books::Book, writers::Writer},
        order_books,
    };

    #[test]
    fn w_shakespeare() {
        let mut writer = Writer {
            first_name: "William".to_owned(),
            last_name: "Shakespeare".to_owned(),
            books: vec![
                Book {
                    title: "Hamlet".to_owned(),
                    year: 1600,
                },
                Book {
                    title: "Othelo".to_owned(),
                    year: 1603,
                },
                Book {
                    title: "Romeo and Juliet".to_owned(),
                    year: 1593,
                },
                Book {
                    title: "MacBeth".to_owned(),
                    year: 1605,
                },
            ],
        };

        order_books(&mut writer);

        assert_eq!("Hamlet", writer.books[0].title);
        assert_eq!("MacBeth", writer.books[1].title);
        assert_eq!("Othelo", writer.books[2].title);
        assert_eq!("Romeo and Juliet", writer.books[3].title);
    }

    #[test]
    fn j_k_rowling() {
        let mut writer = Writer {
            first_name: "Joanne".to_owned(),
            last_name: "Rowling".to_owned(),
            books: vec![
                Book {
                    title: "Harry Potter and the Philosopher's Stone".to_owned(),
                    year: 1997,
                },
                Book {
                    title: "Harry Potter and the Prisoner of Azkaban".to_owned(),
                    year: 1999,
                },
                Book {
                    title: "Harry Potter and the Order of the Phoenix".to_owned(),
                    year: 2003,
                },
                Book {
                    title: "Harry Potter and the Chamber of Secrets".to_owned(),
                    year: 1998,
                },
                Book {
                    title: "Harry Potter and the Deathly Hallows".to_owned(),
                    year: 2007,
                },
            ],
        };

        order_books(&mut writer);

        assert_eq!(
            "Harry Potter and the Chamber of Secrets",
            writer.books[0].title
        );
        assert_eq!(
            "Harry Potter and the Deathly Hallows",
            writer.books[1].title
        );
        assert_eq!(
            "Harry Potter and the Order of the Phoenix",
            writer.books[2].title
        );
        assert_eq!(
            "Harry Potter and the Philosopher's Stone",
            writer.books[3].title
        );
        assert_eq!(
            "Harry Potter and the Prisoner of Azkaban",
            writer.books[4].title
        );
    }
}
