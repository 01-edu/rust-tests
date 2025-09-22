#[derive(Debug, Eq, PartialEq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Jacket {
    White,
    Black,
    Flowers,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

#[inline]
pub fn choose_outfit(
    formality_level: Option<u32>,
    invitation_message: Result<&str, &str>,
) -> Outfit {
    let jacket = match formality_level {
        Some(level) if level > 0 => Jacket::White,
        Some(_) => Jacket::Black,
        _ => Jacket::Flowers,
    };

    Outfit {
        hat: match invitation_message {
            Ok(_) => Hat::Fedora,
            Err(_) if jacket == Jacket::Flowers => Hat::Baseball,
            _ => Hat::Snapback,
        },
        jacket,
    }
}
