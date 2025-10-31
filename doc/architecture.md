# GenFuncHash 架构文档

## 系统概述

GenFuncHash 是一个基于 Rust 的高性能命令行工具，专门用于生成函数名称的哈希值。本文档详细描述了系统的架构设计、模块组织和实现细节。

## 架构原则

### 1. 模块化设计
- 每个功能模块职责单一，高内聚低耦合
- 通过 trait 定义清晰的接口边界
- 支持插件化扩展

### 2. 性能优先
- 使用 Rust 的零成本抽象
- 支持多线程并行处理
- 内存高效的流式处理

### 3. 可扩展性
- 支持新的哈希算法插件
- 灵活的输出格式定制
- 可配置的处理管道

### 4. 错误处理
- 使用 Result 类型进行错误传播
- 详细的错误信息和恢复建议
- 优雅的失败处理

## 系统架构图

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
├─────────────────────────────────────────────────────────────┤
│  Command Parser  │  Config Loader  │  Output Formatter      │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      Service Layer                          │
├─────────────────────────────────────────────────────────────┤
│  Hash Generator  │  Batch Processor  │  Cache Manager       │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                       Core Layer                            │
├─────────────────────────────────────────────────────────────┤
│  Function Parser │  Algorithm Registry │  Format Registry   │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    Algorithm Layer                          │
├─────────────────────────────────────────────────────────────┤
│              Custom Function Name Hash                      │
│           h = 5527*h + 7*char; v = h&0xFFFF; h ^= v*v      │
└─────────────────────────────────────────────────────────────┘
```

## 模块详细设计

### CLI Layer (命令行层)

#### Command Parser (`src/cli/parser.rs`)
- 使用 `clap` 解析命令行参数
- 验证参数有效性
- 提供帮助信息和错误提示

```rust
pub struct CommandParser {
    app: clap::Command,
}

impl CommandParser {
    pub fn parse(&self) -> Result<CliArgs, ParseError>;
    pub fn validate_args(&self, args: &CliArgs) -> Result<(), ValidationError>;
}
```

#### Config Loader (`src/cli/config.rs`)
- 加载和解析配置文件
- 合并命令行参数和配置文件设置
- 提供默认配置

```rust
pub struct ConfigLoader {
    default_config: Configuration,
}

impl ConfigLoader {
    pub fn load_from_file(&self, path: &Path) -> Result<Configuration, ConfigError>;
    pub fn merge_with_args(&self, config: Configuration, args: &CliArgs) -> Configuration;
}
```

#### Output Formatter (`src/cli/output.rs`)
- 格式化输出结果
- 支持多种输出格式（文本、JSON、CSV）
- 处理重定向和文件输出

```rust
pub trait OutputFormatter {
    fn format_single(&self, result: &HashResult) -> String;
    fn format_batch(&self, results: &[HashResult]) -> String;
    fn write_to_file(&self, content: &str, path: &Path) -> Result<(), IoError>;
}
```

### Service Layer (服务层)

#### Hash Generator (`src/service/generator.rs`)
- 核心哈希生成服务
- 管理算法注册表
- 处理缓存逻辑

```rust
pub struct HashGenerator {
    algorithms: AlgorithmRegistry,
    formatters: FormatRegistry,
    cache: Option<Arc<dyn Cache>>,
    config: GeneratorConfig,
}

impl HashGenerator {
    pub async fn generate_single(
        &self,
        function_name: &FunctionName,
        algorithm: &str,
    ) -> Result<HashResult, GeneratorError>;
    
    pub async fn generate_batch(
        &self,
        function_names: &[FunctionName],
        algorithm: &str,
    ) -> Result<Vec<HashResult>, GeneratorError>;
}
```

#### Batch Processor (`src/service/batch.rs`)
- 批量处理管理
- 并发控制和负载均衡
- 进度报告和错误恢复

```rust
pub struct BatchProcessor {
    concurrency: usize,
    chunk_size: usize,
    generator: Arc<HashGenerator>,
}

impl BatchProcessor {
    pub async fn process_stream<S>(&self, stream: S) -> impl Stream<Item = Result<HashResult, ProcessError>>
    where
        S: Stream<Item = FunctionName>;
    
