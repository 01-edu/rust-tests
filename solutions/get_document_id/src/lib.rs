#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorOffice {
    OfficeClose(u32),
    OfficeNotFound(u32),
    OfficeFull(u32),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeOne {
    pub next_office: Result<OfficeTwo, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeTwo {
    pub next_office: Result<OfficeThree, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeThree {
    pub next_office: Result<OfficeFour, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeFour {
    pub document_id: Result<u32, ErrorOffice>,
}

impl OfficeOne {
    #[inline]
    pub fn get_document_id(&self) -> Result<u32, ErrorOffice> {
        self.next_office?.next_office?.next_office?.document_id
    }
}
