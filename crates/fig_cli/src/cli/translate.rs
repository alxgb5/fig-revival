use std::process::ExitCode;

use clap::Args;
use eyre::Result;

#[derive(Debug, Args, Default, PartialEq, Eq)]
pub struct TranslateArgs {
    input: Vec<String>,
    /// Number of completions to generate (must be <=5)
    #[arg(short, long, hide = true)]
    n: Option<i32>,
}

impl TranslateArgs {
    pub fn new(input: Vec<String>) -> Self {
        Self {
            input,
            ..Default::default()
        }
    }

    pub async fn execute(self) -> Result<ExitCode> {
        eprintln!("⚠️  The 'translate' command requires AWS API which has been removed in Fig Local Revival.");
        eprintln!("This is a local-only version focused on terminal autocomplete.");
        Ok(ExitCode::FAILURE)
    }
}
