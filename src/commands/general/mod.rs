mod prelude {
    pub use super::super::prelude::*;

    pub use async_process::Stdio;
    pub use std::time::{Duration, Instant};
}

use serenity::framework::standard::macros::group;

#[group]
#[commands(exec, spam, upload, list, start_timer, read_timer, stop_timer)]
struct General;

mod exec;
use exec::*;

mod spam;
use spam::*;

mod upload;
use upload::*;

mod timer;
use timer::*;
