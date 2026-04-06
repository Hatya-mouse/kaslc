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

mod compile_event;
mod compiler;
mod io;
mod ui;

use crate::{
    print_err::print_err,
    runner::{io::file_utils, ui::event_loop::run_event_loop},
};
use compile_event::CompileEvent;
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
};

pub fn run_target(
    target_path: &Path,
    std_path: PathBuf,
    iterations: i32,
    should_spread: bool,
    input_file: Option<&String>,
    preferred_lang: String,
) {
    // Create a new mpsc channel
    let (tx, rx) = mpsc::channel();
    let (ready_tx, ready_rx) = mpsc::channel::<()>();
    // Get the file contents
    let code = match file_utils::get_file_contents(target_path) {
        Ok(code) => code,
        Err(e) => {
            print_err(e);
            return;
        }
    };

    // Create a compiler thread
    let input_path = input_file.map(PathBuf::from);
    compiler::spawn_compiler_thread(
        std_path,
        input_path,
        code,
        iterations,
        should_spread,
        tx,
        ready_rx,
    );
    run_event_loop(
        iterations,
        target_path.to_str().unwrap(),
        rx,
        ready_tx,
        preferred_lang,
    );
}