    pub async fn process_file(&self, path: &Path) -> Result<Vec<HashResult>, ProcessError>;
}
```

#### Cache Manager (`src/service/cache.rs`)
- 结果缓存管理
- LRU 淘汰策略
- 持久化支持

```rust
pub trait Cache: Send + Sync {
    fn get(&self, key: &str) -> Option<HashResult>;
    fn put(&self, key: String, value: HashResult);
    fn clear(&self);
    fn size(&self) -> usize;
}

pub struct LruCache {
    inner: Arc<Mutex<lru::LruCache<String, HashResult>>>,
    max_size: usize,
}
```

### Core Layer (核心层)

#### Function Parser (`src/core/parser.rs`)
- 函数名解析和验证
- 支持多种编程语言语法
- 标准化处理

```rust
pub struct FunctionParser {
    validators: Vec<Box<dyn Validator>>,
    normalizers: Vec<Box<dyn Normalizer>>,
}

impl FunctionParser {
    pub fn parse(&self, input: &str) -> Result<FunctionName, ParseError>;
    pub fn validate(&self, name: &FunctionName) -> Result<(), ValidationError>;
    pub fn normalize(&self, name: &mut FunctionName) -> Result<(), NormalizationError>;
}
```

#### Algorithm Registry (`src/core/registry.rs`)
- 哈希算法注册和管理
- 动态加载支持
- 算法元数据管理

```rust
pub struct AlgorithmRegistry {
    algorithms: HashMap<String, Box<dyn HashAlgorithm>>,
    metadata: HashMap<String, AlgorithmMetadata>,
}

impl AlgorithmRegistry {
    pub fn register<A: HashAlgorithm + 'static>(&mut self, algorithm: A);
    pub fn get(&self, name: &str) -> Option<&dyn HashAlgorithm>;
    pub fn list_available(&self) -> Vec<&str>;
    pub fn get_metadata(&self, name: &str) -> Option<&AlgorithmMetadata>;
}
```

#### Format Registry (`src/core/format.rs`)
- 输出格式注册和管理
- 自定义格式支持
- 模板处理

```rust
pub struct FormatRegistry {
    formatters: HashMap<String, Box<dyn OutputFormatter>>,
    templates: HashMap<String, Template>,
}

impl FormatRegistry {
    pub fn register<F: OutputFormatter + 'static>(&mut self, formatter: F);
    pub fn format(&self, hash: &[u8], format_name: &str) -> Result<String, FormatError>;
    pub fn register_template(&mut self, name: String, template: Template);
}
```

### Algorithm Layer (算法层)

#### Hash Algorithm Trait (`src/algorithm/mod.rs`)
- 统一的哈希算法接口
- 支持流式和批量计算
- 性能监控支持

```rust
pub trait HashAlgorithm: Send + Sync {
    type Output: AsRef<[u8]> + Send;
    type Error: std::error::Error + Send + Sync + 'static;

    fn name(&self) -> &'static str;
    fn compute(&self, input: &str) -> Result<Self::Output, Self::Error>;
    fn compute_batch(&self, inputs: &[&str]) -> Result<Vec<Self::Output>, Self::Error>;
    fn reset(&mut self);
    
    // 性能相关
    fn expected_output_size(&self) -> usize;
    fn is_cryptographically_secure(&self) -> bool;
    fn performance_tier(&self) -> PerformanceTier;
}
```

#### 自定义哈希算法实现

##### Custom Function Hash (`src/algorithm/custom.rs`)
```rust
/// 自定义函数名哈希算法
/// 基于字符迭代的高效哈希函数，针对函数名特征优化
pub struct CustomHasher;

impl CustomHasher {
    pub fn new() -> Self {
        Self
    }
    
