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
  // TODO: replace unwrap with match
  let cmd_opts = parse_opts(opts, args).unwrap();
  println!("DEBUG: got opts: print_rate {}, pairs {}, num_msgs {}, vary_window {}",
          cmd_opts.print_rate, cmd_opts.pairs.unwrap_or(0), cmd_opts.num_msgs, cmd_opts.vary_window);
}

fn get_opts() -> Options {
  let mut opts = Options::new();
  opts.optopt("r", "", "Print uni-directional message rate (default 1)", "<0,1>");
  opts.optopt("p", "", "Number of pairs involved (default np / 2)", "pairs");
  opts.optopt("w", "",
              "Number of messages sent before acknowledgement (64, 10)
              [cannot be used with -v]", "window");
  opts.optopt("v", "", "Vary the window size (default no)
              [cannot be used with -w]", "<yes,no>");
  opts.optflag("h", "help", "Print this help");
  opts
}

fn parse_opts(opts: Options, args: Vec<String>) -> Option<CmdOpts> {
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(e) => { panic!(e.to_string()) }
  };

  if matches.opt_present("w") && matches.opt_present("v")
      || matches.opt_present("h") {
    print_usage(opts);
    return None;
  };
  let print_rate = match matches.opt_str("r") {
    Some(val) => match val.as_ref() {
      "1" => true,
      _ => false
    },
    _ => false
  };
  let pairs: Option<usize> = match matches.opt_str("p") {
    Some(val) => match val.parse::<usize>().unwrap_or(0) {
      p if p > 0 => Some(p),
      _ => None
    },
    _ => None
  };
  let num_msgs = matches.opt_str("w").unwrap_or("64".to_string()).parse::<usize>().unwrap_or(64);
  let vary_window = match matches.opt_str("v") {
    Some(val) => match val.as_ref() {
      "no" => false,
      "yes" => true,
      _ => false
    },
    _ => false
  };
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
