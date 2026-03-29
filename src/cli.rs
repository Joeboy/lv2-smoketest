use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "lv2-smoketest")]
#[command(about = "LV2 plugin inspection and testing CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print metadata for an installed LV2 plugin URI as JSON.
    ShowPluginMetadata {
        /// Full LV2 plugin URI, e.g. http://example.org/plugins/my-plugin
        plugin_uri: String,
    },

    /// List installed LV2 plugins as JSON.
    ListInstalledPlugins,

    /// Instantiate a plugin and run it once to verify run() completes successfully.
    TestPluginRun {
        /// Full LV2 plugin URI, e.g. http://example.org/plugins/my-plugin
        plugin_uri: String,

        /// Sample rate used for instantiate/run.
        #[arg(long, default_value_t = 44_100.0)]
        sample_rate: f64,

        /// Number of frames to process.
        #[arg(long, default_value_t = 512)]
        frames: usize,
    },
}
