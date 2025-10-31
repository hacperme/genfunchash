use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// C 语言数组格式化器
#[derive(Debug)]
pub struct CArrayFormatter {
    index_counter: Arc<AtomicUsize>,
}

impl CArrayFormatter {
    /// 创建新的格式化器
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
    
    /// 格式化单个函数（从索引 0 开始）
    pub fn format_single(&self, function_name: &str, hash: u32) -> String {
        self.reset_index();
        self.format_entry(function_name, hash)
    }
    
    /// 格式化多个函数
    pub fn format_batch(&self, entries: &[(String, u32)]) -> String {
        self.reset_index();
        entries
            .iter()
            .map(|(name, hash)| self.format_entry(name, *hash))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// 重置索引计数器
    pub fn reset_index(&self) {
        self.index_counter.store(0, Ordering::Relaxed);
    }
    
    /// 获取当前索引值
    pub fn current_index(&self) -> usize {
        self.index_counter.load(Ordering::Relaxed)
    }
}

impl Default for CArrayFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_format() {
        let formatter = CArrayFormatter::new();
        let result = formatter.format_single("func1", 0x3D1EF464);
        assert_eq!(result, "/*[  0]*/{0x3D1EF464,                             (void *)func1},");
    }

    #[test]
    fn test_batch_format() {
        let formatter = CArrayFormatter::new();
        let entries = vec![
            ("func1".to_string(), 0x3D1EF464),
            ("func2".to_string(), 0x3D021EC4),
        ];
        
        let result = formatter.format_batch(&entries);
        let expected = "/*[  0]*/{0x3D1EF464,                             (void *)func1},\n/*[  1]*/{0x3D021EC4,                             (void *)func2},";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_index_increment() {
        let formatter = CArrayFormatter::new();
        
        formatter.format_entry("func1", 0x3D1EF464);
        formatter.format_entry("func2", 0x3D021EC4);
        
        assert_eq!(formatter.current_index(), 2);
    }

    #[test]
    fn test_reset_index() {
        let formatter = CArrayFormatter::new();
        
        formatter.format_entry("func1", 0x3D1EF464);
        assert_eq!(formatter.current_index(), 1);
        
        formatter.reset_index();
        assert_eq!(formatter.current_index(), 0);
    }
}