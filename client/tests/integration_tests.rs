use nix::fcntl::AtFlags;
use nix::libc::c_int;
use nix::sys::stat::fstatat;
use proptest::prelude::*;
use proptest::prelude::{BoxedStrategy, Just, Strategy};
use proptest::strategy::Union;
use proptest_derive::Arbitrary;
use protocol::{ exec_docker, Syscall};
use protocol::{syscall, FileStatDef};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::os::fd::AsRawFd;


fn flag_strategy() -> BoxedStrategy<c_int> {
    prop_oneof![
        10 => Just(AtFlags::empty().bits()),
        1 => Just(AtFlags::AT_SYMLINK_FOLLOW.bits()),
        1 => Just(AtFlags::AT_SYMLINK_NOFOLLOW.bits()),
        1 => Just(AtFlags::AT_NO_AUTOMOUNT.bits()),
        1 => Just(AtFlags::AT_EMPTY_PATH.bits()),
        1 => Just(AtFlags::AT_EACCESS.bits())
    ]
    .boxed()
}

fn file_strategy() -> impl Strategy<Value = String> {
    Union::new(include_str!("./files.txt").lines())
}

fn dir_strategy() -> impl Strategy<Value = Option<String>> {
    Union::new(
        include_str!("./files.txt")
            .lines()
            .map(|line| Just(Some(line.to_string()))),
    )
    .prop_union(Union::new_weighted(vec![(10, Just(None))]))
}

syscall!(Fstatat {
        #[proptest(strategy = "file_strategy()")]
        path: String,
        #[proptest(strategy = "dir_strategy()")]
        dir: Option<String>,
        #[proptest(strategy = "flag_strategy()")]
        flags: i32
    },
    self {
        let fd = self.dir.as_ref().and_then(|dir|File::open(dir).ok()).map(|file|file.as_raw_fd());
        FileStatDef::from(fstatat(fd, self.path.as_str(), AtFlags::from_bits_retain(self.flags))?)
    },
    test_fstatat(fstatat, (left,right): FileStatDef) {
        prop_assert_eq!(left.st_uid, right.st_uid);
        prop_assert_eq!(left.st_gid, right.st_gid);
        Ok::<(),TestCaseError>(())
});
