use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use enum_dispatch::enum_dispatch;
use time::OffsetDateTime;

use crate::logger::message::Message;

#[enum_dispatch(OutputMethod)]
pub trait Output {
    fn output(&mut self, message: &Message) -> io::Result<()>;
}

/// 输出方式
#[enum_dispatch]
pub enum OutputMethod {
    Stdout(Stdout),
    OutputFile(OutFile),
}

/// 标准输出
#[derive(Debug)]
pub struct Stdout {
    pub color: bool,
    pub output: std::io::Stdout,
}

impl Output for Stdout {
    fn output(&mut self, message: &Message) -> io::Result<()> {
        if self.color {
            message.write_color(&mut self.output)
        } else {
            message.write(&mut self.output)
        }
    }
}

impl Default for Stdout {
    fn default() -> Self {
        Self {
            color: true,
            output: std::io::stdout(),
        }
    }
}

/// 文件输出
#[derive(Debug)]
pub struct OutFile {
    pub file: File,
    pub name: String,
    pub path: PathBuf,
    pub delete: Option<i64>,
    pub created_at: OffsetDateTime,
}

impl OutFile {
    pub fn new(path: PathBuf, name: String, delete: Option<i64>) -> io::Result<Self> {
        let created_at = OffsetDateTime::now_local().unwrap();
        fs::create_dir_all(&path)?;
        let file_name = name.replace("{date}", &created_at.date().to_string());
        let file = File::options().create(true).append(true).open(path.join(&file_name))?;
        Ok(Self {
            path,
            name,
            delete,
            created_at,
            file,
        })
    }
    pub fn delete_log_file(&self, now: &OffsetDateTime) -> io::Result<()> {
        if let Some(n) = self.delete {
            for entry in fs::read_dir(&self.path)?.flatten() {
                let meta = entry.metadata()?;
                let created_at: OffsetDateTime = meta.created()?.into();
                if (*now - created_at).whole_days() > n {
                    fs::remove_file(entry.path())?
                }
            }
        };
        Ok(())
    }

    /// 更新日志文件 删除过期文件
    pub fn update_log_file(&self, now: &OffsetDateTime) -> io::Result<File> {
        if let Err(err) = self.delete_log_file(now) {
            eprintln!("日志删除失败: {err}")
        }
        let name = self.name.replace("{date}", &now.date().to_string());
        File::options().create(true).append(true).open(self.path.join(name))
    }
}

impl Default for OutFile {
    fn default() -> Self {
        Self::new("logs".into(), "{date}.log".into(), None).unwrap()
    }
}

impl Output for OutFile {
    fn output(&mut self, message: &Message) -> io::Result<()> {
        if message.begin.date() != self.created_at.date() {
            self.file = self.update_log_file(&message.begin)?;
        }
        message.write(&mut self.file)
    }
}
