use core::fmt;
use std::process::Command;

fn main() {
    run_command();
}


fn run_command() {
    let output = Command::new("sudo")
        .arg("docker")
        .arg("ps")
        .output()
        .expect("failed to execute hello");

    let string = String::from_utf8(output.stdout).expect("failed to stringify output");

    let split_string = string.lines().skip(1);

    for line in split_string {
        println!("{line}");
        handle_line(line);
    }
}

fn handle_line(line: &str) {
    let components: Vec<&str> = line.split("   ").collect();

    let first = components.first().expect("could not find first in line");
    let last = components.last().expect("could not find last in line");
    
    let container: DockerContainer = DockerContainer::new(last, first);

    println!("{container}");
}

#[derive(Debug)]
struct DockerContainer {
    name: String,
    id: String
}

impl fmt::Display for DockerContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Docker with name: {}, id: {}", self.name, self.id)?;
        Ok(())
    }
}

impl DockerContainer {
    fn new(name: &str, id: &str) -> DockerContainer {
        return DockerContainer {
            name: String::from(name),
            id: String::from(id),
        };
    }
}
