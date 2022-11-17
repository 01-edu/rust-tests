use get_document_id::*;

fn main() {
    let office_ok = OfficeOne {
        next_office: Ok(OfficeTwo {
            next_office: Ok(OfficeThree {
                next_office: Ok(OfficeFour {
                    document_id: Ok(13),
                }),
            }),
        }),
    };
    let office_closed = {
        OfficeOne {
            next_office: Ok(OfficeTwo {
                next_office: Err(ErrorOffice::OfficeClose(13)),
            }),
        }
    };

    match office_ok.get_document_id() {
        Ok(id) => println!("Found a document with id {}", id),
        Err(err) => match err {
            ErrorOffice::OfficeClose(_) => println!("Error: office closed!"),
            ErrorOffice::OfficeNotFound(_) => println!("Error: office not found!"),
            ErrorOffice::OfficeFull(_) => println!("Error: office full!"),
        },
    };
    match office_closed.get_document_id() {
        Ok(id) => println!("Found a document with id {}", id),
        Err(err) => match err {
            ErrorOffice::OfficeClose(_) => println!("Error: office closed!"),
            ErrorOffice::OfficeNotFound(_) => println!("Error: office not found!"),
            ErrorOffice::OfficeFull(_) => println!("Error: office full!"),
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
                    }),
                }),
            }),
        };
        assert_eq!(Ok(13), office.get_document_id());
    }
    #[test]
    fn test_get_document_id_closed() {
        let office = {
            OfficeOne {
                next_office: Ok(OfficeTwo {
                    next_office: Err(ErrorOffice::OfficeClose(13)),
                }),
            }
        };
        assert_eq!(Err(ErrorOffice::OfficeClose(13)), office.get_document_id());
    }
    #[test]
    fn test_get_document_id_not_found() {
        let office = {
            OfficeOne {
                next_office: Ok(OfficeTwo {
                    next_office: Err(ErrorOffice::OfficeNotFound(13)),
                }),
            }
        };
        assert_eq!(
            Err(ErrorOffice::OfficeNotFound(13)),
            office.get_document_id()
        );
    }
    #[test]
    fn test_get_document_id_full() {
        let office = {
            OfficeOne {
                next_office: Ok(OfficeTwo {
                    next_office: Err(ErrorOffice::OfficeFull(13)),
                }),
            }
        };
        assert_eq!(Err(ErrorOffice::OfficeFull(13)), office.get_document_id());
    }
}
