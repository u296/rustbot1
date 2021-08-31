use serde::{Serialize, Deserialize};


// fisrt string is what to react to, second is reaction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response {
    AudioCue((String, String)),
    TextReply((String, String)),
}