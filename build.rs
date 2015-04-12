// ++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.#
use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

fn transpile(program: &mut Read, w: &mut Write) -> io::Result<()> {
    try!(w.write_all(b"#[allow(dead_code, unused_imports, unused_mut, unused_variables)]
    fn main() {
        use std::io::{Read, Write};
        use std::num::Wrapping;
        let mut memory = [Wrapping::<u8>(0); 65536];
        let mut ptr = Wrapping::<u16>(0);
        let mut stdin = ::std::io::stdin();
        let mut stdout = ::std::io::stdout();
    "));
    for entry in program.bytes() {
        let b = try!(entry);
        let instruction: &[u8] = match b {
            b'>' => b"ptr = ptr + Wrapping(1);",
            b'<' => b"ptr = ptr - Wrapping(1);",
            b'+' => b"{ let v = &mut memory[ptr.0 as usize]; *v = *v + Wrapping(1); }",
            b'-' => b"{ let v = &mut memory[ptr.0 as usize]; *v = *v - Wrapping(1); }",
            b'.' => b"stdout.write_all(&[memory[ptr.0 as usize].0]).unwrap();",
            b',' => b"{ let mut b = [0; 1]; stdin.read(&mut b).unwrap(); memory[ptr.0 as usize].0 = b[0]; }",
            b'[' => b"while memory[ptr.0 as usize].0 != 0 {",
            b']' => b"}",
            b'#' => { break; }
            _ => { continue; }
        };
        try!(w.write_all(instruction));
        try!(w.write_all(b"\n"));
    }
    try!(w.write_all(b"}\n"));
    Ok(())
}

fn main() {
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_path = Path::new(&src_dir).join("build.rs");
    let dest_path = Path::new(&out_dir).join("t.rs");
    let mut p = File::open(&src_path).unwrap();
    let mut f = File::create(&dest_path).unwrap();
    transpile(&mut p, &mut f).unwrap();
    let mut f = File::create(&Path::new(&src_dir).join("src").join("bin.rs")).unwrap();
    f.write_all(br#"include!(concat!(env!("OUT_DIR"), "/t.rs"));"#).unwrap();
}
