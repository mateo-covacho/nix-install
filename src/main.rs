use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::{process::Command, str};

fn add_package<P: AsRef<Path>>(file_path: P, search_text: &str, new_line: &str) -> io::Result<()> {
    // Open the file for reading
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    // Collect the lines and search for the one containing `search_text`
    let mut new_lines: Vec<String> = Vec::new();
    let mut found = false;

    for line in reader.lines() {
        let line = line?;
        if line.contains(search_text) && !found {
            // Insert the new line above the found line
            new_lines.push(new_line.to_string());
            found = true;
        }
        new_lines.push(line);
    }

    // Open the file for writing and overwrite the content
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    for line in new_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = "/etc/nixos/configuration.nix";
    let search_text = "add packages above";

    let package = "\t\t".to_string() + &args[1];

    add_package(file_path, search_text, package.as_str())?;

    match Command::new("sh")
        .arg("-c")
        .arg("nixos-rebuild switch")
        .output()
    {
        Ok(output) => {
            println!("status: {}", output.status);
            println!("stdout: {}", str::from_utf8(&output.stdout).unwrap());
            println!("stderr: {}", str::from_utf8(&output.stderr).unwrap());
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }

    Ok(())
}
