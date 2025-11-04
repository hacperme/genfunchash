use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Configuration {
    /// 默认设置
    #[serde(default)]
    pub default: DefaultConfig,

    /// 缓存设置
    #[serde(default)]
    pub cache: CacheConfig,

    /// 输出设置
    #[serde(default)]
    pub output: OutputConfig,

    /// 批处理设置
    #[serde(default)]
    pub batch: BatchConfig,
}

/// 默认设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    /// 默认输出格式
    pub output_format: String,

    /// 是否启用缓存
    pub enable_cache: bool,

    /// 并发处理数
    pub concurrency: usize,
}

/// 缓存设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 缓存最大条目数
    pub max_entries: usize,

    /// 缓存生存时间（秒）
    pub ttl_seconds: u64,
}

/// 输出设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// 是否包含时间戳
    pub include_timestamp: bool,

    /// 是否包含数组索引
    pub include_index: bool,

    /// 字段分隔符
    pub separator: String,
}

/// 批处理设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    /// 批处理块大小
    pub chunk_size: usize,

    /// 是否启用并行处理
    pub parallel_processing: bool,
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            output_format: "c-array".to_string(),
            enable_cache: true,
            concurrency: 8,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            ttl_seconds: 3600,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            include_timestamp: true,
            include_index: true,
            separator: "\t".to_string(),
        }
    }
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1000,
            parallel_processing: true,
        }
    }
}

impl Configuration {
    /// 从文件加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("无法读取配置文件: {:?}", path.as_ref()))?;

        let config: Configuration = toml::from_str(&content).with_context(|| "解析配置文件失败")?;

        Ok(config)
    }

    /// 保存配置到文件
    #[allow(dead_code)]
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self).with_context(|| "序列化配置失败")?;

        fs::write(&path, content)
            .with_context(|| format!("无法写入配置文件: {:?}", path.as_ref()))?;

        Ok(())
    }

    /// 创建示例配置文件
    #[allow(dead_code)]
    pub fn create_example<P: AsRef<Path>>(path: P) -> Result<()> {
        let config = Configuration::default();
        config.save_to_file(path)
    }
}
