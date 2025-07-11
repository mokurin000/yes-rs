#![feature(maybe_uninit_slice)]

use rustix::pipe::{IoSliceRaw, SpliceFlags, fcntl_setpipe_size, vmsplice};
use std::{io::stdout, mem::MaybeUninit, os::fd::AsFd as _};

const BATCH: usize = 8;

fn main() {
    fcntl_setpipe_size(stdout(), 1048576).expect("failed to set pipe size");

    let stdout = stdout().lock();
    let fd = stdout.as_fd();
    let buf = [b'y'; 4096];
    let mut bufs = [const { MaybeUninit::uninit() }; BATCH];
    for i in 0..BATCH {
        bufs[i].write(IoSliceRaw::from_slice(&buf));
    }
    let bufs = unsafe { bufs.assume_init_ref() };
    loop {
        unsafe {
            _ = vmsplice(fd, &bufs, SpliceFlags::empty());
        };
    }
}
