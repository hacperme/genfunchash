/// 自定义函数名哈希算法
/// 基于字符迭代的高效哈希函数，针对函数名特征优化
#[derive(Debug, Clone, Default)]
pub struct CustomHasher;

impl CustomHasher {
    /// 创建新的哈希器实例
    pub fn new() -> Self {
        Self
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
    pub fn compute_batch(&self, func_names: &[&str]) -> Vec<(String, u32)> {
        func_names
            .iter()
            .map(|name| (name.to_string(), self.compute_hash(name)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_consistency() {
        let hasher = CustomHasher::new();
        
        // 测试相同输入产生相同输出
        let hash1 = hasher.compute_hash("func1");
        let hash2 = hasher.compute_hash("func1");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_known_values() {
        let hasher = CustomHasher::new();
        
        // 测试已知的哈希值
        assert_eq!(hasher.compute_hash("func1"), 0x3D1EF464);
        assert_eq!(hasher.compute_hash("func2"), 0x3D021EC4);
        assert_eq!(hasher.compute_hash("func3"), 0x3D074302);
        assert_eq!(hasher.compute_hash("func4"), 0x3D04E4E6);
    }

    #[test]
    fn test_different_inputs() {
        let hasher = CustomHasher::new();
        
        // 测试不同输入产生不同输出
        let hash1 = hasher.compute_hash("func1");
        let hash2 = hasher.compute_hash("func2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_batch_computation() {
        let hasher = CustomHasher::new();
        let functions = vec!["func1", "func2", "func3"];
        
        let results = hasher.compute_batch(&functions);
        assert_eq!(results.len(), 3);
        
        // 验证批量计算结果与单独计算一致
        for (name, hash) in results {
            assert_eq!(hash, hasher.compute_hash(&name));
        }
    }
}