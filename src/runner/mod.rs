mod compile_event;
mod file_utils;
mod inputs;
mod outputs;
mod print_utils;
mod ptr_utils;

use crate::runner::{
    inputs::{InputError, ask_for_inputs},
    outputs::print_outputs,
    ptr_utils::{deallocate_blueprint_ptr, get_blueprint_ptr},
};
use compile_event::CompileEvent;
use indicatif::{ProgressBar, ProgressStyle};
use kasl::KaslCompiler;
use owo_colors::OwoColorize;
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::Duration,
};

pub fn run_target(target_path: &Path, std_path: PathBuf) {
    // Create a new mpsc channel
    let (tx, rx) = mpsc::channel();
    let (ready_tx, ready_rx) = mpsc::channel::<()>();
    // Get the file contents
    let code = file_utils::get_file_contents(target_path).unwrap();

    // Create a compiler thread
    thread::spawn(move || {
        // Create a compiler and run the code
        let mut compiler = KaslCompiler::default();
        compiler.add_search_path(std_path.to_path_buf());

        // Measure the elapsed time
        let build_start = std::time::Instant::now();

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

        // Compile the blueprint
        if let Err(e) = compiler.compile(&blueprint) {
            tx.send(CompileEvent::Error(format!("{:#?}", e))).unwrap();
            return;
        }

        let build_elapsed = build_start.elapsed();
        tx.send(CompileEvent::Builded(build_elapsed)).unwrap();
        ready_rx.recv().unwrap();

        // Ask for inputs
        let inputs = match ask_for_inputs(&blueprint, &compiler.prog_ctx.type_registry) {
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

        println!();

        // Measure the elapsed time of execution
        let exec_start = std::time::Instant::now();

        // Run the program with the given inputs
        if let Err(e) = compiler.run(&inputs, &outputs, &states, 1) {
            tx.send(CompileEvent::Error(e)).unwrap();
            return;
        }

        // Measure the elapsed time of execution
        let exec_elapsed = exec_start.elapsed();
        println!(
            "{} in {}μs\n",
            "Executed".yellow().bold(),
            exec_elapsed.as_micros().yellow().bold()
        );

        print_outputs(&blueprint, &outputs, &compiler.prog_ctx.type_registry);

        deallocate_blueprint_ptr(blueprint.get_outputs(), outputs);
        deallocate_blueprint_ptr(blueprint.get_states(), states);
    });

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());

    for event in rx {
        match event {
            CompileEvent::Parsing => {
                spinner.set_message("Parsing...");
                spinner.enable_steady_tick(Duration::from_millis(80));
            }
            CompileEvent::Building => {
                spinner.set_message("Building...");
                spinner.enable_steady_tick(Duration::from_millis(80));
            }
            CompileEvent::Builded(elapsed) => {
                spinner.finish_and_clear();
                println!(
                    "{} in {:.5}s\n",
                    "Building finished".green().bold(),
                    elapsed.as_secs_f32().yellow().bold()
                );
                ready_tx.send(()).unwrap();
            }
            CompileEvent::Error(e) => {
                eprintln!("{}", " ERROR ".on_red().bold());
                eprintln!("{}", e);
            }
        }
    }
}
