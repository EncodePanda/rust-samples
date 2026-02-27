use std::io::{self, BufRead, Cursor};

/// Interesting gotcha explored during Berlin Rust meetup on 26th Feb 2026.
fn example1() -> io::Result<()> {
    let mut rd = io::BufReader::new(Cursor::new(b"hello"));

    let buf: &[u8] = rd.fill_buf()?;
    //          ^       ^
    //          | immutable reference to bytes
    //                  | mutable reference to self
    //
    // fill_buf signature:
    // fn fill_buf(&mut self) -> Result<&[u8]>

    let len: usize = buf.len();
    rd.consume(len);

    // What will not compile:
    // rd.consume(buf.len());
    //
    // fn consume(&mut self, amt: usize)
    //            ^ mutable reference to self

    Ok(())
}

fn take_mut1(v: &mut Vec<u8>) -> &Vec<u8> {
    v.push(42); // answer to everything
    v
}

fn take_mut2(v: &mut Vec<u8>, len: usize) {
    println!("Length is {}", len);
    v.push(44); // kaliber
}

/// Same issue, simpler example
fn example2() {
    let mut v = vec![1, 2, 3];
    let view = take_mut1(&mut v);

    let len = view.len();
    take_mut2(&mut v, len);
    // this does not compile
    // take_mut2(&mut v, view.len());
}
