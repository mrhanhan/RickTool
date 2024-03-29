mod _impl;
mod model;
mod win;
mod envs;

pub type Result<T> = std::result::Result<T, String>;
pub use _impl::conpty::pipe::Pipe;
pub use _impl::conpty::conpty::{Terminal, StartupInfo, Size};

