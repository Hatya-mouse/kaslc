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
        iterations: usize,
        /// Path to the file to load as input.
        #[arg(long)]
        input: Option<String>,
        // Path to the file to write output to.
        // #[arg(long)]
        // output: Option<String>,
    },

    /// Runs a target KASL files multiple times, and calculate min/max/average execution time and the standard deviation.
    /// With this command, the state variables will be reset between each run. If you want to run the target KASL file without resetting the state variables,
    /// use the `run` command with `iterations` argument.
    Bench {
        /// Path to the target KASL file to run.
        target_path: String,
        /// Number of times to run the target KASL file.
        #[arg(long, short, default_value = "100")]
        iterations: usize,
        /// Path to the file to load as input.
        #[arg(long)]
        input: Option<String>,
        // Path to the file to write output to.
        // #[arg(long)]
        // output: Option<String>,
    },

    /// Prints the path to the standard library.
    StdPath,
}
