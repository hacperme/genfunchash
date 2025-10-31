# GenFuncHash API 规范

## 概述

本文档定义了 GenFuncHash 项目的内部 API 接口规范，包括核心模块、函数签名和数据结构。

## 核心 Traits

### HashAlgorithm Trait

```rust
pub trait HashAlgorithm {
    type Output: AsRef<[u8]>;
    type Error: std::error::Error + Send + Sync + 'static;

    /// 算法名称
    fn name(&self) -> &'static str;
    
    /// 计算哈希值
    fn compute(&self, input: &str) -> Result<Self::Output, Self::Error>;
    
    /// 重置算法状态（如果支持流式处理）
    fn reset(&mut self);
}
```

### OutputFormatter Trait

```rust
pub trait OutputFormatter {
    /// 格式化哈希输出
    fn format(&self, hash: &[u8]) -> String;
    
    /// 格式名称
    fn name(&self) -> &'static str;
}
```

## 核心数据结构

### FunctionName

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionName {
    /// 原始函数名
    pub raw: String,
    /// 标准化后的函数名
    pub normalized: String,
    /// 函数名所属的编程语言
    pub language: Option<Language>,
}

impl FunctionName {
    pub fn new(name: &str) -> Result<Self, ParseError>;
    pub fn normalize(&mut self) -> Result<(), ParseError>;
    pub fn validate(&self) -> Result<(), ValidationError>;
}
```

### HashResult

```rust
#[derive(Debug, Clone)]
pub struct HashResult {
    /// 输入的函数名
    pub function_name: FunctionName,
    /// 使用的哈希算法
    pub algorithm: String,
    /// 哈希值（原始字节）
    pub hash: Vec<u8>,
    /// 格式化后的哈希值
    pub formatted: String,
    /// 计算时间戳
    pub timestamp: std::time::SystemTime,
}
```

### Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// 默认哈希算法
    pub default_algorithm: String,
    /// 默认输出格式
    pub default_output_format: String,
    /// 启用的算法列表
    pub enabled_algorithms: Vec<String>,
    /// 批处理设置
    pub batch_settings: BatchSettings,
    /// 缓存设置
    pub cache_settings: CacheSettings,
}
```

## 模块接口

### Parser 模块 (`src/parser.rs`)

```rust
/// 解析函数名字符串
pub fn parse_function_name(input: &str) -> Result<FunctionName, ParseError>;

/// 从文件解析多个函数名
pub fn parse_from_file(path: &Path) -> Result<Vec<FunctionName>, ParseError>;

/// 验证函数名格式
pub fn validate_function_name(name: &FunctionName) -> Result<(), ValidationError>;
```

### Generator 模块 (`src/generator.rs`)

```rust
/// 哈希生成器
pub struct HashGenerator {
    custom_hasher: CustomHasher,
    formatters: HashMap<String, Box<dyn OutputFormatter>>,
    cache: Option<Arc<Mutex<HashMap<String, HashResult>>>>,
}

impl HashGenerator {
    pub fn new() -> Self;
    
    /// 注册输出格式化器
    pub fn register_formatter<F: OutputFormatter + 'static>(&mut self, formatter: F);
    
    /// 生成单个函数名的哈希
    pub fn generate_hash(
        &self,
        function_name: &FunctionName,
        format: &str,
    ) -> Result<HashResult, GeneratorError>;
    
    /// 批量生成哈希
    pub fn generate_batch(
        &self,
        function_names: &[FunctionName],
        format: &str,
    ) -> Result<Vec<HashResult>, GeneratorError>;
    
    /// 异步批量生成
    pub async fn generate_batch_async(
        &self,
        function_names: &[FunctionName],
        format: &str,
    ) -> Result<Vec<HashResult>, GeneratorError>;
}
```

### CLI 模块 (`src/cli.rs`)

```rust
/// 命令行参数结构
#[derive(Parser)]
#[command(name = "genfunchash")]
#[command(about = "Generate hashes for function names")]
pub struct CliArgs {
    /// 函数名
    #[arg(short, long)]
    pub function: Option<String>,
    
    /// 输入文件路径
    #[arg(short = 'i', long)]
    pub input_file: Option<PathBuf>,
    
    /// 输出格式
    #[arg(short, long, default_value = "hex")]
    pub output_format: String,
    
    /// 输出文件路径
    #[arg(short, long)]
    pub output_file: Option<PathBuf>,
    
    /// 配置文件路径
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    
    /// 启用缓存
    #[arg(long)]
    pub enable_cache: bool,
    
    /// 并发数
    #[arg(long, default_value = "4")]
    pub concurrency: usize,
}

/// 执行命令行操作
pub async fn execute(args: CliArgs) -> Result<(), CliError>;
```

## 错误类型定义

