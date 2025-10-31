use crate::hash::CustomHasher;
use crate::formatter::CArrayFormatter;
use crate::config::Configuration;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// 核心处理器
pub struct Processor {
    hasher: CustomHasher,
    formatter: CArrayFormatter,
    config: Configuration,
}

impl Processor {
    /// 创建新的处理器
    pub fn new(config: Configuration) -> Self {
        Self {
            hasher: CustomHasher::new(),
            formatter: CArrayFormatter::new(),
            config,
        }
    }
    
    /// 处理单个函数名
    pub fn process_single(&self, function_name: &str, output_path: Option<&Path>) -> Result<()> {
        let hash = self.hasher.compute_hash(function_name);
        let formatted = self.formatter.format_single(function_name, hash);
        
        self.write_output(&formatted, output_path)
    }
    
    /// 处理多个函数名（从文件读取）
    pub fn process_batch(&self, input_path: &Path, output_path: Option<&Path>) -> Result<()> {
        let functions = self.read_functions_from_file(input_path)?;
        let results = self.hasher.compute_batch(&functions.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let formatted = self.formatter.format_batch(&results);
        
        self.write_output(&formatted, output_path)
    }
    
    /// 从文件读取函数名列表
    fn read_functions_from_file(&self, path: &Path) -> Result<Vec<String>> {
        let file = fs::File::open(path)
            .with_context(|| format!("无法打开输入文件: {:?}", path))?;
        
        let reader = BufReader::new(file);
        let mut functions = Vec::new();
        
        for (line_num, line) in reader.lines().enumerate() {
            let line = line.with_context(|| format!("读取第 {} 行失败", line_num + 1))?;
            let trimmed = line.trim();
            
            // 跳过空行和注释行
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                // 验证函数名格式
                if self.is_valid_function_name(trimmed) {
                    functions.push(trimmed.to_string());
                } else {
                    eprintln!("警告: 第 {} 行包含无效的函数名: {}", line_num + 1, trimmed);
                }
            }
        }
        
        if functions.is_empty() {
            return Err(anyhow::anyhow!("输入文件中没有找到有效的函数名"));
        }
        
        Ok(functions)
    }
    
    /// 验证函数名格式
    fn is_valid_function_name(&self, name: &str) -> bool {
        // 基本验证：不为空，只包含字母数字和下划线，不以数字开头
        if name.is_empty() {
            return false;
        }
        
        // 第一个字符必须是字母或下划线
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return false;
        }
        
        // 其余字符必须是字母、数字或下划线
        name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    }
    
    /// 写入输出
    fn write_output(&self, content: &str, output_path: Option<&Path>) -> Result<()> {
        match output_path {
            Some(path) => {
                fs::write(path, content)
                    .with_context(|| format!("无法写入输出文件: {:?}", path))?;
                
                if !self.config.default.output_format.contains("quiet") {
                    println!("输出已写入: {:?}", path);
                }
            }
            None => {
                println!("{}", content);
            }
        }
        
        Ok(())
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> ProcessorStats {
        ProcessorStats {
            total_processed: self.formatter.current_index(),
        }
    }
}

/// 处理器统计信息
pub struct ProcessorStats {
    pub total_processed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_valid_function_names() {
        let processor = Processor::new(Configuration::default());
        
        assert!(processor.is_valid_function_name("func1"));
        assert!(processor.is_valid_function_name("my_function"));
        assert!(processor.is_valid_function_name("_private_func"));
        assert!(processor.is_valid_function_name("Function123"));
        
        assert!(!processor.is_valid_function_name(""));
        assert!(!processor.is_valid_function_name("123func"));
        assert!(!processor.is_valid_function_name("func-name"));
        assert!(!processor.is_valid_function_name("func name"));
    }

    #[test]
    fn test_process_single() {
        let processor = Processor::new(Configuration::default());
        let result = processor.process_single("func1", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_functions_from_file() -> Result<()> {
        let processor = Processor::new(Configuration::default());
        
        // 创建临时文件
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "func1")?;
        writeln!(temp_file, "func2")?;
        writeln!(temp_file, "# 这是注释")?;
        writeln!(temp_file, "")?; // 空行
        writeln!(temp_file, "func3")?;
        
        let functions = processor.read_functions_from_file(temp_file.path())?;
        assert_eq!(functions, vec!["func1", "func2", "func3"]);
        
        Ok(())
    }
}