mod prelude {
    pub use super::super::prelude::*;

    pub use async_process::Stdio;
    pub use std::time::{Duration, Instant};
}

use serenity::framework::standard::macros::group;

#[group]
#[commands(
    spam,
    upload,
    list,
    start_timer,
    read_timer,
    stop_timer,
    select_random,
    split,
    add_reaction,
    remove_reaction
)]
struct General;

mod spam;
use spam::*;

mod upload;
use upload::*;

mod timer;
use timer::*;

mod random;
use random::*;

mod reactions;
use reactions::*;
