use std::{fs, io, path::Path};

#[derive(Debug, Default)]
pub struct State {
    pub profile: String,
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Read error:\n  {0}")]
    Read(#[from] io::Error),
    #[error("Parse error")]
    Parse,
}
impl State {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, StateError> {
        fn from_file(path: &Path) -> Result<State, StateError> {
            let bytes = fs::read(path)?;
            State::from_slice(&bytes)
        }
        from_file(path.as_ref())
    }
    pub fn from_slice(bytes: &[u8]) -> Result<Self, StateError> {
        let str = std::str::from_utf8(bytes).map_err(|_| StateError::Parse)?;
        Ok(State { profile: str.to_string() })
    }
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), StateError> {
        fs::create_dir_all(path.as_ref().parent().expect("state file path does not have parent?"))?;
        Ok(fs::write(path, &self.profile)?)
    }
}
