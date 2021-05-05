mod prelude {
    pub use super::super::prelude::*;
}

use serenity::framework::standard::macros::group;

#[group]
#[commands(show_latest_log, list_server_members)]
struct Debug;

mod log;
use self::log::*;

mod members;
use members::*;
