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

use crate::{
    print_err::{print_err, print_err_header},
    runner::{CompileEvent, ui::error_formatting::indicate_error},
};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::{sync::mpsc, time::Duration};

pub fn run_event_loop(
    iterations: i32,
    file_path: &str,
    rx: mpsc::Receiver<CompileEvent>,
    ready_tx: mpsc::Sender<()>,
    preferred_lang: String,
) {
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
                    "Building finished".bright_green().bold(),
                    elapsed.as_secs_f32().yellow().bold()
                );
                ready_tx.send(()).unwrap();
            }
            CompileEvent::Running => {
                spinner.set_message("Running...");
                spinner.enable_steady_tick(Duration::from_millis(80));
            }
            CompileEvent::Finished {
                exec_elapsed,
                avg_elapsed,
            } => {
                spinner.finish_and_clear();
                if iterations > 1 {
                    println!(
                        "{} {} times in {}μs (avg: {}ns)\n",
                        "Executed".bright_green().bold(),
                        iterations.cyan().bold(),
                        exec_elapsed.as_micros().yellow().bold(),
                        avg_elapsed.as_nanos().yellow().bold(),
                    );
                } else {
                    println!(
                        "{} in {}μs\n",
                        "Executed".bright_green().bold(),
                        exec_elapsed.as_micros().yellow().bold(),
                    );
                }
                ready_tx.send(()).unwrap();
            }
            CompileEvent::KaslError(errors, source) => {
                spinner.finish_and_clear();
                print_err_header(Some(&format!("{} errors", errors.len().bold())));
                println!();
                for (index, record) in errors.iter().enumerate() {
                    indicate_error(record, file_path, &source, &preferred_lang);

                    if index < errors.len() - 1 {
                        println!();
                    }
                }
            }
            CompileEvent::Error(e) => {
                spinner.finish_and_clear();
                print_err(e);
            }
        }
    }
}
