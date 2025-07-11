use std::io::{BufWriter, Write, stdout};

fn main() {
    let mut stdout = BufWriter::new(stdout().lock());
    loop {
        _ = stdout.write(&[b'y']);
    }
}
