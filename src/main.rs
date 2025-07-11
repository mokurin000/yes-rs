use rustix::pipe::{IoSliceRaw, SpliceFlags, fcntl_setpipe_size, vmsplice};
use std::{io::stdout, mem::transmute, os::fd::AsFd as _};

const BATCH: usize = 8;
const TIMES: usize = 1024;
const CONTENT: [u8; 4] = [b'y', b'e', b's', b'\n'];

fn main() {
    fcntl_setpipe_size(stdout(), 1048576).expect("stdout must be pipefd");

    let buf: [u8; TIMES * CONTENT.len()] = unsafe { transmute([CONTENT; TIMES]) };
    let bufs: Vec<_> = (0..BATCH).map(|_| IoSliceRaw::from_slice(&buf)).collect();

    let stdout = stdout().lock();
    let fd = stdout.as_fd();
    loop {
        unsafe {
            _ = vmsplice(fd, &bufs, SpliceFlags::empty());
        };
    }
}