    /// 核心哈希计算函数
    /// 算法说明：
    /// 1. 初始化 h = 0
    /// 2. 对每个字符 c: h = 5527 * h + 7 * c
    /// 3. 计算 v = h & 0xFFFF (取低16位)
    /// 4. h ^= v * v (异或平方值)
    /// 5. 返回最终的 h 值
    pub fn compute_hash(&self, func_name: &str) -> u32 {
        let mut h: u32 = 0;
        
        for byte in func_name.bytes() {
            h = h.wrapping_mul(5527).wrapping_add(7 * (byte as u32));
            let v = h & 0x0000_FFFF;
            h ^= v.wrapping_mul(v);
        }
        
        h
    }
}

impl Default for CustomHasher {
    fn default() -> Self {
        Self::new()
    }
}
```

##### 算法特点分析
- **高性能**：简单的算数运算，无需复杂的加密操作
- **低冲突**：通过乘法和异或操作分散哈希值分布
- **固定输出**：32位无符号整数，适合快速比较
- **确定性**：相同输入总是产生相同输出
- **适用性**：专门针对函数名字符串特征设计

## 数据流设计

### 单个函数处理流程

```
输入函数名 → 验证格式 → 标准化 → 自定义哈希 → C数组格式化 → 输出
     ↓           ↓         ↓         ↓           ↓          ↓
   Parser   → Validator → Normalizer → CustomHash → CArrayFormatter → 
                                                                    ↓
                                    /*[0]*/{0x3D1EF464, (void *)func1},
```

### 批量处理流程

```
文件输入 → 分块读取 → 并行处理 → 结果聚合 → 输出写入
   ↓         ↓         ↓         ↓         ↓
Reader → Chunker → Workers → Aggregator → Writer
```

### 错误处理流程

```
错误发生 → 错误分类 → 恢复尝试 → 用户通知 → 程序决策
   ↓         ↓         ↓         ↓         ↓
Exception → Classifier → Recovery → Notification → Decision
```

## 并发模型

### Task-Based 并发
- 使用 Tokio 异步运行时
- 基于任务的并发模型
- 支持背压控制

```rust
pub struct ConcurrentProcessor {
    semaphore: Arc<Semaphore>,
    task_queue: Arc<Mutex<VecDeque<Task>>>,
    workers: Vec<JoinHandle<()>>,
}

impl ConcurrentProcessor {
    pub async fn process_concurrent<I, F>(&self, items: I, process_fn: F) -> Vec<Result<Output, Error>>
    where
        I: IntoIterator<Item = Input>,
        F: Fn(Input) -> Future<Output = Result<Output, Error>> + Send + Sync + Clone,
    {
        // 实现并发处理逻辑
    }
}
```

### 资源管理
- 内存池管理
- 连接池复用
- 优雅关闭机制

## 性能优化策略

### 1. 内存优化
- 零拷贝字符串处理
- 内存池复用
- 延迟分配策略

### 2. CPU 优化
- SIMD 指令使用
- 分支预测优化
- 缓存友好的数据结构

### 3. I/O 优化
- 异步文件操作
- 批量读写
- 缓冲区优化

### 4. 算法优化
- 增量哈希计算
- 并行哈希处理
- 硬件加速支持

## 可扩展性设计

### 插件系统
```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> semver::Version;
    fn dependencies(&self) -> &[&'static str];
    
    fn initialize(&mut self, context: &mut PluginContext) -> Result<(), PluginError>;
    fn register_algorithms(&self, registry: &mut AlgorithmRegistry);
    fn register_formatters(&self, registry: &mut FormatRegistry);
    fn shutdown(&mut self) -> Result<(), PluginError>;
}
```

### 动态加载
- 运行时插件发现
- 依赖解析和管理
- 版本兼容性检查

## 测试架构

### 单元测试
- 每个模块独立测试
- Mock 和 Stub 支持
- 属性测试 (Property Testing)

### 集成测试
- 端到端测试流程
- 性能基准测试
- 错误场景测试

### 压力测试
- 大文件处理测试
- 高并发场景测试
- 内存泄漏检测

## 部署架构

### 构建系统
- 多目标编译支持
- 优化配置管理
- 依赖静态链接

### 分发机制
- 预编译二进制
- 包管理器集成
- 容器化部署

### 监控和诊断
- 性能指标收集
- 错误追踪和报告
- 健康状况检查

---

*架构文档版本：1.0*  
*最后更新：2025年10月31日*