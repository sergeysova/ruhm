extern package "foo";
extern package bar;

use ::f;
use ::k::b::a;
use super::f;
use super::super::foo;
use foo::{d};
use foo::{bar as baz};

struct Foo {}


foo::bar()
foo::bar::baz()
::bar::baz()
super::foo()

fn main(argv: Array<String>, argc: i32) -> i32 {
}

fn example(foo, bar) {
}

fn foo(bar: impl Bar) -> impl Baz {
}
