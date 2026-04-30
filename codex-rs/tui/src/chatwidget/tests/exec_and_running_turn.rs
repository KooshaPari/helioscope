use super::*;
use pretty_assertions::assert_eq;

macro_rules! assert_exec_running_turn_snapshot {
    ($($tt:tt)*) => {
        insta::with_settings!({ snapshot_path => "../snapshots" }, {
            assert_snapshot!($($tt)*);
        });
    };
}

include!("exec_and_running_turn/status_and_queue.rs");
include!("exec_and_running_turn/exec_history.rs");
include!("exec_and_running_turn/unified_wait.rs");
