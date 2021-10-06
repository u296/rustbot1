mod prelude {
    pub use super::super::prelude::*;
}

use serenity::framework::standard::macros::group;

#[group]
#[commands(join, leave, stop, enqueue, enqueue_local, skip, command_loop)]
struct Voice;

mod join;
use join::*;

mod play;
use play::*;
