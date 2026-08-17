use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessRequest {
    pub program: OsString,
    pub args: Vec<OsString>,
    pub cwd: PathBuf,
    pub env: BTreeMap<OsString, OsString>,
    pub stdin: Option<Vec<u8>>,
}

impl ProcessRequest {
    pub fn new<I, S>(program: impl Into<OsString>, args: I, cwd: &Path) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        Self {
            program: program.into(),
            args: args.into_iter().map(Into::into).collect(),
            cwd: cwd.to_path_buf(),
            env: BTreeMap::new(),
            stdin: None,
        }
    }

    pub fn with_env(mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    pub fn with_stdin(mut self, input: impl Into<Vec<u8>>) -> Self {
        self.stdin = Some(input.into());
        self
    }

    pub fn display(&self) -> String {
        std::iter::once(self.program.as_os_str())
            .chain(self.args.iter().map(OsString::as_os_str))
            .map(OsStr::to_string_lossy)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

pub trait Process {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String>;
}

pub struct SystemProcess;

impl Process for SystemProcess {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String> {
        let mut command = Command::new(&request.program);
        command
            .args(&request.args)
            .current_dir(&request.cwd)
            .envs(&request.env);
        let output = if let Some(input) = &request.stdin {
            let mut child = command
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|error| format!("could not run `{}`: {error}", request.display()))?;
            child
                .stdin
                .take()
                .expect("piped stdin is present")
                .write_all(input)
                .map_err(|error| format!("could not write to `{}`: {error}", request.display()))?;
            child
                .wait_with_output()
                .map_err(|error| format!("could not wait for `{}`: {error}", request.display()))?
        } else {
            command
                .output()
                .map_err(|error| format!("could not run `{}`: {error}", request.display()))?
        };
        Ok(ProcessOutput {
            status: output.status.code().unwrap_or(2),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}
