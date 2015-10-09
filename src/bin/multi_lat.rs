extern crate mpi;
extern crate num;

use mpi::traits::*;
use mpi::topology::{Universe, SystemCommunicator};
use num::pow;
use mpi::collective::SystemOperation;
use std::process;

const MAX_MSG_SIZE: usize = (1<<22);
const MAX_ALIGNMENT: usize = 16384;
const BUF_SIZE: usize = MAX_MSG_SIZE + MAX_ALIGNMENT;
const LARGE_MSG_SIZE: isize = 8192 - 1;
const LOOP_SMALL: isize = 10000;
const LOOP_LARGE: isize = 1000;
const SKIP_SMALL: isize = 100;
const SKIP_LARGE: isize = 10;

fn main() {
  let universe = mpi::initialize().unwrap();
  let world = universe.world();
  let size = world.size();
  let rank = world.rank();
  let pairs = size/2;

  if size < 2 {
    println!("ERROR: This test requires at least 2 processes");
    process::exit(1);
  }

  if rank == 0 {
    println!("# RUST OSU MPI Multi Latency Test v1.0");
    println!("# Size               Latency (us)");
  }

  world.barrier();
  multi_latency(rank, pairs, &universe, &world);
  world.barrier();
}

fn multi_latency(rank: i32, pairs: i32,
                 universe: &Universe, world: &SystemCommunicator) {
  let s_buf = vec![0;BUF_SIZE];
  let root_process = world.process_at_rank(0);

  for size in (0..1).chain((0..23).map(|x| pow(2, x))) {
    let mut t_start = 0.0f64;

    world.barrier();

    let (range, extra) = match size {
      x @ _ if x > LARGE_MSG_SIZE => (LOOP_LARGE, SKIP_LARGE),
      _ => (LOOP_SMALL, SKIP_SMALL)
    };

    match rank {
      _ if rank < pairs => {
        let peer_rank = rank + pairs;

        for iter in (0 .. range + extra) {
          if iter == extra {
            t_start = universe.get_time();
            world.barrier();
          }

          world.process_at_rank(peer_rank).send(&s_buf[0.. size as usize]);
          // XXX: This will probabaly hurt performance in comarison to C.
          let (_, _) = world.receive_vec::<u32>();
        }
      },
      _ => {
        let peer_rank = rank - pairs;

        for iter in (0 .. range + extra) {
          if iter == extra {
            t_start = universe.get_time();
            world.barrier();
          }

          // XXX: This will probabaly hurt performance in comarison to C.
          let (_, _) = world.receive_vec::<u32>();
          world.process_at_rank(peer_rank).send(&s_buf[0.. size as usize]);
        }
      }
    }

    let t_end = universe.get_time();
    let latency = ((t_end - t_start) * 1.0e6) / (2.0 * range as f64);

    let mut latency_sum = 0.0f64;
    root_process.reduce_into(&latency, Some(&mut latency_sum), SystemOperation::sum());

    if world.rank() == 0 {
      let avg_latency: f64 = latency_sum / (pairs * 2) as f64;
      println!("{:<28} {1:.2}", size, avg_latency);
    }
  }
}
