extern crate getopts;

use getopts::Options;
use std::env;

struct CmdOpts {
  print_rate: bool,
  // XXX: maybe we can remove the option and just do a min/max later?
  pairs: Option<usize>,
  num_msgs: usize,
  vary_window: bool
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let opts = get_opts();
  let cmd_opts = match parse_opts(opts, args) {
    None => std::process::exit(1),
    Some(x) => x
  };

  println!("DEBUG: got opts: print_rate {}, pairs {}, num_msgs {}, vary_window {}",
          cmd_opts.print_rate, cmd_opts.pairs.unwrap_or(0), cmd_opts.num_msgs, cmd_opts.vary_window);
}

fn get_opts() -> Options {
  let mut opts = Options::new();
  opts.optflag("r", "", "Don't print uni-directional message rate (default on)");
  opts.optopt("p", "", "Number of pairs involved (default np / 2)", "pairs");
  opts.optopt("w", "",
              "Number of messages sent before acknowledgement (64, 10)
              [cannot be used with -v]", "window");
  opts.optflag("v", "", "Vary the window size (default off)
              [cannot be used with -w]");
  opts.optflag("h", "help", "Print this help");
  opts
}

fn parse_opts(opts: Options, args: Vec<String>) -> Option<CmdOpts> {
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(e) => {
      println!("{}", e.to_string());
      return None;
    }
  };

  if matches.opt_present("w") && matches.opt_present("v")
      || matches.opt_present("h") {
    print_usage(opts);
    return None;
  };
  let print_rate = !matches.opt_present("r");
  let pairs = match matches.opt_str("p") {
    Some(val) => match val.parse::<usize>().unwrap_or(0) {
      p if p > 0 => Some(p),
      _ => None
    },
    _ => None
  };
  let num_msgs = matches.opt_str("w").unwrap_or("64".to_string()).parse::<usize>().unwrap_or(64);
  let vary_window = matches.opt_present("v");
  Some(CmdOpts {
    print_rate: print_rate,
    // We can't get the default value for pairs, unless we init mpi.
    pairs: pairs,
    num_msgs: num_msgs,
    vary_window: vary_window
  })
}

fn print_usage(opts: Options) {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}
