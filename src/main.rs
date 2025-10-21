use anyhow::Result;
use clap::Parser;

pub mod eli;
pub mod law;

#[derive(Clone, Parser)]
#[clap(author, version, about)]
struct Arg {
  #[clap(short, long, default_value_t = 2)]
  /// jobs
  jobs: usize,
  #[command(flatten)]
  verbosity: clap_verbosity_flag::Verbosity,
  /// e-govデータが入ったフォルダのパス
  #[clap(short, long)]
  egov_folder: String,
  /// 出力先のフォルダ
  #[clap(short, long)]
  output_folder: String,
}

async fn run(args: Arg) -> Result<()> {
  let subscriber = tracing_subscriber::fmt()
    .with_max_level(args.verbosity)
    .finish();
  tracing::subscriber::set_global_default(subscriber)?;

  Ok(())
}

fn main() -> Result<()> {
  let args = Arg::parse();
  let threds = args.jobs;
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(threds)
    .enable_all()
    .build()?
    .block_on(async { run(args).await })?;
  Ok(())
}
