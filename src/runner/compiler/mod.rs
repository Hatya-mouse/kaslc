pub(super) mod ptr_utils;

use crate::{
    print_err::print_err,
    runner::{
        CompileEvent,
        compiler::ptr_utils::{
            deallocate_blueprint_ptr, deallocate_buffer_blueprint_ptr, get_blueprint_ptr,
            get_buffer_blueprint_ptr,
        },
        io::{inputs::ask_for_inputs, outputs::print_outputs, toml_io::load_inputs_from_toml},
    },
};
use kasl::KaslCompiler;
use std::{path::PathBuf, sync::mpsc, thread, time::Duration};

pub(super) fn spawn_compiler_thread(
    std_path: PathBuf,
    input_path: Option<PathBuf>,
    code: String,
    iterations: i32,
    tx: mpsc::Sender<CompileEvent>,
    ready_rx: mpsc::Receiver<()>,
) {
    thread::spawn(move || {
        // Create a compiler and run the code
        let mut compiler = KaslCompiler::default();
        compiler.add_search_path(std_path);

        // Measure the elapsed time
        let build_start = std::time::Instant::now();

        // Notify the main thread that parsing has started
        tx.send(CompileEvent::Parsing).unwrap();
        if let Err(e) = compiler.parse(&code) {
            tx.send(CompileEvent::KaslError(vec![*e], code)).unwrap();
            return;
        }

        // Notify the main thread that building has started
        tx.send(CompileEvent::Building).unwrap();
        let blueprint = match compiler.build() {
            Ok(blueprint) => blueprint,
            Err(e) => {
                tx.send(CompileEvent::KaslError(e, code)).unwrap();
                return;
            }
        };

        // Compile the blueprint
        if let Err(e) = compiler.compile_buffer(&blueprint) {
            tx.send(CompileEvent::KaslError(e, code)).unwrap();
            return;
        }

        let build_elapsed = build_start.elapsed();
        tx.send(CompileEvent::Builded(build_elapsed)).unwrap();
        ready_rx.recv().unwrap();

        // If the input path is set, get the inputs from the file
        let inputs = if let Some(input_path) = input_path {
            match load_inputs_from_toml(&blueprint, iterations, &input_path) {
                Ok(inputs) => inputs,
                Err(e) => {
                    print_err(e);
                    return;
                }
            }
        } else {
            println!("Asking user for inputs");
            // Ask for inputs
            match ask_for_inputs(&blueprint, iterations, &compiler.prog_ctx.type_registry) {
                Ok(inputs) => inputs,
                Err(e) => {
                    print_err(e);
                    return;
                }
            }
        };

        let outputs = get_buffer_blueprint_ptr(blueprint.get_outputs(), iterations as usize);
        let states = get_blueprint_ptr(blueprint.get_states());

        println!();

        // Measure the elapsed time of execution
        tx.send(CompileEvent::Running).unwrap();

        // Run the program with the given inputs
        let mut iter_elapsed = Vec::new();

        for i in 0..iterations {
            let should_init = if i == 0 { 1i8 } else { 0i8 };

            let exec_start = std::time::Instant::now();
            // Run the program with the given inputs
            if let Err(e) = compiler.run_once(&inputs, &outputs, &states, should_init) {
                tx.send(CompileEvent::Error(e)).unwrap();
                return;
            }
            // Measure the elapsed time of execution
            iter_elapsed.push(exec_start.elapsed());
        }

        // Calculate the total elapsed time and average
        let exec_elapsed = iter_elapsed.iter().fold(Duration::default(), |a, b| a + *b);
        let max_elapsed = *iter_elapsed.iter().max().unwrap();
        let min_elapsed = *iter_elapsed.iter().min().unwrap();
        let avg_elapsed = exec_elapsed / iterations as u32;
        tx.send(CompileEvent::Finished {
            exec_elapsed,
            max_elapsed,
            min_elapsed,
            avg_elapsed,
        })
        .unwrap();
        ready_rx.recv().unwrap();

        print_outputs(&blueprint, &outputs, &compiler.prog_ctx.type_registry);

        deallocate_buffer_blueprint_ptr(blueprint.get_outputs(), outputs, iterations as usize);
        deallocate_blueprint_ptr(blueprint.get_states(), states);
    });
}
