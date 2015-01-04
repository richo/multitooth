#![crate_name = "multitooth"]
#![feature(slicing_syntax)]

use std::os;
use std::io::stdio;
use std::io::{IoErrorKind,LineBufferedWriter};
use std::io::process::Command;
use std::thread::{Thread,JoinGuard};

fn watch_ubertooth(cmd: String, mut args: Vec<String>, ubertooth: uint) {
    let mut stdout: LineBufferedWriter<_> = stdio::stdout();

    args.push("-U".to_string());
    args.push(ubertooth.to_string());

    match Command::new(cmd).args(args.as_slice()).spawn() {
        Ok(mut p) => {
            let mut buf = &mut [0u8; 2048];
            let mut output = p.stdout.as_mut().expect("Couldn't open output stream");

            let _ = stdout.write(format!("[{}] ", ubertooth).as_bytes());

            loop {
                match output.read(buf.as_mut_slice()) {
                    Err(e) => {
                        if e.kind != IoErrorKind::EndOfFile {
                            panic!(e);
                        }
                        return
                    },
                    Ok(s) => {
                        // Theoretically, stdout being a LineBufferedWriter *should* mean the right
                        // thing happens here and we can be delightfully naive
                        for i in range(0, s) {
                            let _ = stdout.write_u8(buf[i]);
                            if buf[i] == 0xa {
                                let _ = stdout.write(format!("[{}] ", ubertooth).as_bytes());
                            }
                        }
                    }
                }
            }
        },
        Err(e) => {
            panic!(e);
        }
    }
}

fn main() {
    // Jank, fix this later
    let ubertooths: uint = match os::getenv("UBERTOOTHS") {
        Some(s) => {
            match s.parse() {
                Some (i) => i,
                None => panic!("Must supply UBERTOOTHS"),
            }
        },
        None => panic!("Must supply UBERTOOTHS"),
    };

    let args = os::args();

    if args.len() == 1 {
        panic!("TODO usage()");
    }

    let ref cmd = args[1];
    let ref args = args[2..];

    range(0, ubertooths).map(|i| -> JoinGuard<_> {
        let args = args.to_vec();
        let cmd = cmd.to_string();

        Thread::spawn(move || {
            watch_ubertooth(cmd, args, i);
        })
    }).collect::<Vec<_>>();
}
