use crate::{print_err::print_err, runner::CompileEvent};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::{sync::mpsc, time::Duration};

pub fn run_event_loop(
    iterations: usize,
    rx: mpsc::Receiver<CompileEvent>,
    ready_tx: mpsc::Sender<()>,
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
                    "Building finished".green().bold(),
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
                max_elapsed,
                min_elapsed,
                avg_elapsed,
            } => {
                spinner.finish_and_clear();
                if iterations > 1 {
                    println!(
                        "{} {} times in {}μs (max: {}ns, min: {}ns, avg: {}ns)\n",
                        "Executed".green().bold(),
                        iterations.cyan().bold(),
                        exec_elapsed.as_micros().yellow().bold(),
                        max_elapsed.as_nanos().yellow().bold(),
                        min_elapsed.as_nanos().yellow().bold(),
                        avg_elapsed.as_nanos().yellow().bold(),
                    );
                } else {
                    println!(
                        "{} in {}μs\n",
                        "Executed".green().bold(),
                        exec_elapsed.as_micros().yellow().bold(),
                    );
                }
                ready_tx.send(()).unwrap();
            }
            CompileEvent::Error(e) => {
                print_err(e);
            }
        }
    }
}
