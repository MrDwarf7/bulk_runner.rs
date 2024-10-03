use std::{
    fmt::{self, Debug, Display},
    process::{ExitStatus, Output},
};


#[derive(Debug, Default)]
pub struct BotOutput {
    inner_buf: Vec<u8>,
    stdout: String,
    stderr: String,
    status: ExitStatus,
}

impl BotOutput {
    /// Returns the inner buffer of the output.
    pub fn inner_buf(&self) -> &[u8] {
        &self.inner_buf
    }

    /// Returns the stdout of the output.
    #[inline]
    #[allow(dead_code)]
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Returns the stderr of the output.
    #[inline]
    #[allow(dead_code)]
    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    /// Adds a message to the output.
    #[allow(dead_code)]
    pub fn add_message<T>(&mut self, message: T)
    where
        T: Into<Box<[u8]>>,
    {
        self.inner_buf.extend_from_slice(&message.into());
        // self.stdout.push_str(message.as_ref());
    }

    pub fn print_buffer(&self) {
        let buffer = String::from_utf8_lossy(&self.inner_buf);

        println!("Buffer: {}", buffer);
    }
}

impl From<Output> for BotOutput {
    fn from(output: Output) -> Self {
        Self {
            inner_buf: output.stdout.clone(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            status: output.status,
        }
    }
}

impl From<crate::internals::Error> for BotOutput {
    fn from(error: crate::internals::Error) -> Self {
        Self {
            stderr: error.to_string(),
            ..Default::default()
        }
    }
}

impl From<ExitStatus> for BotOutput {
    fn from(status: ExitStatus) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }
}

impl Display for BotOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "inner_buffer: {:?}", self.inner_buf())?;
        write!(f, "stdout: {}", self.stdout)?;
        write!(f, "stderr: {}", self.stderr)?;
        write!(f, "status: {}", self.status)
    }
}
