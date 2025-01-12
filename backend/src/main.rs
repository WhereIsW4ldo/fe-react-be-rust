use std::{fmt, io};
use std::io::Write;
use std::process::{Command, Stdio};
use rocket_cors::{AllowedMethods};
use serde::{Deserialize, Serialize};

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
    let argument = "sudo docker ps --all --no-trunc --format=\"{{json . }}\" | jq --tab -s .";

    let sudo_docker = Command::new("sh")
        .arg("-c")
        .arg(argument)
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute hello");

    // let string = String::from_utf8(output.stdout).expect("failed to stringify output");

    println!("{}", sudo_docker.status);
    let string = String::from_utf8(sudo_docker.stdout).expect("failed to stringify");
    let dockers: Vec<DockerContainer> = serde_json::from_str(&string).unwrap();

    serde_json::to_string(&dockers).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
struct DockerContainer {
    ID: String,
    Image: String,
    Command: String,
    CreatedAt: String,
    Status: String,
    Ports: String,
    Names: String,
    State: String,
}

impl fmt::Display for DockerContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.to_string().expect("could not parse to json"))?;
        Ok(())
    }
}

impl DockerContainer {
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