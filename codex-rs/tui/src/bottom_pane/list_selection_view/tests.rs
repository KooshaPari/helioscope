mod behavior;
mod support;

use super::*;
use insta::assert_snapshot;
use pretty_assertions::assert_eq;
use support::*;

include!("tests/render.rs");
include!("tests/snapshots.rs");
