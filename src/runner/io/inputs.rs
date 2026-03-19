use crate::runner::{
    compiler::ptr_utils::alloc_buf_for_type,
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
    iterations: i32,
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

    let mut ptrs = Vec::with_capacity(inputs.len());
    for input in inputs {
        let mut str_value = String::new();

        match input.value_type {
            ResolvedType::Primitive(prim_type) => match prim_type {
                PrimitiveType::Bool => {
                    for index in 0..iterations as usize {
                        prompt_input(input, type_registry, index, iterations);
                        let value = if ask_for_value::<bool>() { 1 } else { 0 };
                        str_value
                            .push_str(&ask_and_write::<i8>(&mut ptrs, value, index, iterations));
                    }
                }
                PrimitiveType::Float => {
                    for index in 0..iterations as usize {
                        prompt_input(input, type_registry, index, iterations);
                        let value = ask_for_value::<f32>();
                        str_value
                            .push_str(&ask_and_write::<f32>(&mut ptrs, value, index, iterations));
                    }
                }
                PrimitiveType::Int => {
                    for index in 0..iterations as usize {
                        prompt_input(input, type_registry, index, iterations);
                        let value = ask_for_value::<i32>();
                        str_value
                            .push_str(&ask_and_write::<i32>(&mut ptrs, value, index, iterations));
                    }
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
        print_entered_input(input, &format!("[{}]", str_value));
    }

    Ok(ptrs)
}

fn ask_and_write<T: FromStr + Display>(
    ptrs: &mut Vec<*mut ()>,
    value: T,
    index: usize,
    iterations: i32,
) -> String {
    // Allocate a buffer for the type
    let ptr = alloc_buf_for_type::<T>(iterations as usize);
    // Write the value to the buffer
    let value_string = value.to_string();
    unsafe {
        ptr.add(index).write(value);
    }
    // Push the pointer to the vector
    ptrs.push(ptr as *mut ());
    // Stringify the value and return it
    value_string
}

fn ask_for_value<T: FromStr>() -> T {
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
