use std::process::{Command, Stdio};
use std::io::Read;

pub fn exe_nmap(target: &str, options: &str) -> Result<String, String> {
    let mut args: Vec<&str> = options.split_whitespace().collect();
    args.push(target);

    match Command::new("nmap")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            let mut output = String::new();
            if let Some(mut stdout) = child.stdout.take() {
                stdout.read_to_string(&mut output).unwrap_or_default();
            }

            let status = child.wait().unwrap();
            if status.success() {
                Ok(output)
            } else {
                let mut err_output = String::new();
                if let Some(mut stderr) = child.stderr.take() {
                    stderr.read_to_string(&mut err_output).unwrap_or_default();
                }
                Err(err_output)
            }
        }
        Err(e) => Err(format!("Failed to execute nmap: {}", e)),
    }
}

