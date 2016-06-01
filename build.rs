/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate cmake;

fn main() {
    let dst = cmake::Config::new(".").build_target("ALL_BUILD").build();
    println!("cargo:rust-link-search=native={}", dst.display());
    println!("cargo:rust-link-lib=static=angle");
}
