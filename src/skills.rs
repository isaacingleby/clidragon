pub struct NonCombat {
    pub awareness: u8,
    pub chirurgery: u8,
    pub compose: u8,
    pub courtesy: u8,
    pub dancing: u8,
    pub falconry: u8,
    pub fashion: u8,
    pub first_aid: u8,
    pub flirting: u8,
    pub folk_lore: u8,
    pub gaming: u8,
    pub hunting: u8,
    pub industry: u8,
    pub intrigue: u8,
    pub literacy: u8,
    pub orate: u8,
    pub play_instrument: u8,
    pub recognize: u8,
    pub religion: u8,
    pub singing: u8,
    pub stewardship: u8,
}

pub struct Combat {
    pub battle: u8,
    pub brawling: u8,
    pub crossbow: u8,
    pub bow: u8,
    pub thrown_weapon: u8,
    pub horsemanship: u8,
    pub sword: u8,
    pub charge: u8,
    pub spear: u8,
    pub hafted: u8,
    pub two_handed_hafted: u8,
}

pub struct Skills {
    pub non_combat: NonCombat,
    pub combat: Combat,
}
