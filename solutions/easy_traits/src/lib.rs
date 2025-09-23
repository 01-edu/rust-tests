pub trait AppendStrExt {
    fn append_str(&mut self, str_to_append: &str) -> &mut Self;

    fn append_number(&mut self, nb_to_append: f64) -> &mut Self;

    fn remove_punctuation_marks(&mut self) -> &mut Self;
}

impl AppendStrExt for String {
    #[inline]
    fn append_str(&mut self, str_to_append: &str) -> &mut Self {
        self.push_str(str_to_append);

        self
    }

    #[inline]
    fn append_number(&mut self, nb_to_append: f64) -> &mut Self {
        self.push_str(nb_to_append.to_string().as_str());

        self
    }

    #[inline]
    fn remove_punctuation_marks(&mut self) -> &mut Self {
        self.retain(|c| !matches!(c, '.' | ',' | '?' | '!'));

        self
    }
}
