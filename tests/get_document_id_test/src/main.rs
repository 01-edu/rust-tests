use get_document_id::*;

fn main() {
    let office_ok = OfficeOne {
        next_office: Ok(OfficeTwo {
            next_office: Ok(OfficeThree {
                next_office: Ok(OfficeFour {
                    document_id: Ok(13),
                    id: 4,
                }),
                id: 3,
            }),
            id: 2,
        }),
        id: 1,
    };
    let office_closed = {
        OfficeOne {
            next_office: Ok(OfficeTwo {
                next_office: Err(ErrorOffice::OfficeClose(2)),
                id: 2,
            }),
            id: 1,
        }
    };

    match office_ok.get_document_id() {
        Ok(id) => println!("Found a document with id {}", id),
        Err(err) => match err {
            ErrorOffice::OfficeClose(id) => println!("Error: office {id} closed!"),
            ErrorOffice::OfficeNotFound(id) => println!("Error: office {id} not found!"),
            ErrorOffice::OfficeFull(id) => println!("Error: office {id} full!"),
        },
    };
    match office_closed.get_document_id() {
        Ok(id) => println!("Found a document with id {}", id),
        Err(err) => match err {
            ErrorOffice::OfficeClose(id) => println!("Error: office {id} closed!"),
            ErrorOffice::OfficeNotFound(id) => println!("Error: office {id} not found!"),
            ErrorOffice::OfficeFull(id) => println!("Error: office {id} full!"),
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_document_id_ok() {
        let office = OfficeOne {
            next_office: Ok(OfficeTwo {
                next_office: Ok(OfficeThree {
                    next_office: Ok(OfficeFour {
                        document_id: Ok(13),
                        id: 4,
                    }),
                    id: 3,
                }),
                id: 2,
            }),
            id: 1,
        };

        assert_eq!(Ok(13), office.get_document_id());
    }
    #[test]
    fn test_get_document_id_closed() {
        let office = {
            OfficeOne {
                next_office: Ok(OfficeTwo {
                    next_office: Err(ErrorOffice::OfficeClose(2)),
                    id: 2,
                }),
                id: 1,
            }
        };

        assert_eq!(Err(ErrorOffice::OfficeClose(2)), office.get_document_id());
    }
    #[test]
    fn test_get_document_id_not_found() {
        let office = {
            OfficeOne {
                next_office: Ok(OfficeTwo {
                    next_office: Err(ErrorOffice::OfficeNotFound(2)),
                    id: 2,
                }),
                id: 1,
            }
        };

        assert_eq!(
            Err(ErrorOffice::OfficeNotFound(2)),
            office.get_document_id()
        );
    }
    #[test]
    fn test_get_document_id_full() {
        let office = {
            OfficeOne {
                next_office: Ok(OfficeTwo {
                    next_office: Err(ErrorOffice::OfficeFull(2)),
                    id: 2,
                }),
                id: 1,
            }
        };

        assert_eq!(Err(ErrorOffice::OfficeFull(2)), office.get_document_id());
    }
}
