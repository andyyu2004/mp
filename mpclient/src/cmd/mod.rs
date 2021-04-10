mod filter;

pub(crate) use filter::{Filter, TrackFilter};

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum CmdMode {
    Filter,
    Cmd,
    None,
}

impl Default for CmdMode {
    fn default() -> Self {
        Self::None
    }
}
