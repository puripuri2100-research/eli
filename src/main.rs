use anyhow::Result;
use clap::Parser;

#[derive(Clone, Parser)]
struct Arg {
  #[clap(short, long, default_value_t = 2)]
  /// jobs
  jobs: usize,
}

fn main() -> Result<()> {
  let args = Arg::parse();
  let threds = args.jobs;
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(threds)
    .enable_all()
    .build()?
    .block_on(async {
      println!("Hello world");
    });
  Ok(())
}
