pub struct Parent {
    pub name: String,
    pub glory: u8,
}

pub struct Family {
    pub father: Parent,
    pub mother: Parent,
    pub family_characteristic: String,
    pub notes: String,
}
