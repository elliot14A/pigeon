use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;
