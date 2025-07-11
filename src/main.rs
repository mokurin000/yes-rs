use rustix::pipe::{IoSliceRaw, SpliceFlags, fcntl_setpipe_size, vmsplice};
use std::{io::stdout, os::fd::AsFd as _};

const BATCH: usize = 8;

fn main() {
    fcntl_setpipe_size(stdout(), 1048576).expect("failed to set pipe size");

    let stdout = stdout().lock();
    let fd = stdout.as_fd();
    let buf = [b'y'; 4096];
    let bufs: Vec<_> = (0..BATCH).map(|_| IoSliceRaw::from_slice(&buf)).collect();
    loop {
        unsafe {
            _ = vmsplice(fd, &bufs, SpliceFlags::empty());
        };
    }
}
