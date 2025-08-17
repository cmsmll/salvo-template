use std::{fmt::Write, sync::Arc, thread};

use crossbeam_channel::Sender;
use percent_encoding::percent_decode;
use salvo::{Depot, FlowCtrl, Handler, Request, Response, async_trait, conn::SocketAddr, http::header::LOCATION};
use time::OffsetDateTime;

use crate::logger::{
    message::Message,
    output::{OutFile, Output, OutputMethod, Stdout},
};

#[derive(Clone)]
pub struct Logger {
    sender: Sender<Message>,
}

impl Logger {
    pub fn new(mut writers: Vec<OutputMethod>) -> Self {
        let (sender, rx) = crossbeam_channel::unbounded::<Message>();

        thread::spawn(move || {
            for msg in rx {
                for writer in writers.iter_mut() {
                    if let Err(err) = writer.output(&msg) {
                        eprintln!("输出日志失败: {err}");
                    }
                }
            }
        });

        Self { sender }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(vec![
            OutputMethod::OutputFile(OutFile::default()),
            OutputMethod::Stdout(Stdout::default()),
        ])
    }
}

#[async_trait]
impl Handler for Logger {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let begin = OffsetDateTime::now_local().unwrap();
        let method = req.method().to_string();
        let ip = match req.remote_addr() {
            SocketAddr::IPv4(ip) => ip.ip().to_string(),
            SocketAddr::IPv6(ip) => ip.ip().to_string(),
            SocketAddr::Unix(ip) => match ip.as_pathname() {
                Some(path) => path.display().to_string(),
                None => "Unknow".to_string(),
            },
            _ => "Unknow".to_string(),
        };
        let mut path = percent_decode(req.uri().path().as_bytes())
            .decode_utf8_lossy()
            .to_string();

        ctrl.call_next(req, depot, res).await;

        let mut other = String::new();
        if let Ok(v) = depot.get::<Arc<str>>("error") {
            write!(&mut other, " Error({v})").ok();
        }
        
        if let Ok(v) = depot.get::<Arc<str>>("other") {
            write!(&mut other, " {v}").ok();
        }

        let status = res.status_code.unwrap_or_default().as_u16();
        // 是否重定向
        if let Some(p) = res.headers().get(LOCATION) {
            path.push_str(" -> ");
            path.push_str(&percent_decode(p.as_bytes()).decode_utf8_lossy());
        }
        let elapsed = OffsetDateTime::now_local().unwrap() - begin;

        let msg = Message {
            begin,
            elapsed,
            status,
            ip,
            method,
            path,
            other,
        };

        if let Err(err) = self.sender.send(msg) {
            eprintln!("Send 日志时出现错误 {err}")
        }
    }
}
