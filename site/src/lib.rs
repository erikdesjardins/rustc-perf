// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate antidote;
extern crate chrono;
extern crate collector;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate rust_sysroot;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rmp_serde;
extern crate url;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Json(::serde_json::Error);
            Chrono(::chrono::ParseError);
        }

        errors {
            CommandFailed(command: String) {
                description("command failed; exit code non-zero")
                display("command failed, non-zero exit code: {}", command)
            }
        }
    }
}

mod git;

pub mod api;
pub mod load;
pub mod util;
pub mod server;
