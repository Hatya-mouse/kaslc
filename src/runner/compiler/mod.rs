pub(super) mod ptr_utils;

use crate::{
    print_err::print_err,
    runner::{
        CompileEvent,
        compiler::ptr_utils::{deallocate_blueprint_ptr, get_blueprint_ptr},
        io::{inputs::ask_for_inputs, outputs::print_outputs, toml_io::load_inputs_from_toml},
    },
};
use kasl::KaslCompiler;
use std::{path::PathBuf, sync::mpsc, thread, time::Duration};

pub(super) fn spawn_compiler_thread(
    std_path: PathBuf,
    input_path: Option<PathBuf>,
    code: String,
    iterations: usize,
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

        // If the input path is set, get the inputs from the file
        let inputs = if let Some(input_path) = input_path {
            match load_inputs_from_toml(&blueprint, &input_path) {
                Ok(inputs) => inputs,
                Err(e) => {
                    print_err(e);
                    return;
                }
            }
        } else {
            // Ask for inputs
            match ask_for_inputs(&blueprint, &compiler.prog_ctx.type_registry) {
                Ok(inputs) => inputs,
                Err(e) => {
                    print_err(e);
                    return;
                }
            }
        };

        let outputs = get_blueprint_ptr(blueprint.get_outputs());
        let states = get_blueprint_ptr(blueprint.get_states());

        println!();

        // Measure the elapsed time of execution
        tx.send(CompileEvent::Running).unwrap();

        // Run the program with the given inputs
        let mut iter_elapsed = Vec::new();
        for _ in 0..iterations {
            let exec_start = std::time::Instant::now();
            if let Err(e) = compiler.run(&inputs, &outputs, &states, 1) {
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

        deallocate_blueprint_ptr(blueprint.get_outputs(), outputs);
        deallocate_blueprint_ptr(blueprint.get_states(), states);
    });
}
