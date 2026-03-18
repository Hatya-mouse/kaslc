use crate::runner::{
    compiler::ptr_utils::alloc_for_type,
    ui::input_ui::{print_entered_input, print_input_header, print_inputs, prompt_input},
};
use kasl::{
    scope_manager::IOBlueprint,
    type_registry::{PrimitiveType, ResolvedType, TypeRegistry},
};
use std::{
    fmt::{Display, Formatter},
    io::{self},
    str::FromStr,
};

pub enum InputError {
    /// Non-primitive input type is not supported on kaslc.
    NonPrimitiveInput,
    /// Void input type is not allowed.
    VoidInput,
}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::NonPrimitiveInput => {
                write!(f, "Non-primitive input type is not supported on kaslc.")
            }
            InputError::VoidInput => write!(f, "Void input type is not allowed."),
        }
    }
}

pub fn ask_for_inputs(
    blueprint: &IOBlueprint,
    type_registry: &TypeRegistry,
) -> Result<Vec<*mut ()>, InputError> {
    let inputs = blueprint.get_inputs();

    // Print the list of inputs first
    print_input_header();
    print_inputs(inputs, type_registry);

    // If the input has non-primitive type, warn user and skip asking for input
    if inputs
        .iter()
        .any(|input| matches!(input.value_type, ResolvedType::Struct(_)))
    {
        return Err(InputError::NonPrimitiveInput);
    }

    println!();

    let mut parsed_inputs = Vec::new();
    for input in inputs {
        prompt_input(input, type_registry);

        let str_value;
        match input.value_type {
            ResolvedType::Primitive(prim_type) => match prim_type {
                PrimitiveType::Bool => {
                    let value = ask_for_bool();
                    parsed_inputs.push(alloc_for_type(value));
                    str_value = value.to_string();
                }
                PrimitiveType::Float => {
                    let value: f32 = ask_for_number();
                    parsed_inputs.push(alloc_for_type(value));
                    str_value = value.to_string();
                }
                PrimitiveType::Int => {
                    let value: i32 = ask_for_number();
                    parsed_inputs.push(alloc_for_type(value));
                    str_value = value.to_string();
                }
                PrimitiveType::Void => {
                    return Err(InputError::VoidInput);
                }
            },
            ResolvedType::Struct(_) => {
                unreachable!("This should have been caught by the any() check above")
            }
        }

        print!("\x1b[1A\x1b[2K");
        print_entered_input(input, &str_value);
    }

    Ok(parsed_inputs)
}

fn ask_for_bool() -> bool {
    loop {
        let mut input_str = String::new();

        // Read the user'a input
        io::stdin().read_line(&mut input_str).unwrap();

        // Parse the input
        match input_str.trim().parse::<bool>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid boolean."),
        }
    }
}

fn ask_for_number<T: FromStr>() -> T {
    loop {
        let mut input_str = String::new();

        // Read the user'a input
        io::stdin().read_line(&mut input_str).unwrap();

        // Parse the input
        match input_str.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
