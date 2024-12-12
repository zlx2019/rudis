#![allow(dead_code)]
#![allow(unused_variables)]

mod encode;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use anyhow::Result;


/// RespFrame 编码
/// 将Resp 数据类型序列化为字节流
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

/// RespFrame 解码
/// 将字节流反序列化为 Resp数据类型
pub trait RespDecode{
    fn decode(buf: Self) -> Result<RespFrame, String>;
}


/// 针对于 Redis RESP 协议支持的数据类型
/// 参考于：https://redis.io/docs/latest/develop/reference/protocol-spec/#resp-protocol-description
/// 
/// 用于表示报文中传输的不同类型的数据
pub enum RespFrame {
    SimpleString(SimpleString),         // 简单字符串类型
    SimpleError(SimpleError),           // 简单错误类型
    Integer(i64),                       // 整数类型
    Boolean(bool),                      // 布尔类型
    Double(f64),                        // 浮点型数据
    BigNumber(i128),                    // 大数字类型

    NullBulkString(NullBulkString),

    Nil(Null),                           // 表示不存在的值(nil)
    ArrayNil(Vec<Null>),
    BulkString(BulkString),             // 批量字符串类型
    BulkErrors(Option<Vec<u8>>),        // 批量错误
    BulkResp(Arrays),                   // 批量数据类型
    RespMap(HashMap<String, RespFrame>),    // Map类型
    RespSet(HashSet<RespFrame>)             // Set类型

}

/// 简单字符串类型，该字符串不得包含 CR ( \r ) 或 LF ( \n ) 字符，并以 CRLF 终止（即\r\n ）。
/// 报文格式：`+[message]\r\n`  示例：`+OK\r\n`
pub struct SimpleString(String);
/// 简单错误类型，响应字符串
/// 报文格式：-[Error message]\r\n  示例：-ERR unknown command 'asdf\r\n'
pub struct SimpleError(String);
/// 批量字符串类型
pub struct BulkString(Vec<u8>);
/// 批量字符串空值
pub struct NullBulkString;
/// 批量数据类型
pub struct Arrays(Vec<RespFrame>);
/// 表示数据类型不存在的值
/// 报文格式为: `_\r\n`
pub struct Null;
pub struct ArrayNil;
pub struct RespMap(HashMap<String, RespFrame>);
pub struct RespSet(HashSet<RespFrame>);


impl Deref for SimpleString{
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SimpleString {
    pub fn new(value: impl Into<String>) -> Self{
        Self(value.into())
    }
}

impl Deref for BulkString{
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for SimpleError{
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for Arrays{
    type Target = Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for RespMap {
    type Target = HashMap<String, RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for RespSet {
    type Target = HashSet<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
