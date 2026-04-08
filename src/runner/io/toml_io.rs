//
//  Copyright 2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::runner::{
    file_utils::{FileLoadError, get_file_contents},
    io::blueprint_input::{alloc_and_spread, alloc_and_write_each},
    ui::input_ui::{print_entered_input, print_input_header},
};
use kasl::core::ast::{
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

pub fn load_inputs_buffer_from_toml(
    blueprint: &IOBlueprint,
    iterations: i32,
    path: &Path,
) -> Result<Vec<*const ()>, TomlLoadError> {
    let content = get_file_contents(path).map_err(TomlLoadError::FileLoadError)?;
    let table: HashMap<String, toml::Value> =
        toml::from_str(&content).map_err(|err| TomlLoadError::ParseError(err.to_string()))?;
    let inputs = blueprint.get_inputs();

    // Initialize the pointer vector with capacity
    let mut ptrs: Vec<*const ()> = Vec::with_capacity(inputs.len());

    print_input_header();

    for input in inputs {
        let raw_val = table
            .get(&input.name)
            .ok_or_else(|| TomlLoadError::MissingField(input.name.clone()))?;

        match &input.value_type {
            ResolvedType::Primitive(PrimitiveType::Bool) => {
                let parsed_array = parse_array::<i8, _>(
                    raw_val,
                    iterations,
                    |v| v.as_bool().map(|b| if b { 1 } else { 0 }),
                    &input.name,
                    "Bool",
                )?;
                let ptr = alloc_and_write_each(iterations as usize, |index| parsed_array[index]);
                ptrs.push(ptr);
            }
            ResolvedType::Primitive(PrimitiveType::Float) => {
                let parsed_array = parse_array::<f32, _>(
                    raw_val,
                    iterations,
                    |v| v.as_float().map(|f| f as f32),
                    &input.name,
                    "Float",
                )?;
                let ptr = alloc_and_write_each(iterations as usize, |index| parsed_array[index]);
                ptrs.push(ptr);
            }
            ResolvedType::Primitive(PrimitiveType::Int) => {
                let parsed_array = parse_array::<i32, _>(
                    raw_val,
                    iterations,
                    |v| v.as_integer().map(|i| i as i32),
                    &input.name,
                    "Int",
                )?;
                let ptr = alloc_and_write_each(iterations as usize, |index| parsed_array[index]);
                ptrs.push(ptr);
            }
            ResolvedType::Primitive(PrimitiveType::Void) => {
                return Err(TomlLoadError::VoidInput);
            }
            ResolvedType::Struct(_) | ResolvedType::Array(_) => {
                return Err(TomlLoadError::NonPrimitiveInput);
            }
        };

        print_entered_input(input, &raw_val.to_string());
    }

    Ok(ptrs)
}

pub fn load_inputs_spread_from_toml(
    blueprint: &IOBlueprint,
    iterations: i32,
    path: &Path,
) -> Result<Vec<*const ()>, TomlLoadError> {
    let content = get_file_contents(path).map_err(TomlLoadError::FileLoadError)?;
    let table: HashMap<String, toml::Value> =
        toml::from_str(&content).map_err(|err| TomlLoadError::ParseError(err.to_string()))?;
    let inputs = blueprint.get_inputs();

    // Initialize the pointer vector with capacity
    let mut ptrs: Vec<*const ()> = Vec::with_capacity(inputs.len());

    print_input_header();

    for input in inputs {
        let raw_val = table
            .get(&input.name)
            .ok_or_else(|| TomlLoadError::MissingField(input.name.clone()))?;

        match &input.value_type {
            ResolvedType::Primitive(PrimitiveType::Bool) => {
                let bool_val = parse_scalar(
                    raw_val,
                    |v| v.as_bool().map(|b| if b { 1 } else { 0 }),
                    &input.name,
                    "Bool",
                )?;
                ptrs.push(alloc_and_spread(iterations as usize, bool_val));
            }
            ResolvedType::Primitive(PrimitiveType::Float) => {
                let float_val = parse_scalar(
                    raw_val,
                    |v| v.as_float().map(|f| f as f32),
                    &input.name,
                    "Float",
                )?;
                ptrs.push(alloc_and_spread(iterations as usize, float_val));
            }
            ResolvedType::Primitive(PrimitiveType::Int) => {
                let int_val = parse_scalar(
                    raw_val,
                    |v| v.as_integer().map(|i| i as i32),
                    &input.name,
                    "Int",
                )?;
                ptrs.push(alloc_and_spread(iterations as usize, int_val));
            }
            ResolvedType::Primitive(PrimitiveType::Void) => {
                return Err(TomlLoadError::VoidInput);
            }
            ResolvedType::Struct(_) | ResolvedType::Array(_) => {
                return Err(TomlLoadError::NonPrimitiveInput);
            }
        };

        print_entered_input(input, &raw_val.to_string());
    }

    Ok(ptrs)
}

fn parse_array<T, F>(
    array_item: &toml::Value,
    expected_len: i32,
    parser: F,
    name: &str,
    type_name: &str,
) -> Result<Vec<T>, TomlLoadError>
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

        let mut result = Vec::with_capacity(array.len());
        for value in array.iter() {
            if let Some(parsed) = parser(value) {
                result.push(parsed);
            } else {
                return Err(TomlLoadError::TypeMismatch {
                    name: name.to_string(),
                    expected: type_name.to_string(),
                });
            }
        }

        Ok(result)
    } else {
        Err(TomlLoadError::NotAnArray(name.to_string()))
    }
}

fn parse_scalar<T, F>(
    value: &toml::Value,
    parser: F,
    name: &str,
    type_name: &str,
) -> Result<T, TomlLoadError>
where
    F: Fn(&toml::Value) -> Option<T>,
{
    parser(value).ok_or_else(|| TomlLoadError::TypeMismatch {
        name: name.to_string(),
        expected: type_name.to_string(),
    })
}
