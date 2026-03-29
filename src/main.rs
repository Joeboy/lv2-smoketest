use clap::Parser;
use std::io::{self, Write};
use std::process::ExitCode;

use lv2_smoketest::cli::{Cli, Commands};

enum CommandOutput {
    Text(String),
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::ShowPluginMetadata { plugin_uri } => {
            lv2_smoketest::show_plugin_metadata_json(&plugin_uri).map(CommandOutput::Text)
        }
        Commands::ListInstalledPlugins => {
            lv2_smoketest::list_installed_plugins_json().map(CommandOutput::Text)
        }
        Commands::TestPluginRun {
            plugin_uri,
            sample_rate,
            frames,
        } => lv2_smoketest::test_plugin_run(&plugin_uri, sample_rate, frames)
            .map(|()| {
                CommandOutput::Text(format!(
                    "Success: run() completed for {plugin_uri} (sample_rate={sample_rate}, frames={frames})"
                ))
            }),
    };

    match result {
        Ok(output) => match write_output(output) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("Error: failed to write output: {error}");
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!("Error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn write_output(output: CommandOutput) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    match output {
        CommandOutput::Text(text) => writeln!(stdout, "{text}"),
    }
}
