use error::{AppResult, Error as AppError};
use std::process::{Command, Output};
use uuid::Uuid;

mod error;

static RUNNER: &str = "code-runner";
static PROG: &str = "docker";

fn main() {
    // provition env
    match provitioning() {
        Ok(_) => {
            let code_snippet = r##"
# Python script to print even numbers from 1 to 10
for number in range(1, 11):
    if number % 2 == 0:
        print(number)
"##;

            let my_uuid = Uuid::new_v4().to_string();
            // run the container
            match Command::new(PROG)
                .arg("run")
                .arg("--name")
                .arg(&my_uuid)
                .arg(RUNNER)
                .arg(code_snippet)
                .output()
            {
                Ok(output) => print_output(&output),
                Err(e) => eprintln!("Failed to execute command: {}", e),
            }

            // clean up the container
            match Command::new(PROG).arg("rm").arg(my_uuid).output() {
                Ok(output) => print_output(&output),
                Err(e) => eprintln!("Failed to execute command: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to provition code-runner env: {e}"),
    }
}

fn provitioning() -> AppResult<()> {
    // check it docker is installed
    match Command::new("which").arg(&PROG).output() {
        Ok(output) => {
            if !output.status.success() {
                return Err(AppError::Exec(
                    "Docker not installed on host marchine".to_string(),
                ));
            }
            // build the image
            // docker build -t python-runner .
            match Command::new(PROG)
                .arg("build")
                .arg("-t")
                .arg(RUNNER)
                .arg(".")
                .output()
            {
                Ok(output) => {
                    if !output.status.success() {
                        print_output(&output)
                    }
                    println!("ENV provisioned");
                    Ok(())
                }
                Err(e) => Err(AppError::System(e.to_string())),
            }
        }
        Err(e) => Err(AppError::System(e.to_string())),
    }
}

fn print_output(output: &Output) {
    // Print the output of the command
    println!("Status: {}", output.status);
    // Convert the output to a String and print it
    println!("Stdout: \n{}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
}
