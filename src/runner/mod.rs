mod compile_event;
mod file_utils;
mod inputs;
mod outputs;
mod ptr_utils;

use compile_event::CompileEvent;
use indicatif::ProgressBar;
use kasl::KaslCompiler;
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::Duration,
};

use crate::runner::{
    inputs::{InputError, ask_for_inputs},
    outputs::print_outputs,
    ptr_utils::{deallocate_blueprint_ptr, get_blueprint_ptr},
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
        let outputs = get_blueprint_ptr(blueprint.get_outputs());
        let states = get_blueprint_ptr(blueprint.get_states());

        // Run the program with the given inputs
        compiler.run(&blueprint, &inputs, &outputs, &states);

        print_outputs(&blueprint, &outputs, &compiler.prog_ctx.type_registry);

        deallocate_blueprint_ptr(blueprint.get_outputs(), outputs);
        deallocate_blueprint_ptr(blueprint.get_states(), states);
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
