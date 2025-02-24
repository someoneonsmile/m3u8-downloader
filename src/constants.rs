use console::Emoji;

/// prefix emoji
pub(crate) const PREFIX_EMOJIS: [Emoji<'_, '_>; 4] = [
    Emoji("🛸", ""),
    Emoji("🚀", ""),
    Emoji("🛴", ""),
    Emoji("🛹", ""),
];

/// 最大同时下载数
pub(crate) const MAX_PARALLEL_DOWNLOAD: usize = 50;

pub(crate) const TS_LIST_PATH: &str = "ts_list.txt";
