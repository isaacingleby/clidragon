pub struct Warhorse {
    pub name: String,
    pub breed: String,
    pub size: u8,
    pub dexterity: u8,
    pub strength: u8,
    pub constitution: u8,
    pub armour: u8,
    pub movement: u8,
    pub damage: u8,
    pub hit_points: u8,
}

pub struct BestRidingHorse {
    pub name: String,
    pub breed: String,
    pub worth: u8,
    pub movement: u8,
    pub constitution: u8,
    pub hit_points: u8,
}

pub struct GenericHorse {
    pub breed: String,
    pub worth: u8,
    pub movement: u8,
}

pub struct JoustingScore {
    pub wins: u8,
    pub losses: u8,
}

pub struct Horses {
    pub warhorse: Warhorse,
    pub best_riding_horse: BestRidingHorse,
    pub squires_horse: GenericHorse,
    pub other_horses: Vec<GenericHorse>,
    pub jousting_score: JoustingScore,
    pub equestrian_note: String,
}
