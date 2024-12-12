use itoa::Buffer;
use super::{Arrays, BulkString, Null, NullBulkString, RespEncode, RespFrame, RespMap, RespSet, SimpleError, SimpleString};

/// Resp 协议编码
/// 实现 RespFrame 的所有类型 --> Vec<u8> 之间的转换
/// 报文格式: https://redis.io/docs/latest/develop/reference/protocol-spec/#resp-protocol-description


// 协议终止符，也用于分割报文。
const CRLF: &[u8] = b"\r\n";
const BUF_SIZE: usize = 4096;

/// RespFram to bytes
impl RespEncode for RespFrame {
    fn encode(self) -> Vec<u8> {
        match self {
            RespFrame::SimpleString(simple_string) => todo!(),
            RespFrame::SimpleError(simple_error) => todo!(),
            RespFrame::Integer(_) => todo!(),
            RespFrame::Boolean(_) => todo!(),
            RespFrame::Double(v) => v.encode(),
            RespFrame::BigNumber(_) => todo!(),
            RespFrame::NullBulkString(null_bulk_string) => todo!(),
            RespFrame::Nil(null) => todo!(),
            RespFrame::ArrayNil(vec) => todo!(),
            RespFrame::BulkString(bulk_string) => todo!(),
            RespFrame::BulkErrors(vec) => todo!(),
            RespFrame::BulkResp(arrays) => todo!(),
            RespFrame::RespMap(hash_map) => todo!(),
            RespFrame::RespSet(hash_set) => todo!(),
        }
    }
}

/// - Integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64{
    fn encode(self) -> Vec<u8> {
        let mut vec= Vec::with_capacity(23);
        vec.push(b':');
        if self >= 0 {
            vec.push(b'+');
        }
        // TODO fix opt
        let mut buf = Buffer::new();
        vec.extend_from_slice(buf.format(self).as_bytes());
        vec.extend_from_slice(CRLF);
        vec
    }
}

/// - SimpleString: "+OK\r\n"
impl RespEncode for SimpleString{
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 3);
        buf.push(b'+');
        buf.extend_from_slice(self.as_bytes());
        buf.extend_from_slice(CRLF);
        buf
    }
}

/// - BulkString: "<length>\r\n<data>\r\n"
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(CRLF);
        buf
    }
}

/// - Null: "_\r\n"
impl RespEncode for Null {
    fn encode(self) -> Vec<u8> {
        [b"_", CRLF].concat()
    }
}

/// - NullBulkString: "$-1\r\n"
impl RespEncode for NullBulkString{
    fn encode(self) -> Vec<u8> {
        [b"$-1", CRLF].concat()
    }
}

/// - Arrays: "*<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for Arrays {
    fn encode(self) -> Vec<u8> {
        todo!()
    }
}

/// SimpleError -> Bytes
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        [b"-", self.as_bytes(), CRLF].concat()
    }
}

/// Booleans: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(4);
        buf.push(b'#');
        buf.push(if self { b't' } else { b'f' });
        buf.extend_from_slice(CRLF);
        buf
    }
}

/// Doubles: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
/// 采用科学计数法
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        // is NaN 
        if self.is_nan() {
            return b",nan\r\n".to_vec();
        }
        if self.is_infinite() {
            if self.is_sign_positive() {
                // Positive infinity
                return b",+inf\r\n".to_vec();
            } else {
                // Negative infinity
                return b",-inf\r\n".to_vec();
            }
        }
        // Normal handle
        let encoded = if self.abs() >= 1e8 || (self.abs() < 1e-4 && self != 0.0) {
            format!(",{:+e}\r\n", self)
        } else {
            format!(",{:+}\r\n", self)
        };
        encoded.into_bytes()
    }
}


/// Maps: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
/// 示例: {name: zhangsan, age: 18} --> "%2\r\n$4\r\nname\r\n$8\r\nzhangsan\r\n$3\r\nage\r\n$2\r\n18\r\n" 
impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_SIZE);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (k,v) in self.0 {
            buf.extend_from_slice(&SimpleString::new(k).encode());
            buf.extend_from_slice(&v.encode());
        }
        buf
    }
}


/// Sets: "~<number-of-elements>\r\n<element-1>...<element-n>"
/// 示例: [小明,小王,小张]  --> "~3\r\n$6\r\n小明\r\n$6\r\n小王\r\n$6\r\n小张\r\n"
impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_SIZE);
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());
        for value in self.0{
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}



#[cfg(test)]
mod tests{
    use itoa::Buffer;

    use crate::protocol::{RespEncode, RespFrame};
    #[test]
    fn test_integer_to_bytes(){
        let n: i64 = i64::MAX;
        let mut v = Vec::with_capacity(23);
        v.push(b':');
        if n >= 0 {
            v.push(b'+');
        }
        v.extend_from_slice(Buffer::new().format(n).as_bytes());
        v.extend_from_slice(b"\r\n");
        assert_eq!(23, v.len());
        assert_eq!(23, v.capacity());
        println!("{}", String::from_utf8(v).unwrap());
    }

    #[test]
    fn test_doubles_encode(){
        let nan = RespFrame::Double(std::f64::NAN).encode();
        assert_eq!(",nan\r\n", String::from_utf8_lossy(&nan));
        let num = RespFrame::Double(1.23).encode();
        assert_eq!(",+1.23\r\n", String::from_utf8_lossy(&num));
        let neg = RespFrame::Double(std::f64::NEG_INFINITY).encode();
        assert_eq!(",-inf\r\n", String::from_utf8_lossy(&neg));
    }
}
