// check that the `for<T> T: From<!>` impl is reserved

// revisions: current next
//[next] compile-flags: -Ztrait-solver=next-coherence

#![feature(never_type)]

pub struct MyFoo;
pub trait MyTrait {}

impl MyTrait for MyFoo {}
// This will conflict with the first impl if we impl `for<T> T: From<!>`.
impl<T> MyTrait for T where T: From<!> {} //~ ERROR conflicting implementation

fn main() {}
