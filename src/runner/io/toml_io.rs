use crate::runner::{
    compiler::ptr_utils::alloc_for_type,
    file_utils::{FileLoadError, get_file_contents},
    ui::input_ui::print_entered_input,
};
use kasl::{
    scope_manager::IOBlueprint,
    type_registry::{PrimitiveType, ResolvedType},
};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    path::Path,
};

pub enum TomlLoadError {
    FileLoadError(FileLoadError),
    ParseError(String),
    MissingField(String),
    TypeMismatch { name: String, expected: String },
    VoidInput,
    NonPrimitiveInput,
}

impl Display for TomlLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlLoadError::FileLoadError(err) => write!(f, "{}", err),
            TomlLoadError::ParseError(err) => write!(f, "Failed to parse TOML: {}", err),
            TomlLoadError::MissingField(name) => write!(f, "Missing field: {}", name),
            TomlLoadError::TypeMismatch { name, expected } => {
                write!(f, "Type mismatch for {}: expected {}", name, expected)
            }
            TomlLoadError::VoidInput => write!(f, "Void input"),
            TomlLoadError::NonPrimitiveInput => write!(f, "Non-primitive input"),
        }
    }
}

pub fn load_inputs_from_toml(
    blueprint: &IOBlueprint,
    path: &Path,
) -> Result<Vec<*mut ()>, TomlLoadError> {
    let content = get_file_contents(path).map_err(TomlLoadError::FileLoadError)?;
    let table: HashMap<String, toml::Value> =
        toml::from_str(&content).map_err(|err| TomlLoadError::ParseError(err.to_string()))?;
    let mut ptrs = Vec::new();

    for input in blueprint.get_inputs() {
        let raw_val = table
            .get(&input.name)
            .ok_or_else(|| TomlLoadError::MissingField(input.name.clone()))?;

        let str_value = match &input.value_type {
            ResolvedType::Primitive(PrimitiveType::Bool) => {
                let value = raw_val
                    .as_bool()
                    .ok_or_else(|| TomlLoadError::TypeMismatch {
                        name: input.name.clone(),
                        expected: "Bool".to_string(),
                    })?;
                ptrs.push(alloc_for_type(value));
                value.to_string()
            }
            ResolvedType::Primitive(PrimitiveType::Float) => {
                let value = raw_val
                    .as_float()
                    .ok_or_else(|| TomlLoadError::TypeMismatch {
                        name: input.name.clone(),
                        expected: "Float".to_string(),
                    })? as f32;
                ptrs.push(alloc_for_type(value));
                value.to_string()
            }
            ResolvedType::Primitive(PrimitiveType::Int) => {
                let value = raw_val
                    .as_integer()
                    .ok_or_else(|| TomlLoadError::TypeMismatch {
                        name: input.name.clone(),
                        expected: "Int".to_string(),
                    })? as i32;
                ptrs.push(alloc_for_type(value));
                value.to_string()
            }
            ResolvedType::Primitive(PrimitiveType::Void) => {
                return Err(TomlLoadError::VoidInput);
            }
            ResolvedType::Struct(_) => {
                return Err(TomlLoadError::NonPrimitiveInput);
            }
        };

        print_entered_input(input, &str_value);
    }

    Ok(ptrs)
}
