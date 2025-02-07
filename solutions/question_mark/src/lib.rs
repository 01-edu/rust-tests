#[derive(Clone, Copy)]
pub struct One {
    pub first_layer: Option<Two>,
}

#[derive(Clone, Copy)]
pub struct Two {
    pub second_layer: Option<Three>,
}

#[derive(Clone, Copy)]
pub struct Three {
    pub third_layer: Option<Four>,
}

#[derive(Clone, Copy)]
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        self.first_layer?.second_layer?.third_layer?.fourth_layer
    }
}
