use clap::Parser;
use std::path::PathBuf;

/// GenFuncHash - 为函数名生成哈希值的命令行工具
#[derive(Parser)]
#[command(name = "genfunchash")]
#[command(about = "Generate hashes for function names in C array format")]
#[command(version = "1.0.0")]
pub struct CliArgs {
    /// 单个函数名
    #[arg(short, long, value_name = "FUNCTION")]
    pub function: Option<String>,

    /// 输入文件路径
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// 输出文件路径
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// 配置文件路径
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// 启用结果缓存
    #[arg(long)]
    pub enable_cache: bool,

    /// 并发处理数
    #[arg(long, default_value = "4")]
    pub concurrency: usize,

    /// 静默模式
    #[arg(short, long)]
    pub quiet: bool,

    /// 详细输出
    #[arg(short, long)]
    pub verbose: bool,
}

impl CliArgs {
    /// 验证命令行参数
    pub fn validate(&self) -> Result<(), String> {
        // 必须提供 function 或 input 中的一个
        if self.function.is_none() && self.input.is_none() {
            return Err("必须提供 --function 或 --input 参数".to_string());
        }

        // 不能同时提供 function 和 input
        if self.function.is_some() && self.input.is_some() {
            return Err("不能同时使用 --function 和 --input 参数".to_string());
        }

        // 验证并发数
        if self.concurrency == 0 {
            return Err("并发数必须大于 0".to_string());
        }

        Ok(())
    }

    /// 检查是否为单个函数模式
    pub fn is_single_function(&self) -> bool {
        self.function.is_some()
    }

    /// 检查是否为批量处理模式
    pub fn is_batch_mode(&self) -> bool {
        self.input.is_some()
    }
}
