use std::{fs, path::PathBuf};

pub fn read_input(input: PathBuf) -> anyhow::Result<String> {
    let contents = match fs::read_to_string(&input) {
        Ok(c) => c,
        Err(e) => anyhow::bail!("Failed to read input: {:?}. Error: {:?}", &input, e),
    };

    Ok(contents)
}
