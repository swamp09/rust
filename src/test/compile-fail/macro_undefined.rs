// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test macro_undefined issue

mod m {
    #[macro_export]
    macro_rules! kl {
        () => ()
    }
}

fn main() {
    k!();
    //~^ ERROR cannot find macro `k!` in this scope
    //~^^ HELP did you mean `kl!`?
    kl!();
    //~^ ERROR cannot find macro `kl!` in this scope
    //~^^ HELP have you added the `#[macro_use]` on the module/import?
}
