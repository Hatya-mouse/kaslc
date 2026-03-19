use crate::runner::{
    io::blueprint_input::{alloc_and_spread, alloc_and_write_each},
    ui::input_ui::{
        print_entered_input, print_input_header, print_inputs, prompt_input_buffer,
        prompt_input_spread,
    },
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

pub fn ask_for_inputs_buffer(
    blueprint: &IOBlueprint,
    iterations: i32,
    type_registry: &TypeRegistry,
) -> Result<Vec<*mut ()>, InputError> {
    let inputs = blueprint.get_inputs();
    let mut ptrs = Vec::with_capacity(inputs.len());

    print_input_header();
    print_inputs(&inputs, type_registry);
    println!();

    for input in inputs {
        let mut str_values = Vec::new();

        match input.value_type {
            ResolvedType::Primitive(prim_type) => match prim_type {
                PrimitiveType::Bool => {
                    ptrs.push(alloc_and_write_each(iterations as usize, |index| {
                        prompt_input_buffer(input, type_registry, index, iterations);
                        let bool_val = ask_for_value::<bool>();
                        str_values.push(bool_val.to_string());
                        if bool_val { 1i8 } else { 0i8 }
                    }));
                }
                PrimitiveType::Float => {
                    ptrs.push(alloc_and_write_each(iterations as usize, |index| {
                        prompt_input_buffer(input, type_registry, index, iterations);
                        let float_val = ask_for_value::<f32>();
                        str_values.push(float_val.to_string());
                        float_val
                    }));
                }
                PrimitiveType::Int => {
                    ptrs.push(alloc_and_write_each(iterations as usize, |index| {
                        prompt_input_buffer(input, type_registry, index, iterations);
                        let int_val = ask_for_value::<i32>();
                        str_values.push(int_val.to_string());
                        int_val
                    }));
                }
                PrimitiveType::Void => {
                    return Err(InputError::VoidInput);
                }
            },
            ResolvedType::Struct(_) => {
                unreachable!("This should have been caught by the any() check above")
            }
        }

        print_entered_input(input, &format!("[{}]", str_values.join(", ")));
    }

    Ok(ptrs)
}

pub fn ask_for_inputs_spread(
    blueprint: &IOBlueprint,
    iterations: i32,
    type_registry: &TypeRegistry,
) -> Result<Vec<*mut ()>, InputError> {
    let inputs = blueprint.get_inputs();
    let mut ptrs = Vec::with_capacity(inputs.len());

    print_input_header();
    print_inputs(&inputs, type_registry);
    println!();

    for input in inputs {
        let str_value = match input.value_type {
            ResolvedType::Primitive(prim_type) => match prim_type {
                PrimitiveType::Bool => {
                    prompt_input_spread(input, type_registry);
                    let bool_val = ask_for_value::<bool>();
                    ptrs.push(alloc_and_spread(
                        iterations as usize,
                        if bool_val { 1i8 } else { 0i8 },
                    ));
                    bool_val.to_string()
                }
                PrimitiveType::Float => {
                    prompt_input_spread(input, type_registry);
                    let float_val = ask_for_value::<f32>();
                    ptrs.push(alloc_and_spread(iterations as usize, float_val));
                    float_val.to_string()
                }
                PrimitiveType::Int => {
                    prompt_input_spread(input, type_registry);
                    let int_val = ask_for_value::<i32>();
                    ptrs.push(alloc_and_spread(iterations as usize, int_val));
                    int_val.to_string()
                }
                PrimitiveType::Void => {
                    return Err(InputError::VoidInput);
                }
            },
            ResolvedType::Struct(_) => {
                unreachable!("This should have been caught by the any() check above")
            }
        };

        print_entered_input(input, &str_value);
    }

    Ok(ptrs)
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
