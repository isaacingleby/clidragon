pub struct Passion {
    pub intensity: u8,
    pub target: String,
}

pub struct Fidelitas {
    pub homage: Passion,
    pub fealty: Passion,
    pub duty: Passion,
    pub loyalty: Passion,
}

pub struct Fervour {
    pub love: Passion,
    pub hate: Passion,
}

pub struct Adoratio {
    pub devotion: Passion,
    pub adoration: Passion,
}

pub struct Civilitas {
    pub hospitality: Passion,
    pub station: Passion,
    pub chivalry: Passion,
}

pub struct Passions {
    pub fidelitas: Fidelitas,
    pub fervour: Fervour,
    pub adoratio: Adoratio,
    pub civilitas: Civilitas,
}
