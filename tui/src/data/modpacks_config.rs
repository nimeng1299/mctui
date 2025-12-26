/// Modpacks configuration data
pub struct ModPacksConfig {
    pub modpacks: Vec<ModPacksConfig>,
    pub global_setting: GlobalSetting,
}

impl Default for ModPacksConfig {
    fn default() -> Self {
        Self {
            modpacks: Vec::new(),
            global_setting: GlobalSetting {
                java_type: JavaType::Auto,
                memory_type: MemoryType::Auto,
                game_resolution: GameResolution::Auto,
            },
        }
    }
}

pub struct ModPackConfig {
    pub info: ModPackInfo,
    pub setting: ModpackSetting,
}

pub struct ModPackInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

pub struct ModpackSetting {
    pub java_type: Option<JavaType>,
    pub memory_type: Option<MemoryType>,
    pub game_resolution: Option<GameResolution>,
}

pub struct GlobalSetting {
    pub java_type: JavaType,
    pub memory_type: MemoryType,
    pub game_resolution: GameResolution,
}

/// 使用哪种Java运行环境
pub enum JavaType {
    Auto,
    Java { version: String, path: String },
    Custom { path: String },
}

/// 分配内存方式
pub enum MemoryType {
    Auto,
    Custom { min: usize },
}

pub enum GameResolution {
    Auto,
    Custom { width: usize, height: usize },
}
