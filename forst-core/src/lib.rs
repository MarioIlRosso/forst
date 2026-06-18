use anyhow::Result;
use forst_config::{config_values, tree_config::TreeConfig};
use std::{
    fs::read_dir,
    io::{BufWriter, Write, stdout},
    path::Path,
};

fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{}B", bytes)
    } else {
        format!("{:.1}{}", size, UNITS[unit])
    }
}

pub fn walk_tree(
    path: &Path,
    mut depth: usize,
    config: &TreeConfig,
    writer: &mut impl Write,
) -> Result<()> {
    let metadata = path.metadata()?;

    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("?");

    if file_name.starts_with(".") && !config.display_hidden {
        return Ok(());
    }

    let (char, symbol): (char, char) = {
        if metadata.is_dir() {
            if config.no_unicode {
                ('|', config_values::DIR_SYM)
            } else {
                (config_values::DIR_CHAR, config_values::DIR_SYM)
            }
        } else if metadata.is_file() {
            if config.no_unicode {
                ('-', config_values::FILE_SYM)
            } else {
                (config_values::FILE_CHAR, config_values::FILE_SYM)
            }
        } else {
            ('?', '?')
        }
    };

    let bytes_size = metadata.len();

    let label = format!("{} {} ({})", char, file_name, symbol);

    let stdout = stdout();
    let mut handle = BufWriter::new(stdout.lock());

    writeln!(
        handle,
        "{:width$}{:width2$} {:>8}",
        "",
        label,
        if config.display_sizes {
            human_size(bytes_size)
        } else {
            String::new()
        },
        width = depth,
        width2 = 40 - depth,
    )?;

    if let Some(max_d) = config.max_depth {
        if depth >= max_d {
            return Ok(());
        }
    }

    if metadata.is_dir() {
        depth += 2;
        let entries = read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            walk_tree(&path, depth, config, writer)?;
        }
    }

    handle.flush()?;

    Ok(())
}
