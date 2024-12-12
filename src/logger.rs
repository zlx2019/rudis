#![allow(unused_variables)]
#![allow(dead_code)]

use time::{macros::format_description, UtcOffset};
use tracing_subscriber::fmt::time::OffsetTime;

/// Logger 初始化
pub fn logger_init(){
    // 定义日志时间格式、时区.
    let local_offset = UtcOffset::from_hms(8, 0, 0).unwrap();
    let timer = OffsetTime::new(
        local_offset,
        format_description!("[year]/[month]/[day] [hour]:[minute]:[second]"),
    );
    tracing_subscriber::fmt()
        // 显示日志级别
        .with_level(true) 
        // 显示行号
        .with_line_number(true)
        // 显示文件名 
        .with_file(true) 
        // 显示线程ID
        .with_thread_ids(false)
        // 显示目标 
        .with_target(false) 
        // 日志时间格式
        .with_timer(timer)
        .init();
}