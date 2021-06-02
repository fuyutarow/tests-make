use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use colored::*;
use indexmap::IndexMap as Map;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Test {
    pub script: String,
    pub tobe: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestResult {
    pub code: i32,
    pub output: String,
    pub error: String,
    pub tobe: String,
    pub pass: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manager {
    pub tests: Map<String, Test>,
    pub env: Option<HashMap<String, String>>,
    pub includes: Option<Vec<PathBuf>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workspace {
    pub tests: Map<String, Test>,
    pub env: HashMap<String, String>,
    pub success_tests: Map<String, TestResult>,
    pub fail_tests: Map<String, TestResult>,
}

impl From<Manager> for Workspace {
    fn from(manager: Manager) -> Self {
        Self {
            tests: manager.tests,
            env: manager.env.unwrap_or_default(),
            success_tests: Map::default(),
            fail_tests: Map::default(),
        }
    }
}

impl Workspace {
    pub fn run(&mut self) -> anyhow::Result<()> {
        let env_vars = self.env.to_owned();
        let options = run_script::ScriptOptions {
            runner: None,
            working_directory: None,
            input_redirection: run_script::IoOptions::Inherit,
            output_redirection: run_script::IoOptions::Pipe,
            exit_on_error: false,
            print_commands: false,
            env_vars: Some(env_vars),
        };

        let mut success_tests = Map::<String, TestResult>::new();
        let mut fail_tests = Map::<String, TestResult>::new();
        self.tests.iter().for_each(|(test_name, test)| {
            let (code, output, error) = run_script::run(&test.script, &vec![], &options).unwrap();

            let res = TestResult {
                pass: test.tobe == output,
                code,
                output,
                error,
                tobe: test.tobe.to_owned(),
            };

            if res.pass {
                success_tests.insert(test_name.to_owned(), res);
            } else {
                fail_tests.insert(test_name.to_owned(), res);
            }
        });

        self.success_tests = success_tests;
        self.fail_tests = fail_tests;
        Ok(())
    }
}

impl Manager {
    pub fn from_fpath(fpath: PathBuf) -> anyhow::Result<Self> {
        let mut f = File::open(fpath).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let manager = toml::from_str::<Self>(&contents)?;
        Ok(manager)
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let mut workspace = Workspace::from(self.to_owned());

        if let Some(includes) = self.includes.to_owned() {
            let workspaces = includes
                .iter()
                .filter_map(|(fpath)| match Self::from_fpath(fpath.to_owned()) {
                    Ok(manager) => {
                        let workspace_name = fpath.file_stem().unwrap().to_str().unwrap();
                        Some((workspace_name.to_owned(), Workspace::from(manager)))
                    }
                    Err(_) => None,
                })
                .collect::<HashMap<String, Workspace>>();
        }

        workspace.run()?;

        {
            for (test_name, test_result) in workspace.success_tests {
                println!("{} ... {}", &test_name, "ok".green());
            }

            println!();

            for (test_name, test_result) in workspace.fail_tests {
                print!("{} ... ", test_name);

                print_diff(&test_result.output, &test_result.tobe);
            }
        }
        Ok(())
    }
}

fn print_diff(left: &str, right: &str) {
    let formating = |s: &str, label: &str| {
        format!(
            "{label}\t{lines}",
            label = label,
            lines = s
                .lines()
                .map(String::from)
                .collect::<Vec<String>>()
                .join(&format!("\n{label}\t", label = label))
                .replace("\n", "\\n\n")
        )
    };

    if left != right {
        let left = formating(left, "-");
        let right = formating(right, "+");
        println!(
            r#"({actual}|{expected}):
{left}
{right}
"#,
            actual = "-actual".red(),
            expected = "+expected".green(),
            left = left.red(),
            right = right.green()
        );
    }
}
