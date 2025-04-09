pub struct Wounds {
    pub w1: bool,
    pub w2: bool,
    pub w3: bool,
    pub w4: bool,
    pub w5: bool,
    pub w6: bool,
    pub w7: bool,
}

pub struct Health {
    pub current_hp: u8,
    pub debilitated: bool,
    pub wounds: Wounds,
    pub aggravation_damage: u8,
    pub deterioration_damage: u8,
}
