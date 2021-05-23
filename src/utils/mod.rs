mod prelude {
    pub use serenity::prelude::*;
    pub use songbird::tracks::TrackHandle;
    pub use tracing::*;
}

mod guild;
mod textchannel;
mod voice;
mod local;

pub use guild::*;
pub use textchannel::*;
pub use voice::*;
pub use local::*;
