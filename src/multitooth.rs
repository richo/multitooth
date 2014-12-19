#![crate_name = "multitooth"]
#![feature(slicing_syntax)]

use std::os;
use std::io::stdio;
use std::io::process::Command;

fn watch_ubertooth(cmd: String, mut args: Vec<String>, ubertooth: uint) {
    let mut stdout = stdio::stdout();

    args.push("-U".to_string());
    args.push(ubertooth.to_string());

    match Command::new(cmd).args(args.as_slice()).spawn() {
        Ok(mut p) => {
            let mut buf = &mut [0u8, ..2048];
            let mut output = p.stdout.as_mut().expect("Couldn't open output stream");

            stdout.write(format!("[{}] ", ubertooth).as_bytes());

            loop {
                match output.read(buf.as_mut_slice()) {
                    Err(e) => { println!("{}", e); return },
                    Ok(s) => {
                        // Theoretically, stdout being a LineBufferedWriter *should* mean the right
                        // thing happens here and we can be delightfully naive
                        for i in range(0, s) {
                            stdout.write_u8(buf[i]);
                            if buf[i] == 0xa {
                                stdout.write(format!("[{}] ", ubertooth).as_bytes());
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
            match from_str(s.as_slice()) {
                Some (i) => i,
                None => panic!("Must supply UBERTOOTHS"),
            }
        },
        None => panic!("Must supply UBERTOOTHS"),
    };

    let args = os::args();
    let ref cmd = args[1];
    let ref args = args[2..];

    for i in range(0, ubertooths) {
        let args = args.to_vec();
        let cmd = cmd.to_string();

        spawn(move || {
            watch_ubertooth(cmd, args, i);
        });
    }
}
