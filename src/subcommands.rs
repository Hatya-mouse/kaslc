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

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Installs a standard library of KASL in the directory set by the KASL_STD_PATH environment variable,
    /// or the path given by the --std-path option
    Install {
        /// Path to the directory to install the standard library to.
        #[arg(long)]
        std_path: Option<String>,
    },

    /// Compiles and runs a target KASL file. This command preserves the state variables between runs.
    Run {
        /// Path to the target KASL file to run.
        target_path: String,
        /// Number of times to run the target KASL file.
        #[arg(long, short, default_value = "1")]
        iterations: i32,
        /// Path to the file to load as input.
        #[arg(long)]
        input: Option<String>,
        /// Whether not to spread the input across iterations.
        #[arg(long, default_value = "false")]
        no_spread: bool,
        // Path to the file to write output to.
        // #[arg(long)]
        // output: Option<String>,
    },

    /// Prints the path to the standard library.
    StdPath,
}
