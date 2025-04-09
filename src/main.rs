use clidragon::{
    armour::Armour, attacks::Weapon, character_traits::CharacterTraits,
    characteristics::Characteristics, family::Family, health::Health,
    history_and_events::HistoryAndEvents, horses::Horses, passions::Passions,
    passive_glory::PassiveGlory, personal_info::PersonalInfo, skills::Skills, squire::Squire,
};

struct PendragonCharacter {
    name: String,
    escutcheon: String,
    glory: u8,
    honour: u8,
    session_glory: u8,
    personal_info: PersonalInfo,
    characteristics: Characteristics,
    traits: CharacterTraits,
    health: Health,
    armour: Armour,
    attacks: Vec<Weapon>,
    passions: Passions,
    skills: Skills,
    horses: Horses,
    squire: Squire,
    history_and_events: Vec<HistoryAndEvents>,
    passive_glory: PassiveGlory,
    equipment: Vec<String>,
    family: Family,
    holdings: Vec<String>,
}

fn main() {
    println!("Hello, world!");
}
