use std::ffi::CString;
use std::io::{BufRead, Read};
use std::{collections::HashMap, io::BufReader};

use nix::unistd::execve;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execves {
    pub command: String,

    #[serde(default)]
    pub arguments: Vec<String>,

    #[serde(default)]
    pub environment: HashMap<String, String>,

    #[serde(default, rename = "environment_files")]
    pub environment_files: Vec<String>,
}

impl Execves {
    pub fn new(command: String) -> Self {
        return Self {
            command,
            arguments: Vec::new(),
            environment: HashMap::new(),
            environment_files: Vec::new(),
        };
    }

    pub fn arguments(mut self, arguments: &[String]) -> Self {
        for arg in arguments {
            self.arguments.push(arg.to_owned())
        }
        self
    }

    pub fn environment(mut self, environment: HashMap<&str, &str>) -> Self {
        for (k, v) in environment {
            self.environment.insert(k.to_owned(), v.to_owned());
        }
        self
    }

    pub fn from_reader<R>(reader: R) -> Self
    where
        R: Read,
    {
        serde_yaml::from_reader(reader).expect("unable to parase from reader")
    }

    pub fn call(self) {
        execve(
            &self.create_command(),
            &self.create_arguments(),
            &self.create_environment(),
        )
        .expect("unable to start new command");
    }

    fn create_command(&self) -> CString {
        CString::new(self.command.clone()).unwrap()
    }

    fn create_arguments(&self) -> Vec<CString> {
        let mut args = Vec::new();
        args.push(self.create_command());
        args.extend(create_cstring_vec(&self.arguments));
        args
    }

    fn create_environment(&self) -> Vec<CString> {
        let mut env: Vec<String> = self.environment.iter().map(format_env).collect();

        for file in &self.environment_files {
            let fp = std::fs::File::open(file).unwrap();
            let bfp = BufReader::new(fp);
            for line in bfp.lines() {
                env.push(line.unwrap())
            }
        }

        create_cstring_vec(&env)
    }
}

fn create_cstring_vec(input: &[String]) -> Vec<CString> {
    input.into_iter().map(|s| CString::new(s.clone()).unwrap()).collect()
}

fn format_env(entry: (&String, &String)) -> String {
    format!("{}={}", entry.0, entry.1)
}
