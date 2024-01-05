fn main() {
    println!("Hello, world!");
}
use chrono::Local;
use rolling_file::*;
use std::env::current_dir;
use tracing::{info, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::reload;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%F %T%.3f"))
    }
}

pub fn init_log() {
    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY) // rotate log files once every minute
        .filename_prefix("x-collect")
        .filename_suffix("log")
        .max_log_files(14)
        .build("/home/xbrother/code/battery_test")
        .expect("failed to initialize rolling file appender");

    let format = tracing_subscriber::fmt::format()
        .with_target(true)
        .with_timer(LocalTimer)
        .with_line_number(true);
    // 注册
    tracing_subscriber::fmt()
        .with_writer(appender)
        .event_format(format)
        .with_ansi(false)
        .with_max_level(Level::TRACE)
        .init();
    // .with_env_filter("axum_demo=trace")
    //how to dyn modify log level
}
