
    use csv::Reader;
    use crate::models::field::Field;
    use crate::config::Config;
    use crate::error::{Result, CustomError, FieldError, FieldErrorKind};
    
    pub struct CsvReader;
    #[derive(Debug)]
    struct FieldPositions {
        name: usize,
        label: usize,
        type_id: usize,
        input_type_id: usize,
        options: usize,
    }
    
    impl CsvReader {
        pub fn new() -> Self {
            CsvReader
        }
    
        fn open_csv(&self, config: &Config) -> Result<Reader<std::fs::File>> {
            Ok(Reader::from_path(&config.source_file_name)?)
        }
    
        fn get_field_positions(&self, headers: &csv::StringRecord) -> Result<FieldPositions> {
            Ok(FieldPositions {
                name: headers.iter()
                    .position(|h| h == "name")
                    .ok_or_else(|| CustomError::MissingColumn("name".to_string()))?,
                    
                label: headers.iter()
                    .position(|h| h == "label")
                    .ok_or_else(|| CustomError::MissingColumn("label".to_string()))?,
                    
                type_id: headers.iter()
                    .position(|h| h == "type_id")
                    .ok_or_else(|| CustomError::MissingColumn("type_id".to_string()))?,
                    
                input_type_id: headers.iter()
                    .position(|h| h == "input_type_id")
                    .ok_or_else(|| CustomError::MissingColumn("input_type_id".to_string()))?,
                    
                options: headers.iter()
                    .position(|h| h == "options")
                    .ok_or_else(|| CustomError::MissingColumn("options".to_string()))?,
            })
        }
        
        #[allow(dead_code)]
        fn validate_required_field(&self, value: &str, field_name: &str, row: usize) -> Result<String> {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Err(self.field_error(row, 
                    FieldErrorKind::RequiredFieldEmpty(field_name.to_string())
                ));
            }
            Ok(trimmed.to_string())
        }
    
        fn field_error(&self, row: usize, error: FieldErrorKind) -> CustomError {
            CustomError::FieldError(FieldError { row, error })
        }
    
        fn validate_field_name(&self, name: &str, row: usize) -> Result<String> {
            let trimmed = name.trim();
            
            if trimmed.is_empty() {
                return Err(self.field_error(row, 
                    FieldErrorKind::RequiredFieldEmpty("name".to_string())
                ));
            }
            
            if !trimmed.chars().all(|c| c.is_alphanumeric()) {
                return Err(self.field_error(row,
                    FieldErrorKind::InvalidFieldName(trimmed.to_string())
                ));
            }
            
            Ok(trimmed.to_string())
        }
    
        fn validate_label(&self, label: &str, row: usize) -> Result<String> {
            let trimmed = label.trim();
            
            if trimmed.is_empty() {
                return Err(self.field_error(row, 
                    FieldErrorKind::RequiredFieldEmpty("label".to_string())
                ));
            }
            
            if label.trim() == " " {
                return Err(self.field_error(row,
                    FieldErrorKind::InvalidLabel("Label cannot be a single space".to_string())
                ));
            }
            
            Ok(trimmed.to_string())
        }
    
        fn validate_type_id(&self, type_id: &str, row: usize) -> Result<u8> {
            let type_id: u8 = type_id.parse()
                .map_err(|_| self.field_error(row, FieldErrorKind::ParseError("type_id".to_string())))?;
                
            match type_id {
                0 | 1 | 2 | 3 | 4 | 5 | 6 | 10 => Ok(type_id),
                _ => Err(self.field_error(row, FieldErrorKind::InvalidTypeId(type_id.to_string())))
            }
        }
    
        fn validate_input_type(&self, input_type: &str, type_id: u8, row: usize) -> Result<u8> {
            let input_type: u8 = input_type.parse()
                .map_err(|_| self.field_error(row, FieldErrorKind::ParseError("input_type_id".to_string())))?;
            
            match type_id {
                0 => match input_type {
                    0..=6 => Ok(input_type),
                    _ => Err(self.field_error(row, FieldErrorKind::InvalidInputType(
                        "Text fields (type_id: 0) accept these input types:\n\
                        0: Anything\n\
                        1: Integer\n\
                        2: Money\n\
                        3: Alphanumeric\n\
                        4: Decimal\n\
                        5: URL\n\
                        6: Password".to_string()
                    )))
                },
                
                2 => match input_type {
                    0..=2 => Ok(input_type),
                    _ => Err(self.field_error(row, FieldErrorKind::InvalidInputType(
                        "Single Selection fields (type_id: 2) accept these input types:\n\
                        0: Standard dropdown\n\
                        1: Tree dropdown\n\
                        2: Radio selection".to_string()
                    )))
                },
                
                4 => match input_type {
                    0..=1 => Ok(input_type),
                    _ => Err(self.field_error(row, FieldErrorKind::InvalidInputType(
                        "Date fields (type_id: 4) accept these input types:\n\
                        0: Date only\n\
                        1: Date and time".to_string()
                    )))
                },
                
                1 | 3 | 5 | 6 | 10 => {
                    if input_type == 0 {
                        Ok(input_type)
                    } else {
                        Err(self.field_error(row, FieldErrorKind::InvalidInputType(format!(
                            "Field type {} only accepts input_type_id 0. This field type has no input options.",
                            match type_id {
                                1 => "Memo",
                                3 => "Multiple Selection",
                                5 => "Time",
                                6 => "Checkbox",
                                10 => "Rich",
                                _ => unreachable!()
                            }
                        ))))
                    }
                },
                
                _ => Err(self.field_error(row, FieldErrorKind::InvalidInputType("Invalid type_id".to_string())))
            }
        }
    
        fn validate_options(&self, options: &str, type_id: u8, row: usize) -> Result<String> {
            match type_id {
                // Single/Multiple Selection fields (2, 3) require options
                2 | 3 => {
                    if options.trim().is_empty() {
                        Err(self.field_error(row, FieldErrorKind::MissingOptions(
                            "Selection fields require at least one option".to_string()
                        )))
                    } else {
                        Ok(options.to_string())
                    }
                },
                // Other field types don't need options validation
                _ => Ok(options.to_string())
            }
        }
    
        pub fn read_fields(&self, config: &Config) -> Result<Vec<Field>> {
            let mut fields = Vec::new();
            let mut reader = self.open_csv(config)?;
            
            let headers = reader.headers()?;
            let positions = self.get_field_positions(headers)?;
    
            for (row_idx, result) in reader.records().enumerate() {
                let record = result?;
                
                // Validate type_id first as other validations depend on it
                let type_id = self.validate_type_id(&record[positions.type_id], row_idx)?;
                
                let field = Field::new(
                    self.validate_field_name(&record[positions.name], row_idx)?,
                    self.validate_label(&record[positions.label], row_idx)?,
                    type_id,
                    self.validate_input_type(&record[positions.input_type_id], type_id, row_idx)?,
                    self.validate_options(&record[positions.options], type_id, row_idx)?,
                );
                
                fields.push(field);
            }
    
            Ok(fields)
        }
    }
