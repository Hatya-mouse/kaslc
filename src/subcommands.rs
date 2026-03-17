use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Installs a standard library of KASL in the directory set by the KASL_STD_PATH environment variable,
    /// or the path given by the --std-path option
    Install {
        #[arg(long)]
        std_path: Option<String>,
    },

    /// Compiles and runs a target KASL file.
    Run { target_path: String },

    /// Prints the path to the standard library.
    StdPath,
}
