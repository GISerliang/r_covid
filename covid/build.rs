//! #   rCovid
//!                         build.rs
//!                         -------------------------------------
//!     begin               2022/05/30
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

extern crate embed_resource;

fn main() {
    #[cfg(windows)]
    embed_resource::compile("../resources/windows/res.rc");
}