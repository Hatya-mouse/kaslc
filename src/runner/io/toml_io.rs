use crate::runner::{
    compiler::ptr_utils::alloc_buf_for_type,
    file_utils::{FileLoadError, get_file_contents},
    ui::input_ui::{print_entered_input, print_input_header},
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
    TypeMismatch {
        name: String,
        expected: String,
    },
    NotAnArray(String),
    ArrayLengthMismatch {
        name: String,
        expected: usize,
        actual: usize,
    },
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
            TomlLoadError::NotAnArray(name) => write!(f, "Array is expected, but got {}", name),
            TomlLoadError::ArrayLengthMismatch {
                name,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Array length does not match for {}: expected {}, actual {}",
                    name, expected, actual
                )
            }
            TomlLoadError::VoidInput => write!(f, "Void input"),
            TomlLoadError::NonPrimitiveInput => write!(f, "Non-primitive input"),
        }
    }
}

pub fn load_inputs_from_toml(
    blueprint: &IOBlueprint,
    iterations: i32,
    path: &Path,
) -> Result<Vec<*mut ()>, TomlLoadError> {
    let content = get_file_contents(path).map_err(TomlLoadError::FileLoadError)?;
    let table: HashMap<String, toml::Value> =
        toml::from_str(&content).map_err(|err| TomlLoadError::ParseError(err.to_string()))?;
    let inputs = blueprint.get_inputs();

    // Initialize the pointer vector with capacity
    let mut ptrs = Vec::with_capacity(inputs.len());

    print_input_header();

    for input in inputs {
        let raw_val = table
            .get(&input.name)
            .ok_or_else(|| TomlLoadError::MissingField(input.name.clone()))?;

        match &input.value_type {
            ResolvedType::Primitive(PrimitiveType::Bool) => {
                let ptr = alloc_buf_for_type::<i8>(iterations as usize);
                parse_and_write_array::<i8, _>(
                    raw_val,
                    ptr,
                    iterations,
                    |v| v.as_bool().map(|b| if b { 1 } else { 0 }),
                    &input.name,
                    "Bool",
                )?;
                ptrs.push(ptr as *mut ());
            }
            ResolvedType::Primitive(PrimitiveType::Float) => {
                let ptr = alloc_buf_for_type::<f32>(iterations as usize);
                parse_and_write_array::<f32, _>(
                    raw_val,
                    ptr,
                    iterations,
                    |v| v.as_float().map(|f| f as f32),
                    &input.name,
                    "Float",
                )?;
                ptrs.push(ptr as *mut ());
            }
            ResolvedType::Primitive(PrimitiveType::Int) => {
                let ptr = alloc_buf_for_type::<i32>(iterations as usize);
                parse_and_write_array::<i32, _>(
                    raw_val,
                    ptr,
                    iterations,
                    |v| v.as_integer().map(|i| i as i32),
                    &input.name,
                    "Int",
                )?;
                ptrs.push(ptr as *mut ());
            }
            ResolvedType::Primitive(PrimitiveType::Void) => {
                return Err(TomlLoadError::VoidInput);
            }
            ResolvedType::Struct(_) => {
                return Err(TomlLoadError::NonPrimitiveInput);
            }
        };

        print_entered_input(input, &raw_val.to_string());
    }

    Ok(ptrs)
}

fn parse_and_write_array<T, F>(
    array_item: &toml::Value,
    ptr: *mut T,
    expected_len: i32,
    parser: F,
    name: &str,
    type_name: &str,
) -> Result<(), TomlLoadError>
where
    T: Sized,
    F: Fn(&toml::Value) -> Option<T>,
{
    if let Some(array) = array_item.as_array() {
        let usize_expected_len = expected_len as usize;
        if array.len() != usize_expected_len {
            return Err(TomlLoadError::ArrayLengthMismatch {
                name: name.to_string(),
                expected: usize_expected_len,
                actual: array.len(),
            });
        }

        for (index, value) in array.iter().enumerate() {
            if let Some(parsed) = parser(value) {
                unsafe {
                    ptr.add(index).write(parsed);
                }
            } else {
                return Err(TomlLoadError::TypeMismatch {
                    name: name.to_string(),
                    expected: type_name.to_string(),
                });
            }
        }

        Ok(())
    } else {
        Err(TomlLoadError::NotAnArray(name.to_string()))
    }
}
