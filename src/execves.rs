use std::collections::HashMap;
use std::ffi::CString;
use std::io::{BufRead, Read};

use serde::{Deserialize, Serialize};

use crate::execves;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execves {
    pub command: String,
    pub arguments: Vec<String>,
    pub environment: HashMap<String, String>,
    #[serde(rename = "environment_files")]
    pub environment_files: Option<Vec<String>>,
    pub executables: Option<Vec<String>>,
}

impl Execves {
    pub fn new(command: String) -> Self {
        return Self {
            command,
            arguments: Vec::new(),
            environment: HashMap::new(),
            environment_files: Some(Vec::new()),
            executables: Some(Vec::new()),
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
        execves(
            self.create_command(),
            &self.create_arguments(),
            &self.create_environment(),
            &self.create_secrets(),
        )
        .unwrap()
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
        let env: Vec<String> = self.environment.iter().map(|(k, v)| format_env(&k, &v)).collect();

        create_cstring_vec(&env)
    }

    fn create_secrets(&self) -> Vec<CString> {
        let mut resp = Vec::new();

        if let Some(files) = &self.environment_files {
            for name in files {
                let fp = std::fs::File::open(name).unwrap();
                let bfp = std::io::BufReader::new(fp);

                for line in bfp.lines() {
                    if let Ok(line) = line {
                        resp.push(CString::new(line).unwrap())
                    }
                }
            }
        }

        resp
    }
}

fn create_cstring_vec(input: &[String]) -> Vec<CString> {
    input.into_iter().map(|s| CString::new(s.clone()).unwrap()).collect()
}

fn format_env(key: &str, value: &str) -> String {
    format!("{key}={value}")
}
