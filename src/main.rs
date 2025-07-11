use std::{io::stdout, os::fd::AsFd as _};

use rustix::{io::write, pipe::fcntl_setpipe_size};

fn main() {
    fcntl_setpipe_size(stdout(), 1048576).expect("failed to set pipe size");

    let stdout = stdout().lock();
    let fd = stdout.as_fd();
    loop {
        _ = write(fd, &[b'y'; 4096]);
    }
}
