use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::process::Stdio;
use std::{process::Command, str};

fn add_package(file_path: &str, search_text: &str, new_line: &str) -> io::Result<()> {
    // Open the file for reading
    let file = File::open(file_path)?;
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
    let file_path = "/etc/nixos/configuration.nix";
    let search_text = "add packages above";
    let selected_package = match Command::new("sh")
        .arg("-c")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .arg(r#"
        src_cmd="reload:nix-search {q}"
        default_prompt="Search nix.pkgs > "
        fzf --bind "change:$src_cmd" \
                --bind "start:$src_cmd" \
                --disabled \
                --prompt "$default_prompt" \
                --bind "ctrl-t:transform:[[ \$FZF_PROMPT != \"$default_prompt\" ]] &&
                echo 'rebind(change)+change-prompt($default_prompt)+disable-search' ||
                echo 'unbind(change)+change-prompt(Filtering with fzf > )+enable-search'" \
                | awk '{print $1}' | tee /dev/stderr"#)
        .spawn().unwrap().wait_with_output()
    {
        Ok(output) => {
            str::from_utf8(&output.stdout).unwrap().trim().to_string()
        }
        Err(e) => {
            println!("error: {}", e);
            panic!("Un error");
        }
    };

    add_package(file_path, search_text, &selected_package)?;

    Command::new("sh")
        .arg("-c")
        .arg("sudo nixos-rebuild switch")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn().unwrap().wait().unwrap();

    Ok(())
}
