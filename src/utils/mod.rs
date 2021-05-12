mod prelude {
    pub use serenity::prelude::*;
    pub use tracing::*;
}

mod textchannel;
mod voice;
mod guild;

pub use textchannel::*;
pub use voice::*;
pub use guild::*;