use console::Emoji;

/// prefix emoji
pub const PREFIX_EMOJIS: [Emoji<'_, '_>; 4] = [
    Emoji("🛸", ""),
    Emoji("🚀", ""),
    Emoji("🛴", ""),
    Emoji("🛹", ""),
];

/// 最大同时下载数
pub const MAX_PARALLEL_DOWNLOAD: usize = 50;

pub const TS_LIST_PATH: &str = "ts_list.txt";
