mod cli;
mod config;
mod formatter;
mod hash;
mod processor;

use cli::CliArgs;
use config::Configuration;
use processor::Processor;
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    
    // 验证参数
    if let Err(e) = args.validate() {
        eprintln!("参数错误: {}", e);
        std::process::exit(1);
    }
    
    // 加载配置
    let config = if let Some(config_path) = &args.config {
        Configuration::load_from_file(config_path)?
    } else {
        Configuration::default()
    };
    
    // 创建处理器
    let processor = Processor::new(config);
    
    // 根据参数执行相应操作
    if args.is_single_function() {
        let function_name = args.function.as_ref().unwrap();
        processor.process_single(function_name, args.output.as_deref())?;
    } else if args.is_batch_mode() {
        let input_path = args.input.as_ref().unwrap();
        processor.process_batch(input_path, args.output.as_deref())?;
    }
    
    // 显示统计信息（如果启用详细模式）
    if args.verbose {
        let stats = processor.get_stats();
        eprintln!("处理完成，共处理 {} 个函数", stats.total_processed);
    }
    
    Ok(())
}
