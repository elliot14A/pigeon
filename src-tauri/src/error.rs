use serde::{Deserialize, Serialize};
use snafu::Snafu;

#[derive(Debug, Snafu, Serialize, Deserialize)]
#[snafu(visibility(pub))]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;
