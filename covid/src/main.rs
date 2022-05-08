//! #   rCovid
//!                         main.rs
//!                         -------------------------------------
//!     begin               2021/12/30
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// Forbid warnings in release builds:
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

use chrono::Local;
use std::io;
use tracing::Level;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

use rcovid_app;
use rcovid_core;

struct RcdLocalTimer;

impl FormatTime for RcdLocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}

// When compiling natively:
#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("./log", "r_covid.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(RcdLocalTimer);

    // Init tracing and set format
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .with_writer(io::stdout) // write to stdout
            .with_ansi(false) // close ansi color out
            .event_format(format)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::WARN)
            .with_writer(non_blocking) // write to file, override the stdout
            .with_ansi(false) // close ansi color out
            .event_format(format)
            .init();
    }

    let scope = tracing::span!(tracing::Level::DEBUG, "rCovid");
    let _enter = scope.enter();
    tracing::debug!("rCovid starting...");

    let icon = image::open("./assets/logo.png")
        .expect("LOGO文件打开失败")
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    let options = eframe::NativeOptions {
        maximized: true,
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        ..Default::default()
    };
    eframe::run_native(
        rcovid_core::APP_KEY,
        options,
        Box::new(|cc| Box::new(rcovid_app::rcdapplication::RcdApplication::new(cc))),
    );
}
