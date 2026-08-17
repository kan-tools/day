use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessRequest {
    pub program: OsString,
    pub args: Vec<OsString>,
    pub cwd: PathBuf,
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
        }
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
}

pub trait Process {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String>;
}

pub struct SystemProcess;

impl Process for SystemProcess {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String> {
        let status = Command::new(&request.program)
            .args(&request.args)
            .current_dir(&request.cwd)
            .status()
            .map_err(|error| format!("could not run `{}`: {error}", request.display()))?;
        Ok(ProcessOutput {
            status: status.code().unwrap_or(2),
        })
    }
}
