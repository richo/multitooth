#![crate_name = "multitooth"]
#![feature(slicing_syntax)]

extern crate getopts;

use std::os;
use std::io::stdio;
use std::io::{IoErrorKind,LineBufferedWriter};
use std::io::process::Command;
use std::thread::{Thread,JoinGuard};

use getopts::{optopt,optflag,getopts,OptGroup,usage};

fn watch_ubertooth(cmd: String, mut args: Vec<String>, ubertooth: uint, opts: Opts) {
    let mut stdout: LineBufferedWriter<_> = stdio::stdout();

    args.push("-U".to_string());
    args.push(ubertooth.to_string());

    if opts.advertising {
        args.push("-A".to_string());
        args.push((ubertooth % 3 + 37).to_string());
    }

    if opts.debug {
        println!("[{}] {} -- {}", ubertooth, cmd, args);
        return;
    }

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

fn get_args() -> (Vec<String>, Vec<String>) {
    let args = os::args();

    let mut thru = false;
    let mut parseargs: Vec<String> = vec![];
    let mut thruargs: Vec<String> = vec![];

    for a in args.iter() {
        if a.as_slice() == "--" {
            thru = true;
        } else {
            if thru {
                thruargs.push(a.clone());
            } else {
                parseargs.push(a.clone());
            }
        }
    }
    return (parseargs, thruargs);
}

#[derive(Clone)]
struct Opts {
    ubertooths: uint,
    advertising: bool,
    debug: bool,
}

fn parse_opts(args: Vec<String>, opts: &[OptGroup]) -> Option<Opts> {
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => panic!(f),
    };

    if matches.opt_present("h") {
        return None;
    }

    let advertising = matches.opt_present("A");
    let debug = matches.opt_present("d");

    let ubertooths: uint = match matches.opt_str("n") {
        Some(n) => match n.parse() {
            Some (i) => i,
            None => return None,
        },
        None => return None,
    };

    return Some(Opts {
        ubertooths: ubertooths,
        advertising: advertising,
        debug: debug,
    })
}

fn print_usage(opts: &[OptGroup], msg: Option<&str>) {
    let ref program = os::args()[0];
    let brief = format!("Usage: {} [options] -- ubertooth-<tool> [uberooth options]", program);
    print!("{}", usage(brief.as_slice(), opts));

    if let Some(s) = msg {
        println!("{}", s);
    }
}

fn main() {
    let opts = &[
        optopt("n", "", "number of ubertooths", "UBERTOOTHS"),
        optflag("h", "help", "print this help menu"),
        optflag("A", "advertising", "add the advertising address flag"),
        optflag("d", "debug", "print invocations instead of running children"),
    ];

    let (parseargs, thruargs) = get_args();

    let options = match parse_opts(parseargs, opts) {
        Some(o) => o,
        None => {
            print_usage(opts, Some("-n is required"));
            return;
        }
    };

    if thruargs.len() == 0 {
        print_usage(opts, Some("Must supply an ubertooth cmd"));
        return;
    }

    let ref cmd = thruargs[0];
    let ref args = thruargs[1..];

    range(0, options.ubertooths).map(|i| -> JoinGuard<_> {
        let args = args.to_vec();
        let cmd = cmd.to_string();
        let options = options.clone();

        Thread::spawn(move || {
            watch_ubertooth(cmd, args, i, options);
        })
    }).collect::<Vec<_>>();
}
