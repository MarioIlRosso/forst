use anyhow::Result;
use clap::Parser;
use forst_config::tree_config::TreeConfig;
use forst_core::walk_tree;
use std::io::BufWriter;
use std::io::{Write, stdout};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "forst", version, about = "A simple rust tree viewer")]
struct Cli {
    path: PathBuf,

    #[arg(short = 'a', long)]
    all: bool,

    #[arg(short = 'L', long)]
    max_depth: Option<usize>,

    #[arg(long = "no-unicode")]
    no_unicode: bool,

    #[arg(short = 's', long = "sizes")]
    display_sizes: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = TreeConfig {
        display_hidden: cli.all,
        max_depth: cli.max_depth,
        no_unicode: cli.no_unicode,
        display_sizes: cli.display_sizes,
    };

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    walk_tree(&cli.path, 0, &config, &mut writer)?;
    writer.flush()?;
    Ok(())
}
