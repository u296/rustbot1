mod prelude {
    pub use super::super::prelude::*;
}

use serenity::framework::standard::macros::group;

#[group]
#[commands(join, leave, play, play_local, stop, enqueue)]
struct Voice;

mod join;
use join::*;

mod play;
use play::*;
