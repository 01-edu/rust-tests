#[macro_export]
macro_rules! hash_map {
    () => {
        ::std::collections::HashMap::new()
    };

    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )+
        map
    }};
}
