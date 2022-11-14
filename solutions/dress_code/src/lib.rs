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

pub fn choose_outfit(
    formality_level: Option<u32>,
    invitation_message: Result<&str, &str>,
) -> Outfit {
    let mut outfit = Outfit {
        jacket: Jacket::Black,
        hat: Hat::Snapback,
    };

    match formality_level {
        Some(level) => {
            if level > 0 {
                outfit.jacket = Jacket::White;
            }
        }
        _ => {
            outfit.jacket = Jacket::Flowers;
        }
    }
    match invitation_message {
        Ok(_) => {
            outfit.hat = Hat::Fedora;
        }
        Err(_) => {
            if let Jacket::Flowers = outfit.jacket {
                outfit.hat = Hat::Baseball;
            }
        }
    }
    outfit
}
