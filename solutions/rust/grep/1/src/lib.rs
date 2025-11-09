use anyhow::{Error, Context};
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Flags {
    pub print_line_number: bool,
    pub print_file_names: bool,
    pub case_insensitive: bool,
    pub invert_match: bool,
    pub match_entire_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            print_line_number: flags.contains(&"-n"),
            print_file_names: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            invert_match: flags.contains(&"-v"),
            match_entire_line: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();

    for &file in files {
        let content = read_to_string(file)
            .with_context(|| format!("Failed to read file: {}", file))?;
        let mut matched = false;

        for (i, line) in content.lines().enumerate() {
            let mut line_cmp = line.to_string();
            let mut pattern_cmp = pattern.to_string();

            if flags.case_insensitive {
                line_cmp = line_cmp.to_lowercase();
                pattern_cmp = pattern_cmp.to_lowercase();
            }

            let mut is_match = if flags.match_entire_line {
                line_cmp == pattern_cmp
            } else {
                line_cmp.contains(&pattern_cmp)
            };

            if flags.invert_match {
                is_match = !is_match;
            }

            if is_match {
                matched = true;
                if flags.print_file_names {
                    results.push(file.to_string());
                    break;
                }

                let mut output = String::new();

                if files.len() > 1 {
                    output.push_str(file);
                    output.push(':');
                }

                if flags.print_line_number {
                    output.push_str(&(i + 1).to_string());
                    output.push(':');
                }

                output.push_str(line);
                results.push(output);
            }
        }
    }

    Ok(results)
}
