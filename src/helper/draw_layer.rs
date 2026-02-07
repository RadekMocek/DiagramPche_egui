// "DL" == Draw List / Draw Layer / Draw channeL :)

pub const N_DL_USER_CHANNELS: i64 = 10;
//pub const N_DL_REAL_CHANNELS: i64 = 2 * N_DL_USER_CHANNELS;
pub const DL_USER_CHANNEL_DEFAULT_NODE: i64 = 4;
pub const DL_USER_CHANNEL_DEFAULT_PATH: i64 = 5;

pub fn dl_user_channel_to_real_channel(input: i64, is_node: bool) -> i64 {
    let result = if is_node { 2 * input } else { 2 * input + 1 };
    // Minus because draw commands are stored in a max heap
    -result
}
