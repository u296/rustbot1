mod prelude {
    pub use serenity::prelude::*;
    pub use songbird::tracks::TrackHandle;
    pub use tracing::*;
}

mod guild;
mod local;
mod textchannel;
mod voice;

pub use guild::*;
pub use local::*;
pub use textchannel::*;
pub use voice::*;
