#[macro_export]
macro_rules! hash_map {
    () => {{
        crate::HashMap::new()
    }};

    ($($k:expr => $v:expr),+ $(,)?) => {{
        use crate::HashMap;
        let mut hm = HashMap::new();

        $(
            hm.insert($k, $v);
        )+

        hm
    }};
}
