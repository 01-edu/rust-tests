use std::iter;

pub fn scytale_cipher(message: &str, i: usize) -> String {
    let i = i.min(message.len());
    if message.is_empty() || i == 0 {
        String::new()
    } else {
        let cols = message.len().div_ceil(i);

        (0..i)
            .map(|n| {
                message
                    .chars()
                    .skip(n)
                    .step_by(i)
                    .chain(iter::repeat_n(' ', cols - (message.len() - n).div_ceil(i)))
                    .collect::<String>()
            })
            .collect::<String>()
            .trim_end()
            .to_owned()
    }
}
