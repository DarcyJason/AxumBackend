pub enum ProjectOwner {
    User,
    Group,
}

pub struct ProjectFolder {
    pub name: String,
    pub path: String,
}

pub enum FieldType {
    Text,
    Hide,
    CheckBox,
    Url,
}

pub struct AdditionalField {
    pub field_type: FieldType,
    pub field_content: String,
}
