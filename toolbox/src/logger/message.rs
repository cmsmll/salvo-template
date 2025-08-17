use std::io::{self, Write};

use time::{OffsetDateTime, format_description::BorrowedFormatItem, macros::format_description};

/// 重置
const RESET: &str = "\x1b[0m";
/// 红色字体
const RED: &str = "\x1b[31m";
/// 绿色字体
const GREEN: &str = "\x1b[32m";
/// 黄色字体
const YELLOW: &str = "\x1b[33m";
/// 蓝色字体
const BLUE: &str = "\x1b[34m";

/// 红色背景
const BG_RED: &str = "\x1b[41m";
/// 绿色背景
const BG_GREEN: &str = "\x1b[42m";
/// 黄色背景
const BG_YELLOW: &str = "\x1b[43m";
/// 蓝色背景
const BG_BLUE: &str = "\x1b[44m";
/// 紫色背景
const BG_PURPLE: &str = "\x1b[45m";

const FMT: &[BorrowedFormatItem<'_>] =
    format_description!("[year repr:last_two]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:4]");

pub struct Message {
    pub begin: OffsetDateTime,
    pub elapsed: time::Duration,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub ip: String,
    pub other: String,
}

impl Message {
    const fn status_color(&self) -> &'static str {
        match self.status {
            0..200 => BG_BLUE,
            200..300 => BG_GREEN,
            300..400 => BG_YELLOW,
            400..600 => BG_RED,
            _ => BG_PURPLE,
        }
    }

    const fn elapsed_color(&self) -> &'static str {
        match (self.elapsed).whole_milliseconds() {
            0..10 => GREEN,
            10..20 => BLUE,
            20..30 => YELLOW,
            _ => RED,
        }
    }

    fn method_color(&self) -> &'static str {
        match self.method.as_str() {
            "GET" => BG_GREEN,
            "POST" => BG_BLUE,
            "PATCH" | "PUT" => BG_YELLOW,
            "DELETE" => BG_RED,
            _ => BG_PURPLE,
        }
    }

    fn format(&self) -> String {
        match self.begin.format(FMT) {
            Ok(s) => s,
            Err(_) => format!("{}", self.begin),
        }
    }

    pub fn write(&self, out: &mut impl Write) -> io::Result<()> {
        write!(out, "[{}] ", self.format())?;
        write!(out, "SALVO │ ")?;
        write!(out, "{} │ ", self.status)?;
        write!(out, "{:>3.0} │ ", self.elapsed)?;
        write!(out, "{:<15} │ ", self.ip)?;
        write!(out, "{:>6} │ ", self.method)?;
        write!(out, "{} ", self.path)?;
        writeln!(out, "{}", self.other)
    }

    pub fn write_color(&self, out: &mut impl Write) -> io::Result<()> {
        write!(out, "[{}] ", self.format())?; // 时间
        write!(out, "{YELLOW}SALVO{RESET} │ ")?; // LOGO
        write!(out, "{} {} {RESET} │ ", self.status_color(), self.status)?; // 状态吗
        write!(out, "{}{:>3.0}{RESET} │ ", self.elapsed_color(), self.elapsed)?; // 耗时
        write!(out, "{YELLOW}{:<15}{RESET} │ ", self.ip)?; // ip地址
        write!(out, "{} {:>6} {RESET} ", self.method_color(), self.method)?; // 访问方式
        write!(out, "{} ", self.path)?; // 访问路径
        writeln!(out, "{RED}{}{RESET}", self.other) // 其他信息
    }
}
