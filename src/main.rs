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

mod cli;
mod highlighter;
mod print_err;
mod runner;
mod std_installer;
mod subcommands;

use crate::{print_err::print_err, runner::run_target, std_installer::install_std};
use clap::Parser;
use cli::Cli;
use owo_colors::OwoColorize;
use rust_embed::RustEmbed;
use std::{env, path::Path};
use subcommands::Subcommands;

#[derive(RustEmbed)]
#[folder = "kasl-std/std"]
struct StdLib;

fn main() {
    let cli = Cli::parse();

    let preferred_lang = cli.lang.unwrap_or_else(|| {
        std::env::var("LANG")
            .unwrap_or_default()
            .split('_')
            .next()
            .unwrap_or("en")
            .to_string()
    });

    // Get the KASL_STD_PATH environment variable, or set a default path
    let default_std_path = match env::var("KASL_STD_PATH") {
        Ok(value) => value,
        Err(_) => {
            if let Some(home_path) = dirs::home_dir() {
                let default_path = home_path.join(".kasl").join("std");
                unsafe {
                    env::set_var("KASL_STD_PATH", &default_path);
                }
                default_path.to_string_lossy().into_owned()
            } else {
                print_err("Could not determine home directory for default KASL_STD_PATH");
                return;
            }
        }
    };

    match &cli.command {
        Subcommands::Install { std_path } => {
            let copy_path = Path::new(std_path.as_ref().unwrap_or(&default_std_path));
            install_std(copy_path).unwrap();
        }
        Subcommands::Run {
            target_path,
            iterations,
            input,
            no_spread,
        } => {
            if *iterations < 1 {
                print_err("Iterations must be greater than 0");
                return;
            }

            let target_path = Path::new(target_path);
            let std_path = Path::new(&default_std_path);
            run_target(
                target_path,
                std_path.to_path_buf(),
                *iterations,
                !*no_spread,
                input.as_ref(),
                preferred_lang,
            );
        }
        Subcommands::StdPath => {
            println!(
                "Path to {}: {}",
                "std".bright_green().bold(),
                default_std_path
            );
        }
    }
}
