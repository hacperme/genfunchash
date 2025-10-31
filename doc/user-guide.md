# GenFuncHash 用户手册

## 目录

1. [安装](#安装)
2. [快速开始](#快速开始)
3. [命令行选项](#命令行选项)
4. [配置文件](#配置文件)
5. [哈希算法说明](#哈希算法说明)
6. [输出格式](#输出格式)
7. [使用场景](#使用场景)
8. [最佳实践](#最佳实践)
9. [故障排除](#故障排除)

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/your-username/genfunchash.git
cd genfunchash

# 编译安装
cargo build --release
cargo install --path .
```

### 使用 Cargo 安装

```bash
cargo install genfunchash
```

### 预编译二进制文件

从 [Releases](https://github.com/your-username/genfunchash/releases) 页面下载适合您操作系统的预编译二进制文件。

## 快速开始

### 基本用法

生成单个函数名的哈希值：

```bash
genfunchash -f "func1"
```

输出：

```c
/*[ 0]*/{0x3D1EF464,                             (void *)func1},
```

### 批量处理

```bash
genfunchash -i functions.txt -o hash_table.c
```

输出文件内容：

```c
/*[ 0]*/{0x3D1EF464,                             (void *)func1},
/*[ 1]*/{0x3D021EC4,                             (void *)func2},
/*[ 2]*/{0x3D074302,                             (void *)func3},
```

### 批量处理

从文件读取函数名列表：

```bash
genfunchash -i functions.txt -o results.txt
```

`functions.txt` 内容示例：
```
func1
func2
func3
func4
```

## 命令行选项

### 基本选项

| 选项 | 短选项 | 描述 | 默认值 |
|------|--------|------|--------|
| `--function` | `-f` | 单个函数名 | - |
| `--input-file` | `-i` | 输入文件路径 | - |
| `--output-format` | `-o` | 输出格式 | `hex` |
| `--output-file` | `-O` | 输出文件路径 | - |
| `--config` | `-c` | 配置文件路径 | - |

### 高级选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--enable-cache` | 启用结果缓存 | `false` |
| `--concurrency` | 并发处理数 | `4` |
| `--quiet` | 静默模式 | `false` |
| `--verbose` | 详细输出 | `false` |
| `--json` | JSON 格式输出 | `false` |

### 使用示例

```bash
# 基本用法 - 单个函数
genfunchash -f "func1"

# 批量处理 - 从文件读取
genfunchash -i functions.txt -o hash_table.c

# 启用缓存提升性能
genfunchash -i functions.txt --enable-cache --concurrency 8

# 详细输出模式
genfunchash -f "func1" --verbose

# 指定输出文件
genfunchash -i functions.txt -o hash_functions.c
```

## 配置文件

### 配置文件格式

创建 `genfunchash.toml` 配置文件：

```toml
[default]
output_format = "c-array"
enable_cache = true
concurrency = 8

[cache]
max_entries = 10000
ttl_seconds = 3600

[output]
include_timestamp = true
include_index = true
separator = "\t"

[batch]
chunk_size = 1000
parallel_processing = true
```

### 配置选项说明

#### `[default]` 部分
- `output_format`: 默认输出格式
- `enable_cache`: 是否启用缓存
- `concurrency`: 并发处理数

#### `[cache]` 部分
- `max_entries`: 缓存最大条目数
- `ttl_seconds`: 缓存生存时间（秒）

#### `[output]` 部分
- `include_timestamp`: 是否包含时间戳
- `include_index`: 是否包含数组索引
- `separator`: 字段分隔符

#### `[batch]` 部分
- `chunk_size`: 批处理块大小
- `parallel_processing`: 是否启用并行处理

## 哈希算法说明

本工具使用专门为函数名设计的自定义哈希算法：

### 算法特点

| 特征 | 描述 |
|------|------|
| **输出长度** | 32 位无符号整数 |
| **性能** | 极高 - 仅使用基本算术运算 |
| **确定性** | 相同输入总是产生相同输出 |
| **冲突率** | 低 - 针对函数名特征优化 |

### 算法实现

```rust
fn compute_hash(func_name: &str) -> u32 {
    let mut h: u32 = 0;
    for byte in func_name.bytes() {
        h = h.wrapping_mul(5527).wrapping_add(7 * (byte as u32));
        let v = h & 0x0000_FFFF;
        h ^= v.wrapping_mul(v);
    }
    h
}
```

### 使用场景

- **函数去重**: 快速识别重复的函数名
- **代码指纹**: 为代码库生成唯一标识
- **符号表索引**: 高效的符号查找和比较

## 输出格式

### 默认输出格式

工具生成 C 语言数组初始化格式，用于直接嵌入到 C 代码中：

```c
/*[索引]*/{0x哈希值,                             (void *)函数名},
```

### 格式说明

| 组件 | 描述 | 示例 |
|------|------|------|
| `/*[索引]*/` | 注释形式的数组索引 | `/*[ 0]*/`, `/*[ 1]*/` |
| `0x哈希值` | 32位十六进制哈希值 | `0x3D1EF464` |
| `(void *)函数名` | 函数指针转换 | `(void *)func1` |

### 输出模板

使用 `--template` 选项自定义输出格式：

```bash
genfunchash -f "func1" --template "{function}:{algorithm}:{hash}"
```

支持的模板变量：
- `{function}`: 函数名
- `{algorithm}`: 算法名称
- `{hash}`: 哈希值
- `{timestamp}`: 时间戳
- `{format}`: 输出格式

## 使用场景

### 1. 代码去重

识别重复的函数名：

```bash
# 生成所有函数的哈希值
genfunchash -i all_functions.txt -o hashes.txt

# 查找重复
sort hashes.txt | uniq -d
```

### 2. 代码指纹

为代码库生成指纹：

```bash
# 提取函数名并生成哈希
grep -r "function\|def\|func" src/ | cut -d: -f2 | genfunchash -i - > fingerprint.txt
```

### 3. API 版本控制

跟踪 API 函数变化：

```bash
# 为当前版本生成哈希
genfunchash -i api_functions.txt -o v1.0_hashes.txt

# 比较版本差异
diff v1.0_hashes.txt v2.0_hashes.txt
```

### 4. 安全审计

识别可疑函数名：

```bash
# 生成函数哈希并与已知恶意哈希比较
genfunchash -i suspicious_functions.txt --json | jq '.hash' | sort > current_hashes.txt
comm -12 current_hashes.txt known_malicious.txt
```

## 最佳实践

### 1. 性能优化

- 对于大量函数名，启用缓存：`--enable-cache`
- 调整并发数以匹配 CPU 核心数：`--concurrency 8`
- 使用快速算法进行初步筛选：`-a md5`

### 2. 安全考虑

- 对于安全敏感应用，使用 SHA-256 或更强算法
- 定期更新工具以获取安全修复
- 不要在生产环境中使用 MD5 进行安全验证

### 3. 输出管理

- 使用配置文件统一团队设置
- 对于自动化脚本，使用 `--json` 输出
- 启用时间戳便于审计：`include_timestamp = true`

### 4. 错误处理

- 使用 `--verbose` 模式调试问题
- 检查输入文件的编码格式
- 处理特殊字符和 Unicode 函数名

## 故障排除

### 常见问题

#### 1. "Unknown algorithm" 错误

**问题**: 指定了不支持的哈希算法

**解决方案**:
```bash
# 查看支持的算法
genfunchash --help

# 使用支持的算法
genfunchash -f "func1" -a sha256
```

#### 2. 文件读取错误

**问题**: 无法读取输入文件

**解决方案**:
- 检查文件路径是否正确
- 确认文件权限
- 验证文件编码（推荐 UTF-8）

#### 3. 内存使用过高

**问题**: 处理大文件时内存不足

**解决方案**:
```bash
# 减少并发数
genfunchash -i large_file.txt --concurrency 2

# 分批处理
split -l 10000 large_file.txt chunk_
for chunk in chunk_*; do
    genfunchash -i "$chunk" >> results.txt
done
```

#### 4. 性能问题

**问题**: 处理速度缓慢

**解决方案**:
- 启用缓存提升性能：`--enable-cache`
- 启用缓存：`--enable-cache`
- 增加并发数：`--concurrency 16`

### 调试模式

启用详细输出获取更多信息：

```bash
genfunchash -f "func1" --verbose
```

### 日志文件

设置环境变量启用日志：

```bash
export RUST_LOG=debug
genfunchash -f "func1" 2> debug.log
```

### 支持

- GitHub Issues: [项目地址]/issues
- 文档: [项目地址]/docs
- 社区: [讨论区地址]

---

*用户手册版本：1.0*  
*最后更新：2025年10月31日*