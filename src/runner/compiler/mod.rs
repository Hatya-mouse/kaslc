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

pub(super) mod ptr_utils;

use crate::{
    print_err::print_err,
    runner::{
        CompileEvent,
        compiler::ptr_utils::{
            deallocate_blueprint_ptr, deallocate_buffer_blueprint_ptr, get_blueprint_ptr,
            get_buffer_blueprint_ptr,
        },
        io::{
            outputs::print_outputs,
            toml_io::{load_inputs_buffer_from_toml, load_inputs_spread_from_toml},
            user_inputs::{InputError, ask_for_inputs_buffer, ask_for_inputs_spread},
        },
    },
};
use kasl::core::{
    KaslCompiler,
    ast::{
        scope_manager::IOBlueprint,
        type_registry::{ResolvedType, TypeRegistry},
    },
    run_program::run_buffer,
};
use kasl::cranelift_backend::CraneliftBackend;
use std::{path::PathBuf, sync::mpsc, thread};

pub(super) fn spawn_compiler_thread(
    std_path: PathBuf,
    input_path: Option<PathBuf>,
    code: String,
    iterations: i32,
    should_spread: bool,
    tx: mpsc::Sender<CompileEvent>,
    ready_rx: mpsc::Receiver<()>,
) {
    thread::spawn(move || {
        // Create a compiler and a backend, and run the code
        let mut compiler = KaslCompiler::default();
        let mut backend = CraneliftBackend::default();
        compiler.add_search_path(std_path);

        // Measure the elapsed time
        let build_start = std::time::Instant::now();

        // Compile the program
        let Some((program, blueprint)) = compile_kasl(&tx, &mut compiler, &mut backend, code)
        else {
            return;
        };

        let build_elapsed = build_start.elapsed();
        tx.send(CompileEvent::Builded(build_elapsed)).unwrap();
        ready_rx.recv().unwrap();

        // If the input has non-primitive type, warn user and skip asking for input
        if blueprint.get_inputs().iter().any(|input| {
            matches!(
                input.value_type,
                ResolvedType::Struct(_) | ResolvedType::Array(_)
            )
        }) {
            print_err(InputError::NonPrimitiveInput);
            return;
        }

        let inputs = match get_inputs(
            &blueprint,
            iterations,
            input_path,
            should_spread,
            &compiler.get_prog_ctx().type_registry,
        ) {
            Ok(inputs) => inputs,
            Err(e) => {
                tx.send(CompileEvent::Error(e)).unwrap();
                return;
            }
        };

        let outputs = get_buffer_blueprint_ptr(blueprint.get_outputs(), iterations as usize);
        let states = get_blueprint_ptr(blueprint.get_states());

        println!();
        tx.send(CompileEvent::Running).unwrap();

        // Measure the elapsed time of execution
        let exec_start = std::time::Instant::now();

        // Run the program with the given inputs
        unsafe {
            run_buffer(program, &inputs, &outputs, &states, 1, iterations);
        }

        // Measure the elapsed time of execution
        let exec_elapsed = exec_start.elapsed();

        // Calculate the total elapsed time and average
        let avg_elapsed = exec_elapsed / iterations as u32;
        tx.send(CompileEvent::Finished {
            exec_elapsed,
            avg_elapsed,
        })
        .unwrap();
        ready_rx.recv().unwrap();

        print_outputs(
            &blueprint,
            &outputs,
            iterations as usize,
            &compiler.get_prog_ctx().type_registry,
        );

        deallocate_buffer_blueprint_ptr(blueprint.get_outputs(), outputs, iterations as usize);
        deallocate_blueprint_ptr(blueprint.get_states(), states);
    });
}

fn compile_kasl(
    tx: &mpsc::Sender<CompileEvent>,
    compiler: &mut KaslCompiler,
    backend: &mut CraneliftBackend,
    code: String,
) -> Option<(*const u8, IOBlueprint)> {
    // Notify the main thread that parsing has started
    tx.send(CompileEvent::Parsing).unwrap();
    if let Err(e) = compiler.parse(&code) {
        tx.send(CompileEvent::KaslError(vec![*e], code)).unwrap();
        return None;
    }

    // Notify the main thread that building has started
    tx.send(CompileEvent::Building).unwrap();
    let blueprint = match compiler.build() {
        Ok((blueprint, e)) => {
            if !e.is_empty() {
                tx.send(CompileEvent::KaslWarning(e, code.clone())).unwrap();
            }
            blueprint
        }
        Err(e) => {
            tx.send(CompileEvent::KaslError(e, code)).unwrap();
            return None;
        }
    };

    // Compile the blueprint
    let func = match compiler.lower_buffer(&blueprint) {
        Ok(func) => func,
        Err(e) => {
            tx.send(CompileEvent::KaslError(vec![e], code)).unwrap();
            return None;
        }
    };

    let program = match backend.compile(func) {
        Ok(program) => program,
        Err(e) => {
            tx.send(CompileEvent::Error(e)).unwrap();
            return None;
        }
    };

    Some((program, blueprint))
}

fn get_inputs(
    blueprint: &IOBlueprint,
    iterations: i32,
    input_path: Option<PathBuf>,
    should_spread: bool,
    type_registry: &TypeRegistry,
) -> Result<Vec<*const ()>, String> {
    match (input_path, should_spread) {
        (Some(path), false) => {
            load_inputs_buffer_from_toml(blueprint, iterations, &path).map_err(|e| e.to_string())
        }
        (Some(path), true) => {
            load_inputs_spread_from_toml(blueprint, iterations, &path).map_err(|e| e.to_string())
        }
        (None, false) => {
            ask_for_inputs_buffer(blueprint, iterations, type_registry).map_err(|e| e.to_string())
        }
        (None, true) => {
            ask_for_inputs_spread(blueprint, iterations, type_registry).map_err(|e| e.to_string())
        }
    }
}
