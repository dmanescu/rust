// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[pkgid="foreign_lib"];
// NOTE: remove after the next snapshot
#[link(name="foreign_lib", vers="0.0")];

pub mod rustrt {
    use std::libc;

    #[link(name = "rustrt")]
    extern {
        fn rust_get_test_int() -> libc::intptr_t;
    }
}
