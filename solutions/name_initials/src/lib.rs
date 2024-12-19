pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|n| n.split_once(' ').unwrap())
        .map(|(f, l)| {
            format!(
                "{}. {}.",
                f.chars().next().unwrap(),
                l.chars().next().unwrap()
            )
        })
        .collect()
}