```rust
/// 解析错误
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid function name format: {0}")]
    InvalidFormat(String),
    
    #[error("Empty function name")]
    EmptyName,
    
    #[error("File read error: {0}")]
    FileError(#[from] std::io::Error),
}

/// 生成器错误
#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("Unknown algorithm: {0}")]
    UnknownAlgorithm(String),
    
    #[error("Unknown output format: {0}")]
    UnknownFormat(String),
    
    #[error("Hash computation failed: {0}")]
    ComputationError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
}

/// CLI 错误
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),
    
    #[error("Generator error: {0}")]
    GeneratorError(#[from] GeneratorError),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

## 自定义哈希算法实现

### CustomHasher (`src/hash/custom.rs`)

```rust
/// 自定义函数名哈希算法
pub struct CustomHasher;

impl CustomHasher {
    pub fn new() -> Self {
        CustomHasher
    }
    
    /// 计算函数名哈希值
    /// 算法：h = 5527 * h + 7 * char; v = h & 0xFFFF; h ^= v * v
    pub fn compute_hash(&self, func_name: &str) -> u32 {
        let mut h: u32 = 0;
        
        for byte in func_name.bytes() {
            h = h.wrapping_mul(5527).wrapping_add(7 * (byte as u32));
            let v = h & 0x0000_FFFF;
            h ^= v.wrapping_mul(v);
        }
        
        h
    }
    
    /// 批量计算哈希值
    pub fn compute_batch(&self, func_names: &[&str]) -> Vec<u32> {
        func_names.iter().map(|name| self.compute_hash(name)).collect()
    }
}

impl Default for CustomHasher {
    fn default() -> Self {
        Self::new()
    }
}
```

## 常用工具函数

### 字符串处理

```rust
/// 标准化函数名（移除空格、转换大小写等）
pub fn normalize_function_name(name: &str) -> String;

/// 检测编程语言
pub fn detect_language(name: &str) -> Option<Language>;

/// 清理特殊字符
pub fn sanitize_function_name(name: &str) -> String;
```

### 输出格式化

```rust
/// C 语言数组格式化器
pub struct CArrayFormatter {
    index_counter: Arc<AtomicUsize>,
}

impl CArrayFormatter {
    pub fn new() -> Self {
        CArrayFormatter {
            index_counter: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    /// 格式化为 C 数组条目
    /// 输出格式: /*[索引]*/{0x哈希值,                             (void *)函数名},
    pub fn format_entry(&self, function_name: &str, hash: u32) -> String {
        let index = self.index_counter.fetch_add(1, Ordering::Relaxed);
        format!(
            "/*[{:3}]*/{{0x{:08X},                             (void *){}}},",
            index, hash, function_name
        )
    }
    
    /// 重置索引计数器
    pub fn reset_index(&self) {
        self.index_counter.store(0, Ordering::Relaxed);
    }
}

impl OutputFormatter for CArrayFormatter {
    fn format(&self, hash: &[u8]) -> String {
        // 假设 hash 是 4 字节的 u32
        let hash_value = u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]]);
        format!("0x{:08X}", hash_value)
    }
    
    fn name(&self) -> &'static str {
        "c-array"
    }
}
```

### 配置管理

```rust
/// 加载配置文件
pub fn load_config(path: &Path) -> Result<Configuration, ConfigError>;

/// 保存配置文件
pub fn save_config(config: &Configuration, path: &Path) -> Result<(), ConfigError>;

/// 获取默认配置
pub fn default_config() -> Configuration;
```

## 异步接口

```rust
/// 异步哈希生成器
pub struct AsyncHashGenerator {
    inner: Arc<HashGenerator>,
    executor: tokio::runtime::Handle,
}

impl AsyncHashGenerator {
    /// 异步生成哈希
    pub async fn generate_hash_async(
        &self,
        function_name: &FunctionName,
        algorithm: &str,
        format: &str,
    ) -> Result<HashResult, GeneratorError>;
    
    /// 流式处理
    pub fn generate_stream(
        &self,
        function_names: impl Stream<Item = FunctionName>,
        algorithm: &str,
        format: &str,
    ) -> impl Stream<Item = Result<HashResult, GeneratorError>>;
}
```

## 插件接口

```rust
/// 插件特征
pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn initialize(&mut self) -> Result<(), PluginError>;
    fn register_algorithms(&self, generator: &mut HashGenerator);
    fn register_formatters(&self, generator: &mut HashGenerator);
}

/// 插件管理器
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self;
    pub fn load_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginError>;
    pub fn initialize_all(&mut self) -> Result<(), PluginError>;
    pub fn register_all(&self, generator: &mut HashGenerator);
}
```

---

*API 版本：1.0*  
*最后更新：2025年10月31日*