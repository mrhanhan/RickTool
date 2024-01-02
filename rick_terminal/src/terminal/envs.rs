use std::collections::hash_map::HashMap;

/// 环境变量表达式解析器 多个值使用; 分割，在 linux 会被解析为 : TODO 还未处理路径分隔符情况
/// 支持格式: {} {{}{{}}}
/// 解析过程重忽略大小写
pub struct EnvsExpressionParse {
    /// 环境变了
    envs: HashMap<String, String>
}


impl EnvsExpressionParse {
    /// 创建新的解析器
    /// @param include_system_envs 是否包含系统环境变量
    /// whether to include system environment variables
    pub fn new(include_system_envs: bool) -> Self {
        let mut parse = Self {
            envs: HashMap::new()
        };
        if include_system_envs {
            for (k, v) in std::env::vars() {
                parse.envs.insert(k.to_lowercase(), v);
            }
        }
        parse
    }
    /// 添加环境变量
    /// @param key 还款变量key @value 环境变量值
    pub fn add_env<T: ToString>(&mut self, key: T, value: T) {
        self.envs.insert(key.to_string().to_lowercase(), value.to_string());
    }
    /// 解析内容, 将表达式替换为直
    pub fn parse<T: ToString>(&self, content: T) -> Result<String, String> {
        let mut index = 0_usize;
        let content = content.to_string();
        let array: Vec<char> = content.chars().collect();
        let mut value = String::new();
        // 转义符
        let mut escape = false;
        let mut parse_key = false;
        let mut keys = Vec::<String>::new();
        let mut key = String::new();
        loop {
            if index >= array.len() {
                // 判断语法是否存在问题
                if parse_key || keys.len() > 1 {
                    return Err("解析出错，语法存在问题".into());
                }
                break;
            }
            let code = array[index];
            index += 1;
            // 判断是否是特殊符号，并且经过转义
            if escape && (code == '\\' || code == '{' || code == '}'){
                // 是否是正常字符 \\{
                escape = false;
                // 是否是处理的key
                if parse_key {
                    key.push(code);
                } else {
                    value.push(code);
                    continue;
                }
            }
            // 判断是否存在转移符号
            if code == '\\' {
                // 是否是第一次出现 \{
                escape = true;
                continue;
            }
            // 判断是否是出现了特殊的符号 {
            if code == '{' {
                // 判断是否是嵌套变量
                if parse_key {
                    keys.push(key.clone());
                    key.clear();
                } else {
                    // 不是嵌套变量
                    parse_key = true;
                }
                continue;
            }
            // 判断在解析变量时候的闭合
            if parse_key && code == '}' {
                // 1. 处理当前key重的内容 判断环境重是否存在key
                let mut key_value = String::new();
                let mut temp_key = key.to_lowercase();
                if self.envs.contains_key(&temp_key) {
                    // 如果存在 替换变量
                    key_value.push_str(self.envs.get(&temp_key).unwrap().as_str());
                }
                // 2. 判断是否存在嵌套变量
                if keys.is_empty() {
                    // 2.1 不是嵌套变量，这时候需要把值添加到全局中
                    value.push_str(key_value.as_str());
                    key.clear();
                    parse_key = false;
                } else {
                    let mut temp_key = keys.pop().unwrap();
                    temp_key.push_str(key_value.as_str());
                    key = temp_key;
                }
                continue;
            }
            // 判断是否是在解析变量
            if parse_key {
                key.push(code);
            } else {
                value.push(code);
            }
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse() {
        let parse = super::EnvsExpressionParse::new(true);
        println!("Hello Path:{}", parse.parse("{PATH}\\}{{A}DATA}").unwrap());
    }
    #[test]
    fn test_parse_add() {
        let mut parse = super::EnvsExpressionParse::new(true);
        parse.add_env("A", "PATH");
        parse.add_env("B", "A");
        parse.add_env("C", "B");
        println!("Hello Path:{}", parse.parse("{{A}}").unwrap());
        println!("Hello A:{}", parse.parse("{{C}}").unwrap());
    }

}