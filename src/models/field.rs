use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub label: String,
    pub type_id: u8,
    pub input_type_id: u8,
    pub options: String,
}
impl Field {
    pub fn new(
        name: String, 
        label: String, 
        type_id: u8, 
        input_type_id: u8, 
        options: String
    ) -> Self {
        Field {
            name,
            label,
            type_id,
            input_type_id,
            options,
        }
    }
}