/// game page data
pub struct GameData {
    pub is_starting: u8, // 启动游戏的进度
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            is_starting: 0,
        }
    }
}