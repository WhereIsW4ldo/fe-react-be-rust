use std::fmt;
use std::process::Command;
use rocket_cors::{AllowedMethods};
use serde::Serialize;

#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error,
    Cors, CorsOptions
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![run_command])
        .attach(make_cors())
}

#[get("/sudo/docker/ps")]
fn run_command() -> String {
    let output = Command::new("sudo")
        .arg("docker")
        .arg("ps")
        .output()
        .expect("failed to execute hello");

    let string = String::from_utf8(output.stdout).expect("failed to stringify output");

    let split_string = string.lines().skip(1);

    let lines = split_string
        .map(|line| handle_line(line))
        .flatten()
        .collect::<Vec<_>>();

    if let Ok(data) = serde_json::to_string(&lines)
    {
        return data;
    }

    "".to_string()
}

fn handle_line(line: &str) -> Option<DockerContainer>{
    let components: Vec<&str> = line.split("   ").collect();
    DockerContainer::new_from_line(&components)
}

#[derive(Debug, Serialize)]
struct DockerContainer {
    id: String,
    image: String,
    command: String,
    created: String,
    status: String,
    ports: Vec<String>,
    name: String
}

impl fmt::Display for DockerContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.to_string().expect("could not parse to json"))?;
        Ok(())
    }
}

impl DockerContainer {
    fn new(id: &str, image: &str, command: &str, created: &str, status: &str, ports: &str, name: &str) -> DockerContainer {
        DockerContainer {
            id: String::from(id),
            image: String::from(image),
            command: String::from(command),
            created: String::from(created),
            status: String::from(status),
            ports: ports.split(", ").map(String::from).collect(),
            name: String::from(name),
        }
    }

    pub fn new_from_line(info: &Vec<&str>) -> Option<DockerContainer> {
        let (id, rest) = info.split_first().expect("could not split");
        let (image, rest) = rest.split_first().expect("could not split");
        let (command, rest) = rest.split_first().expect("could not split");
        let (created, rest) = rest.split_first().expect("could not split");
        let (status, rest) = rest.split_first().expect("could not split");
        let (ports, rest) = rest.split_first().expect("could not split");
        let name = rest.first().expect("could not take first");

        Some(DockerContainer::new(id, image, command, created, status, ports, name))
    }

    fn to_string(&self) -> Option<String> {
        serde_json::to_string(&self).ok()
    }
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        ..Default::default()
    }.to_cors().unwrap()
}