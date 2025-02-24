use console::Emoji;

/// prefix emoji
pub(crate) static PREFIX_EMOJIS: [Emoji<'_, '_>; 4] = [
    Emoji("🛸", ""),
    Emoji("🚀", ""),
    Emoji("🛴", ""),
    Emoji("🛹", ""),
];

/// 最大同时下载数
pub(crate) static MAX_PARALLEL_DOWNLOAD: usize = 50;

pub(crate) static TS_LIST_PATH: &str = "ts_list.txt";
