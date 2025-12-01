use anyhow::Result;
use clap::Parser;
use gen_eli::law::{egov_xml_parse, parse_ref};
use japanese_law_id::Date;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tracing::{info, trace};

async fn get_all_folder_names(path: &str) -> Result<Vec<String>> {
  let mut dirs = tokio_stream::wrappers::ReadDirStream::new(fs::read_dir(path).await?);
  let mut v = Vec::new();
  while let Some(Ok(dir_entry)) = dirs.next().await {
    if dir_entry.file_type().await?.is_dir() {
      let s = dir_entry
        .file_name()
        .to_str()
        .unwrap_or_default()
        .to_string();
      v.push(s)
    }
  }
  Ok(v)
}

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
  /// 結果を出力するフォルダのパス
  #[clap(short, long)]
  output_folder: String,
}

async fn run(args: Arg) -> Result<()> {
  let subscriber = tracing_subscriber::fmt()
    .with_max_level(args.verbosity)
    .finish();
  tracing::subscriber::set_global_default(subscriber)?;

  info!("start");

  fs::create_dir_all(&args.output_folder).await?;

  trace!("[START] get all folder name");
  let folders = get_all_folder_names(&args.egov_folder).await?;
  trace!("[END] get all folder name");

  // 法令名が法令番号だけだった時に備える
  let re_fix_name = Regex::new("^(?<name>.+号)（.+）$").unwrap();

  let mut law_name_list = HashMap::new();
  if let Ok(all_law_list_text) =
    fs::read_to_string(format!("{}/all_law_list.csv", args.egov_folder)).await
  {
    let mut all_law_list_lines = all_law_list_text.lines();
    all_law_list_lines.next();
    for law in all_law_list_lines {
      if !law.trim().is_empty() {
        let info = law.split(',').collect::<Vec<_>>();
        let law_id = info[11].to_string();
        let mut v = Vec::new();
        // 法令IDテキスト
        let law_name0 = info[1];
        if !law_name0.is_empty() {
          v.push(law_name0.to_string());
        }
        // 法令名
        let law_name1 = info[2];
        if !law_name1.is_empty() {
          if let Some(caps) = re_fix_name.captures(law_name1) {
            let num_text = &caps["name"];
            v.push(num_text.to_string());
          }
          v.push(law_name1.to_string());
        }
        // 旧法令名
        let law_name2 = info[4];

        if !law_name2.is_empty() {
          if let Some(caps) = re_fix_name.captures(law_name2) {
            let num_text = &caps["name"];
            v.push(num_text.to_string());
          }
          v.push(law_name2.to_string());
        }
        law_name_list.insert(law_id.clone(), v);
      }
    }
  };

  let mut law_map = HashMap::new();
  let mut target_map = HashMap::new();
  let mut folder_stream = tokio_stream::iter(folders);
  info!("[START] parse law files");
  while let Some(folder_name) = folder_stream.next().await {
    trace!("[START] parse law: {folder_name}");
    let law_id_and_patch_id = folder_name;
    let mut law_id = String::new();
    let mut date_s = String::new();
    let mut patch_id = None;
    for (i, s) in law_id_and_patch_id.split("_").enumerate() {
      if i == 0 {
        law_id = s.to_string();
      }
      if i == 1 {
        date_s = s.to_string();
      }
      if i == 2 && s != "000000000000000" {
        patch_id = Some(s.to_string())
      }
    }
    let year = date_s[0..3].parse::<usize>()?;
    let month = date_s[4..5].parse::<usize>()?;
    let day = date_s[5..6].parse::<usize>()?;
    let xml_path = Path::new(&args.egov_folder)
      .join(&law_id_and_patch_id)
      .join(&law_id_and_patch_id)
      .with_extension("xml");
    let xml_file = fs::read_to_string(xml_path).await?;
    let mut content = None;
    if let Some(names) = law_name_list.get(&law_id) {
      for law_name in names {
        let (law_content, _triple) = egov_xml_parse(
          xml_file.as_bytes(),
          Date::new_ad(year, month, day),
          Some(law_name.clone()),
          law_id.clone(),
          patch_id.clone(),
        )?;
        let law_info = law_content.get("").unwrap();
        law_map.insert(law_name.clone(), law_info.clone());
        content = Some(law_content);
      }
    }
    if let Some(c) = content {
      target_map.insert(law_id_and_patch_id, c);
    }
  }
  info!("[END] parse law files");

  let mut target_stream = tokio_stream::iter(target_map);

  info!("[START] analysis");
  while let Some((id, target)) = target_stream.next().await {
    trace!("[START] analysis: {id}",);
    let finds = parse_ref(&target, &law_map);
    if !finds.is_empty() {
      trace!("[START] write: {id}");
      let output_file_path = Path::new(&args.output_folder)
        .join(&id)
        .with_extension("jsonl");
      let mut output_file = File::create(output_file_path).await?;
      let mut find_stream = tokio_stream::iter(finds);
      while let Some(result) = find_stream.next().await {
        let s = serde_json::to_string(&result)?;
        output_file.write_all(format!("{s}\n").as_bytes()).await?;
      }
      output_file.flush().await?;
      trace!("[END] write: {id}");
    }
    trace!("[END] analysis: {id}",);
  }
  info!("[END] analysis");

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
