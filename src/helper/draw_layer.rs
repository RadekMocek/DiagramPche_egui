// "DL" == Draw List / Draw Layer / Draw channeL :)

pub const N_DL_USER_CHANNELS: i64 = 10;
//pub const N_DL_REAL_CHANNELS: i64 = 2 * N_DL_USER_CHANNELS;
pub const DL_USER_CHANNEL_DEFAULT_NODE: i64 = 4;
pub const DL_USER_CHANNEL_DEFAULT_PATH: i64 = 5;

pub enum DLPriority {
    Node,
    Path,
    PathLabel,
}

pub fn dl_user_channel_to_real_channel(input: i64, priority: DLPriority) -> i64 {
    let result = match priority {
        DLPriority::Node => 3 * input,
        DLPriority::Path => 3 * input + 1,
        DLPriority::PathLabel => 3 * input + 2,
    };

    // Minus because draw commands are stored in a max heap
    -result
}
