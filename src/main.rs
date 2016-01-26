extern crate getopts;

use std::env;
use std::process;
use std::io;
use std::io::{Write, Read};
use std::thread;

use getopts::Options;

fn watch_ubertooth(cmd: String, mut args: Vec<String>, ubertooth: u8, opts: Opts) {
    let mut stdout = io::stdout();

    args.push("-U".to_string());
    args.push(ubertooth.to_string());

    if opts.advertising {
        args.push("-A".to_string());
        args.push((ubertooth % 3 + 37).to_string());
    }

    if opts.debug {
        println!("[{}] {} -- {:?}", ubertooth, cmd, args);
        return;
    }

    match process::Command::new(cmd).args(&args[..]).spawn() {
        Ok(mut p) => {
            let mut buf = &mut [0u8; 2048];
            let mut output = p.stdout.as_mut().expect("Couldn't open output stream");

            let _ = stdout.write(format!("[{}] ", ubertooth).as_bytes());

            loop {
                match output.read(buf) {
                    Err(e) => {
                        // TODO: I'm not really sure where EOF ended up in the stdlib
                        // if e.kind() != ErrorKind::EndOfFile {
                        panic!(e);
                        // }
                    }
                    Ok(s) => {
                        // Theoretically, stdout being a LineBufferedWriter *should* mean the right
                        // thing happens here and we can be delightfully naive
                        for i in 0..s {
                            let _ = stdout.write(&[buf[i]]);
                            if buf[i] == 0xa {
                                let _ = stdout.write(format!("[{}] ", ubertooth).as_bytes());
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            panic!(e);
        }
    }
}

fn get_args() -> (Vec<String>, Vec<String>) {
    let args: Vec<_> = env::args().collect();

    let mut thru = false;
    let mut parseargs: Vec<String> = vec![];
    let mut thruargs: Vec<String> = vec![];

    for a in args.iter() {
        if a == "--" {
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
    ubertooths: u8,
    advertising: bool,
    debug: bool,
}

fn parse_opts(args: Vec<String>, opts: &Options) -> Option<Opts> {
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f),
    };

    if matches.opt_present("h") {
        return None;
    }

    let advertising = matches.opt_present("A");
    let debug = matches.opt_present("d");

    let ubertooths: u8 = match matches.opt_str("n") {
        Some(n) => {
            match n.parse() {
                Ok(i) => i,
                Err(_) => return None,
            }
        }
        None => return None,
    };

    return Some(Opts {
        ubertooths: ubertooths,
        advertising: advertising,
        debug: debug,
    });
}

fn print_usage(opts: &Options, msg: Option<&str>) {
    let ref program = env::args().next().unwrap();
    let brief = format!("Usage: {} [options] -- ubertooth-<tool> [uberooth options]",
                        program);
    print!("{}", opts.usage(&brief));

    if let Some(s) = msg {
        println!("{}", s);
    }
}

fn main() {
    let mut opts = Options::new();
    opts.optopt("n", "", "number of ubertooths", "UBERTOOTHS");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("A", "advertising", "add the advertising address flag");
    opts.optflag("d",
                 "debug",
                 "print invocations instead of running children");

    let (parseargs, thruargs) = get_args();

    let options = match parse_opts(parseargs, &opts) {
        Some(o) => o,
        None => {
            print_usage(&opts, Some("-n is required"));
            return;
        }
    };

    if thruargs.len() == 0 {
        print_usage(&opts, Some("Must supply an ubertooth cmd"));
        return;
    }

    let ref cmd = thruargs[0];
    let ref args = thruargs[1..];

    (0..options.ubertooths)
        .map(|i| -> thread::JoinHandle<_> {
            let args = args.to_vec();
            let cmd = cmd.to_string();
            let options = options.clone();

            thread::spawn(move || {
                watch_ubertooth(cmd, args, i, options);
            })
        })
        .collect::<Vec<_>>();
}
