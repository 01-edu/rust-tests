use std::fmt;

pub struct Park {
    pub name: String,
    pub park_type: ParkType,
    pub address: String,
    pub cap: String,
    pub state: String,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - ", self.park_type)?;

        if self.name.is_empty() {
            write!(f, "No name, ")?;
        } else {
            write!(f, "{}, ", self.name)?;
        }

        if self.address.is_empty() {
            write!(f, "No address, ")?;
        } else {
            write!(f, "{}, ", self.address)?;
        }

        if self.cap.is_empty() {
            write!(f, "No cap - ")?;
        } else {
            write!(f, "{} - ", self.cap)?;
        }

        if self.state.is_empty() {
            write!(f, "No state")?;
        } else {
            write!(f, "{}", self.state)?;
        }
        Ok(())
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParkType::Garden => write!(f, "garden")?,
            ParkType::Forest => write!(f, "forest")?,
            ParkType::Playground => write!(f, "playground")?,
        }
        Ok(())
    }
}
