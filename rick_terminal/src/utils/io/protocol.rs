use std::cmp::min;

/// 传输协议
/// 标记: 4byte  4byte数据场地 nbyte json 数据

const FLAGS: &[u8] = &[0x01, 0xff, 0x02, 0xff];

/// 传输协议编码器，将字符串编码传输
pub struct Encode {}

impl Encode {

    /// 编码数据
    /// - `@param T` 可以转换为 str 的类型
    /// - `@param` `content` 需要编码的内容
    /// - `@return` 返回编码后的信息
    pub fn encode<T: AsRef<str>>(content: T) -> Vec<u8> {
        let mut buf = Vec::<u8>::from(content.as_ref());
        let content_length = buf.len() as u32;
        buf.insert(0, FLAGS[0]);
        buf.insert(1, FLAGS[1]);
        buf.insert(2, FLAGS[2]);
        buf.insert(3, FLAGS[3]);
        // 长度
        buf.insert(4, (content_length & 0xff) as _);
        buf.insert(5, (content_length >> 8 & 0xff) as _);
        buf.insert(6, (content_length >> 16 & 0xff) as _);
        buf.insert(7, (content_length >> 24 & 0xff) as _);
        buf
    }
}

/// 解码器
pub struct Decode {
    /// 缓冲区
    cache_buf: Vec<u8>,
    /// 解码后的回调函数
    callback: Box<dyn Fn(&[u8])>,
    /// 缓冲区数据
    data_cache: Vec<u8>,
    /// 解析模式 true 解析重, false 没解析
    mode: bool,
    /// 正在解析消息的数据长度
    length: u32
}

/// 判断指定偏移量的数据是否和指定数组数据一致
/// - @param data 数据缓冲区
/// - @param offset 开始对比的偏移量
/// - @param flag 需要对比的标记数组
/// - @return 返回对比结果
fn start_with(data: &Vec<u8>, offset: usize, flag: &[u8]) -> bool {
    let result = true;
    for index in 0..flag.len() {
        if data.len() <= (index + offset) || data[index + offset] != flag[index] {
            return false;
        }
    }
    result
}

impl Decode {
    ///
    pub fn new(callback: Box<dyn Fn(&[u8])>) -> Self {
        Self {
            callback,
            mode: false,
            length: 0,
            data_cache: Vec::new(),
            cache_buf: Vec::new(),
        }
    }
    /// 开始解码
    /// - @param data: 需要解码的数据片段
    pub fn decode<T: AsRef<[u8]>>(&mut self, data: T) {
        let buf = data.as_ref();
        // 1. 判断是否存在缓存
        let mut pool = if self.cache_buf.is_empty() {
            Vec::from(buf)
        } else {
            let mut pool = self.cache_buf.clone();
            self.cache_buf.clear();
            for x in buf {
                pool.push(*x);
            }
            pool
        };
        // 1.2 判断是否是解析模式
        // 2. 判断是否足够解析内容
        // 3. 如果够的话，开始解析 开始判断是否存在标记头
        let mut index = 0_usize;
        loop {
            // 判断是否是解析模式
            if self.mode {
                // 计算剩余需要解析的内容
                let length = self.length - self.data_cache.len() as u32;
                // 获取最小解析长度
                let available_length = min(length, (pool.len() - index) as _);
                for i in 0..available_length {
                    self.data_cache.push(pool[index + i as usize]);
                }
                // 判断是否足够
                if self.data_cache.len() as u32 == self.length {
                    // 开始调用，关闭解析模式
                    self.mode = false;
                    self.length = 0;
                    (self.callback)(self.data_cache.as_slice());
                    self.data_cache.clear();
                }
                index += available_length as usize;
                continue;
            }
            if pool.len() - index <= 8 {
                // 不够，放到下次解析
                self.cache_buf = Vec::from(&pool[index..]);
                break;
            }
            // 对齐标记
            if !start_with(&pool, index, FLAGS) {
                // 对齐失败，说明存在脏数据
                index += 1;
                continue;
            }
            // 如果对齐成功,开始读取长度
            let mut length = 0_u32;
            let length_offset = index + FLAGS.len();
            length |= pool[length_offset + 3_usize] as u32;
            length << 8;
            length |= pool[length_offset + 2_usize] as u32;
            length << 8;
            length |= pool[length_offset + 1_usize] as u32;
            length << 8;
            length |= pool[length_offset + 0_usize] as u32;
            // 判断缓冲期中的数据是否住够读取
            index += 8;
            // 判断剩余数据是否足够
            if length <= (pool.len() - index) as _ {
                // 读取全部数据
                let data = &pool[index..(index + length as usize)];
                (self.callback)(data);
                index += length as usize;
            } else {
                // 读取部分数据，暂存到缓冲区
                self.mode = true;
                self.length = length;
                self.data_cache = Vec::from(&pool[index..pool.len()]);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::io::protocol::FLAGS;

    #[test]
    fn test_encode() {
        let content = "HelloWorld";
        println!("{:?}", super::Encode::encode(content));
        println!("{:?}", String::from_utf8_lossy(super::Encode::encode(content).as_slice()));
    }

    #[test]
    fn test_decode() {
        let mut decode = super::Decode::new(Box::new(|data: &[u8]| {
            println!("{:?}", data);
            println!("{:?}", String::from_utf8_lossy(data));
        }));
        let mut data = Vec::from(FLAGS);
        data.push(10);
        data.push(0);
        data.push(0);
        data.push(0);
        let content = "HelloWorld";
        let bytes = content.as_bytes();
        for x in bytes {
            data.push(*x);
        }
        decode.decode(data);
    }

    #[test]
    fn test_encode_decode() {
        let mut decode = super::Decode::new(Box::new(|data: &[u8]| {
            println!("{:?}", data);
            println!("{:?}", String::from_utf8_lossy(data));
        }));
        decode.decode(super::Encode::encode("HelloWorld"));
        decode.decode(super::Encode::encode("HelloWorldData"));
        decode.decode(super::Encode::encode("HelloWorld啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊"));
    }
}