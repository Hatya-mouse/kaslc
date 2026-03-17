use kasl::{
    scope_manager::IOBlueprint,
    type_registry::{PrimitiveType, ResolvedType},
};
use std::{
    alloc::{Layout, alloc},
    io,
    str::FromStr,
};

pub(super) enum InputError {
    /// Non-primitive input type is not supported on kaslc.
    NonPrimitiveInput,
    /// Void input type is not allowed.
    VoidInput,
}

pub(super) fn ask_for_inputs(blueprint: &IOBlueprint) -> Result<Vec<*mut ()>, InputError> {
    let inputs = blueprint.get_inputs();

    // If the input has non-primitive type, warn user and skip asking for input
    if inputs
        .iter()
        .any(|input| matches!(input.value_type, ResolvedType::Struct(_)))
    {
        return Err(InputError::NonPrimitiveInput);
    }

    let mut parsed_inputs = Vec::new();
    for (index, input) in inputs.iter().enumerate() {
        match input.value_type {
            ResolvedType::Primitive(prim_type) => match prim_type {
                PrimitiveType::Bool => {
                    let value = ask_for_bool(index);
                    parsed_inputs.push(alloc_for_type(value));
                }
                PrimitiveType::Float => {
                    let value: f32 = ask_for_number(index);
                    parsed_inputs.push(alloc_for_type(value));
                }
                PrimitiveType::Int => {
                    let value: i32 = ask_for_number(index);
                    parsed_inputs.push(alloc_for_type(value));
                }
                PrimitiveType::Void => {
                    return Err(InputError::VoidInput);
                }
            },
            ResolvedType::Struct(_) => {
                unreachable!("This should have been caught by the any() check above")
            }
        }
    }

    Ok(parsed_inputs)
}

fn ask_for_bool(index: usize) -> bool {
    loop {
        let mut input_str = String::new();

        // Read the user'a input
        println!("Enter Bool input for the input #{}", index);
        io::stdin().read_line(&mut input_str).unwrap();

        // Parse the input
        match input_str.as_str() {
            "t" => return true,
            "f" => return false,
            _ => println!("Invalid input. Please enter a valid boolean."),
        }
    }
}

fn ask_for_number<T: FromStr>(index: usize) -> T {
    loop {
        let mut input_str = String::new();

        // Read the user'a input
        println!("Enter Int input for the input #{}", index);
        io::stdin().read_line(&mut input_str).unwrap();

        // Parse the input
        match input_str.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn alloc_for_type<T: Sized>(value: T) -> *mut () {
    let layout = Layout::new::<T>();
    unsafe {
        let ptr = alloc(layout) as *mut T;
        ptr.write(value);
        ptr as *mut ()
    }
}
