use surrealdb::sql::Thing;

pub enum ProjectOwner {
    User(Thing),
    Group(Thing),
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
