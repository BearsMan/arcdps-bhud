mod combat;

use crate::pubsub::dispatch;
use smol::Task;

pub fn imgui(not_charsel_or_loading: bool) {
    Task::spawn(
        dispatch([1, not_charsel_or_loading as u8].to_vec())
    ).detach();
}

pub use combat::cbt as combat;
pub use combat::cbt_local as combat_local;
