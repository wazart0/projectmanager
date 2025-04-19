use struct_field_names_as_array::FieldNamesAsArray;

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub enum Frequency {
    Yearly,
    Monthly,
    Weekly,
    Daily,
    Hourly,
    Minutely,
    Secondly,
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequency::Yearly => write!(f, "Yearly"),
            Frequency::Monthly => write!(f, "Monthly"),
            Frequency::Weekly => write!(f, "Weekly"),
            Frequency::Daily => write!(f, "Daily"),
            Frequency::Hourly => write!(f, "Hourly"),
            Frequency::Minutely => write!(f, "Minutely"),
            Frequency::Secondly => write!(f, "Secondly"),
        }
    }
}

#[derive(
    bitcode::Encode,
    bitcode::Decode,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    Debug,
    FieldNamesAsArray,
)]
pub struct Resource {
    pub resource_id: i64,
    pub name: String,
    pub resource_type_id: i64,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub cost: Option<f64>,
    pub cost_currency: String,
    pub billing_frequency: Option<Frequency>,
    pub billing_interval: Option<i32>,
    pub availability: Option<String>, // TODO: placeholder, modify to reference to calendar
    pub capacity: Option<f64>,
    pub capacity_unit: Option<String>,
    pub is_active: bool,
}

impl Resource {
    pub fn fields() -> [&'static str; 13] {
        Resource::FIELD_NAMES_AS_ARRAY
    }
}

#[derive(
    bitcode::Encode,
    bitcode::Decode,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    Debug,
    FieldNamesAsArray,
)]
pub struct ResourceType {
    pub resource_type_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
}

impl ResourceType {
    pub fn fields() -> [&'static str; 4] {
        ResourceType::FIELD_NAMES_AS_ARRAY
    }
}
