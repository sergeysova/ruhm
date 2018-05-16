extern package "foo";
extern package bar;

use ::f;
use ::k::b::a;
use super::f;
use super::super::foo;
use foo::{d};
use foo::{bar as baz};

struct Foo {}
