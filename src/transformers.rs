use crate::models::field::Field;
use serde::Serialize;

#[derive(Serialize)]
pub struct FieldJson {
    usage: u8,
    name: String,
    label: String,
    #[serde(rename = "type")]
    type_id: String,
    inputtype: String,
    new_values: String,
    searchable: bool,
    user_searchable: bool,
    calendar_searchable: bool,
    copytochild: bool,
    copytochildonupdate: bool,
}

pub struct JsonTransformer;

impl JsonTransformer {
    pub fn transform_fields(fields: &[Field]) -> Vec<FieldJson> {
        fields
            .iter()
            .map(|field| FieldJson {
                usage: 1, // Default value
                name: field.name.clone(),
                label: field.label.clone(),
                type_id: field.type_id.to_string(),
                inputtype: field.input_type_id.to_string(),
                new_values: field.options.clone(),
                searchable: true,          // Default value
                user_searchable: true,     // Default value
                calendar_searchable: true, // Default value
                copytochild: true,         // Default value
                copytochildonupdate: true, // Default value
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn to_json(fields: &[Field]) -> Result<String, serde_json::Error> {
        let json_fields = Self::transform_fields(fields);
        serde_json::to_string_pretty(&json_fields)
    }
}
