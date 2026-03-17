mod compile_event;
mod file_utils;

use compile_event::CompileEvent;
use indicatif::ProgressBar;
use kasl::{
    KaslCompiler,
    scope_manager::{BlueprintItem, IOBlueprint},
    type_registry::{PrimitiveType, ResolvedType},
};
use std::{
    alloc::{Layout, alloc, dealloc},
    io,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::Duration,
};

pub fn run_target(target_path: &Path, std_path: PathBuf) {
    // Create a new mpsc channel
    let (tx, rx) = mpsc::channel();
    // Get the file contents
    let code = file_utils::get_file_contents(target_path).unwrap();

    // Create a compiler thread
    thread::spawn(move || {
        // Create a compiler and run the code
        let mut compiler = KaslCompiler::default();
        compiler.add_search_path(std_path.to_path_buf());

        // Notify the main thread that parsing has started
        tx.send(CompileEvent::Parsing).unwrap();
        if let Err(e) = compiler.parse(&code) {
            tx.send(CompileEvent::Error(e.to_string())).unwrap();
            return;
        }

        // Notify the main thread that building has started
        tx.send(CompileEvent::Building).unwrap();
        let blueprint = match compiler.build() {
            Ok(blueprint) => blueprint,
            Err(e) => {
                tx.send(CompileEvent::Error(format!("{:#?}", e))).unwrap();
                return;
            }
        };

        // Ask for inputs
        let inputs = match ask_for_inputs(&blueprint) {
            Ok(inputs) => inputs,
            Err(e) => match e {
                InputError::NonPrimitiveInput => {
                    eprintln!("Error: Non-primitive input type is not supported on kaslc.");
                    return;
                }
                InputError::VoidInput => {
                    eprintln!("Error: Void input type is not allowed.");
                    return;
                }
            },
        };
        let (outputs, output_layouts) = get_blueprint_ptr(blueprint.get_outputs());
        let (states, state_layouts) = get_blueprint_ptr(blueprint.get_states());

        // Run the program with the given inputs
        compiler.run(&blueprint, &inputs, &outputs, &states);

        println!("Outputs: {:?}", outputs);
        println!("States: {:?}", states);

        deallocate_blueprint_ptr(outputs, output_layouts);
        deallocate_blueprint_ptr(states, state_layouts);
    });

    for event in rx {
        match event {
            CompileEvent::Parsing => {
                let spinner = ProgressBar::new_spinner();
                spinner.set_message("Parsing...");
                spinner.enable_steady_tick(Duration::from_millis(80));
            }
            CompileEvent::Building => {
                let spinner = ProgressBar::new_spinner();
                spinner.set_message("Building...");
                spinner.enable_steady_tick(Duration::from_millis(80));
            }
            CompileEvent::Error(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

enum InputError {
    /// Non-primitive input type is not supported on kaslc.
    NonPrimitiveInput,
    /// Void input type is not allowed.
    VoidInput,
}

fn ask_for_inputs(blueprint: &IOBlueprint) -> Result<Vec<*mut ()>, InputError> {
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
                    let mut value = ask_for_bool(index);
                    parsed_inputs.push(&mut value as *mut bool as *mut ());
                }
                PrimitiveType::Float => {
                    let mut value = ask_for_float(index);
                    parsed_inputs.push(&mut value as *mut f32 as *mut ());
                }
                PrimitiveType::Int => {
                    let mut value = ask_for_int(index);
                    parsed_inputs.push(&mut value as *mut i32 as *mut ());
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

fn ask_for_float(index: usize) -> f32 {
    loop {
        let mut input_str = String::new();

        // Read the user'a input
        println!("Enter Float input for the input #{}", index);
        io::stdin().read_line(&mut input_str).unwrap();

        // Parse the input
        match input_str.trim().parse::<f32>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid float."),
        }
    }
}

fn ask_for_int(index: usize) -> i32 {
    loop {
        let mut input_str = String::new();

        // Read the user'a input
        println!("Enter Int input for the input #{}", index);
        io::stdin().read_line(&mut input_str).unwrap();

        // Parse the input
        match input_str.trim().parse::<i32>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn get_blueprint_ptr(items: &[BlueprintItem]) -> (Vec<*mut ()>, Vec<Layout>) {
    let mut ptrs: Vec<*mut ()> = Vec::with_capacity(items.len());
    let mut layouts: Vec<Layout> = Vec::with_capacity(items.len());
    for item in items {
        let layout = Layout::from_size_align(item.size, item.align as usize).unwrap();

        unsafe {
            let ptr: *mut u8 = alloc(layout);

            if ptr.is_null() {
                panic!("Failed to allocate memory for blueprint item");
            }

            let void_ptr = ptr as *mut ();
            ptrs.push(void_ptr);
            layouts.push(layout);
        }
    }
    (ptrs, layouts)
}

fn deallocate_blueprint_ptr(ptrs: Vec<*mut ()>, layouts: Vec<Layout>) {
    unsafe {
        for (ptr, layout) in ptrs.iter().zip(layouts) {
            if !ptr.is_null() {
                dealloc(*ptr as *mut u8, layout);
            }
        }
    }
}
