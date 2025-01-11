use core::fmt;
use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json::Result;

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

    if let Some(container) = DockerContainer::new_from_line(&components)
    {
        println!("{container}");
    }

}

// CONTAINER ID, IMAGE, COMMAND, CREATED, STATUS, PORTS, NAMES

#[derive(Debug, Serialize)]
struct DockerContainer {
    id: String,
    image: String,
    command: String,
    created: String,
    status: String,
    port: String,
    name: String
}

impl fmt::Display for DockerContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.to_string().expect("could not parse to json"))?;
        Ok(())
    }
}

impl DockerContainer {
    fn new(id: &str, image: &str, command: &str, created: &str, status: &str, port: &str, name: &str) -> DockerContainer {
        return DockerContainer {
            id: String::from(id), 
            image: String::from(image),
            command: String::from(command), 
            created: String::from(created),
            status: String::from(status), 
            port: String::from(port),
            name: String::from(name), 
        };
    }

    fn new_from_line(info: &Vec<&str>) -> Option<DockerContainer> {
        let (id, rest) = info.split_first().expect("could not split");  
        let (image, rest) = rest.split_first().expect("could not split");  
        let (command, rest) = rest.split_first().expect("could not split");  
        let (created, rest) = rest.split_first().expect("could not split");  
        let (status, rest) = rest.split_first().expect("could not split");  
        let (port, rest) = rest.split_first().expect("could not split");  
        let name = rest.first().expect("could not take first");
        
        Some(DockerContainer::new(id, image, command, created, status, port, name))
    }

    fn to_string(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}
