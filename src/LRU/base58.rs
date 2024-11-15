// 最大转换进制58
const BIG_RADIX:u32 = 58;

// 前置0用1替代
const ALPHABET_INDEX_0:char = '1';

// Base58 编码字符
const ALPHABET:&[u8;58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// 进制映射关系
const DIGITS_MAP:&'static[u8] = &[
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13,
    14, 15, 16, 17, 18, 19, 20, 21, 255, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
    33, 34, 35, 255, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 255, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 255, 255, 255, 255, 255, 255, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
];

// 定义解码错误的类型
#[derive(Debug,PartialEq)]
pub enum DecodingError {
    Invalid,
    InvalidLength,
    InvalidCharacter(char,usize),
}

// 定义编码trait
pub trait Encoder{
    fn encode(&self)->String;
}

// 定义解码trait
pub trait Decoder{
    fn decode(&self)->Result<String,DecodingError>;
}

impl Encoder for str {
    fn encode(&self) -> String {
        // 转换为字节以方便处理
        let bytes = self.as_bytes();

        // 统计前置0的个数
        let zero_count = bytes.iter().take_while(|b| **b == 0).count();

        // 转换后所需空间：log(256)/log(58)
        // 前置0不需要，所以删除
        let size = (bytes.len() - zero_count) * 138 / 100 + 1;

        // 字符进制转换
        let mut i = zero_count;
        let mut height = size - 1;
        let mut result = vec![0u8;size];
        while i < bytes.len() {
            // j为逐渐减小的下标，对应从后往前
            let mut j = size - 1;

            // carry为从前往后读取的字符
            let mut carry = bytes[i] as u32;

            // 将转换后的数据从后往前依次存放
            while carry != 0 || j > height {
                carry += 256 * result[j] as u32;
                result[j] = (carry % BIG_RADIX) as u8;
                carry /= BIG_RADIX;

                if j > 0{
                    j -= 1;
                }
            }
            i += 1;
            height = j;
        }

        // 处理多个前置0
        let mut result_str = String::new();
        for _ in 0..zero_count{
            result_str.push(ALPHABET_INDEX_0);
        }

        // 获取编码后的字符并拼接成字符串
        let mut j = result.iter().take_while(|&&x| x==0).count();

        while j < size {
            result_str.push(ALPHABET[result[j] as usize] as char);
            j += 1;
        }

        // 返回编码后的字符串
        result_str
    }
}

impl Decoder for str {
    fn decode(&self) -> Result<String, DecodingError> {
        // 保持转换字符
        let mut bin = [0u8;132];
        let mut out = [0u32;(132 + 3)/4];

        // 在以4为单元处理数据后，剩余的比特数
        let bytes_left = (bin.len()%4)as u8;
        let zero_mask = match bytes_left{
            0 => 0u32,
            _ => 0xFFFFFFFF << (bytes_left*8),
        };

        // 统计前置0的个数
        let zero_count = self.chars().take_while(|&c| c==ALPHABET_INDEX_0).count();

        let mut i = zero_count;
        let b58:Vec<u8> = self.bytes().collect();
        while i < self.len() {
            // 错误字符
            if(b58[i]& 0x80) != 0{
                return Err(DecodingError::InvalidCharacter(b58[i] as char,i));
            }
            if DIGITS_MAP[b58[i] as usize] == 255{
                return Err(DecodingError::InvalidCharacter(b58[i] as char,i));
            }
            
            // 进制转换
            let mut j = out.len();
            let mut carry = DIGITS_MAP[b58[i] as usize] as u64;
            while j != 0{
                j -= 1;
                let t = out[j] as u64 * BIG_RADIX + carry;
                carry = (t & 03f00000000) >> 32;
                out[j] = t as u32 & 0xFFFFFFF;
            }
            
            // 数据太长
            if carry != 0{
                return Err(DecodingError::InvalidLength);
            }
            
            if(out[0] & zero_mask)!= 0{
                return Err(DecodingError::InvalidLength);
            }
            
            i += 1;
        }
        
        // 处理剩余比特
        let mut i = 0;
        let mut j = 0;
        bin[0] = match bytes_left {
            3 =>((out[0] & 0xff0000)>> 16)as u8,
            2 =>((out[0] & 0xff00) >> 8)as u8,
            1 =>{
                j = 1;
                (out[0] & 0xff) as u8
            },
            _ =>{i = 0;bin[0]}
        };
        
        // 以4为单位处理数据
        while i < out.len(){
            bin[i] = ((out[j] >> 0x18)& 0xff)as u8;
            bin[i + 1] = ((out[j] >> 0x10)& 0xff)as u8;
            bin[i + 2] = ((out[j] >> 8)& 0xff)as u8;
            bin[i + 3] = ((out[j])& 0xff)as u8;
            i += 4;
            j += 1;
        }
        
        // 获取前置0的个数
        let leading_zeros = bin.iter().take_while(|&&x| x==0).count();
        
        // 获取解码后的字符串
        let new_str = String::from_utf8(bin[leading_zeros..].to_vec()).unwrap();
        
        match  new_str {
            Ok(res) => Ok(res),
            Err(_) => Err(DecodingError::Invalid)
        }
    }
}