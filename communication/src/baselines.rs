use struct_field_names_as_array::FieldNamesAsArray;

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
pub struct Baseline {
    pub baseline_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub is_active: bool,
}

impl Baseline {
    pub fn fields() -> [&'static str; 5] {
        Baseline::FIELD_NAMES_AS_ARRAY
    }
}
