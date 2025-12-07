use crate::eli::{self, EliOntology};
use anyhow::Result;
pub use japanese_law_id::Date;
use japanese_law_xml_schema::{
  article_number::{ArticleNumber, parse_article_number},
  law::LawType,
  utils::{
    Toc, WithNumberArticle, text_from_paragraph_list, toc_list_from_main_provision,
    with_number_article_list_from_main_provision,
  },
};
pub use oxrdf::Triple;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};
use tracing::trace;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Law {
  date: Date,
  // 法令名が無く，法令番号だけの時がある
  name: Option<String>,
  law_id: String,
  law_id_text: String,
  patch_id: Option<String>,
  law_type: LawType,
  part_number: Option<ArticleNumber>,
  chapter_number: Option<ArticleNumber>,
  section_number: Option<ArticleNumber>,
  subsection_number: Option<ArticleNumber>,
  division_number: Option<ArticleNumber>,
  article_number: Option<ArticleNumber>,
  paragraph_number: Option<ArticleNumber>,
  paragraph_text: Option<String>,
  egov_link: Option<String>,
}

impl Law {
  pub fn new(
    date: Date,
    name: Option<String>,
    law_id: String,
    law_id_text: String,
    law_type: LawType,
  ) -> Self {
    Self {
      date,
      name,
      law_id,
      law_id_text,
      patch_id: None,
      law_type,
      part_number: None,
      chapter_number: None,
      section_number: None,
      subsection_number: None,
      division_number: None,
      article_number: None,
      paragraph_number: None,
      paragraph_text: None,
      egov_link: None,
    }
  }
  pub fn set_name(&mut self, name: Option<String>) {
    self.name = name
  }
  pub fn get_name(&self) -> Option<String> {
    self.name.clone()
  }
  pub fn set_law_id(&mut self, id: String, text: String) {
    self.law_id_text = text;
    self.law_id = id;
  }
  pub fn get_law_id(&self) -> String {
    self.law_id.clone()
  }
  pub fn get_law_id_text(&self) -> String {
    self.law_id_text.clone()
  }
  pub fn set_patch_id(&mut self, patch_id: String) {
    self.patch_id = Some(patch_id)
  }
  pub fn set_numbers(&mut self, number: &WithNumberArticle) {
    self.part_number = number.get_part();
    self.chapter_number = number.get_chapter();
    self.section_number = number.get_section();
    self.subsection_number = number.get_subsection();
    self.division_number = number.get_division();
    self.article_number = Some(number.get_article().num);
    self.paragraph_number = None;
  }
  pub fn set_numbers_from_toc(&mut self, toc: &Toc) {
    self.part_number = toc.get_part();
    self.chapter_number = toc.get_chapter();
    self.section_number = toc.get_section();
    self.subsection_number = toc.get_subsection();
    self.division_number = toc.get_division();
    self.article_number = toc.get_article();
    self.paragraph_number = None;
  }
  pub fn set_paragraph_number(&mut self, paragraph_number: ArticleNumber) {
    self.paragraph_number = Some(paragraph_number);
  }
  pub fn set_paragraph_text(&mut self, text: String) {
    self.paragraph_text = Some(text);
  }
  pub fn set_egov_link(&mut self, egov_link: String) {
    self.egov_link = Some(egov_link);
  }

  pub fn law_type_str(&self) -> String {
    let s = match self.law_type {
      // 憲法
      LawType::Constitution => "constitution",
      // 法律
      LawType::Act => "act",
      // 政令, 太政官布告など
      LawType::CabinetOrder => "co",
      // 勅令
      LawType::ImperialOrder => "io",
      // 府省令
      LawType::MinisterialOrdinance => "mo",
      // 規則
      LawType::Rule => "rule",
      // その他
      LawType::Misc => "misc",
    };
    String::from(s)
  }
  pub fn article_number_str(&self) -> Option<String> {
    self
      .article_number
      .clone()
      .map(|num| format!("article{}", num.num_str()))
  }
  pub fn paragraph_number_str(&self) -> Option<String> {
    self
      .paragraph_number
      .clone()
      .map(|num| format!("paragraph{}", num.num_str()))
  }

  // 番号を検索して親要素を生成する
  pub fn parent(&self) -> Self {
    let mut parent = self.clone();
    if self.paragraph_number.is_some() {
      parent.paragraph_number = None;
      parent.paragraph_text = None;
    } else if self.article_number.is_some() {
      parent.article_number = None;
    } else if self.division_number.is_some() {
      parent.division_number = None;
    } else if self.subsection_number.is_some() {
      parent.subsection_number = None
    } else if self.section_number.is_some() {
      parent.section_number = None;
    } else if self.chapter_number.is_some() {
      parent.chapter_number = None;
    } else if self.part_number.is_some() {
      parent.part_number = None;
    }
    parent
  }

  /// 第○章，第○条第△項といった条項番号のテキストを生成する
  fn number_text(&self) -> String {
    if let Some(num) = &self.part_number {
      num.part_text()
    } else if let Some(num) = &self.chapter_number {
      num.chapter_text()
    } else if let Some(num) = &self.section_number {
      num.section_text()
    } else if let Some(num) = &self.subsection_number {
      num.subsection_text()
    } else if let Some(num) = &self.division_number {
      num.division_text()
    } else if let Some(num) = &self.article_number {
      if let Some(para_num) = &self.paragraph_number {
        format!("{}{}", num.article_text(), para_num.paragraph_text())
      } else {
        num.article_text()
      }
    } else if let Some(num) = &self.paragraph_number {
      num.paragraph_text()
    } else {
      String::new()
    }
  }

  /// `#Mp-Pa_2-Ch_40`のような，条項に振られているIDを生成する．
  /// 具体的な例: <https://laws.e-gov.go.jp/law/129AC0000000089#Mp-Pa_3-Ch_1-Se_2-Ss_3-Di_4>
  /// まずはMainProvisionだけ対応．
  fn egov_id(&self) -> Option<String> {
    let mut s = String::new();
    if let Some(num) = &self.part_number {
      s.push_str(&format!("-Pa_{}", num.num_str()))
    }
    if let Some(num) = &self.chapter_number {
      s.push_str(&format!("-Ch_{}", num.num_str()))
    }
    if let Some(num) = &self.section_number {
      s.push_str(&format!("-Se_{}", num.num_str()))
    }
    if let Some(num) = &self.subsection_number {
      s.push_str(&format!("-Ss_{}", num.num_str()))
    }
    if let Some(num) = &self.division_number {
      s.push_str(&format!("-Di_{}", num.num_str()))
    }
    if let Some(num) = &self.article_number {
      s.push_str(&format!("-At_{}", num.num_str()))
    }
    if let Some(num) = &self.paragraph_number {
      s.push_str(&format!("-Pr_{}", num.num_str()))
    }
    if s.is_empty() {
      None
    } else {
      Some(format!("#Mp{s}"))
    }
  }
}

impl eli::Eli for Law {
  fn published(&self) -> eli::Published {
    if let Some(link) = &self.egov_link {
      eli::Published::Uri(link.clone())
    } else {
      eli::Published::Uri(format!(
        "https://laws.e-gov.go.jp/law/{}/{}_{}{}",
        self.law_id,
        self.date.joined_str(),
        self
          .patch_id
          .clone()
          .unwrap_or("000000000000000".to_string()),
        self.egov_id().unwrap_or_default()
      ))
    }
  }

  /// `/eli/2024/12/12/mo/506M60000100140/000000000000000/article2/paragraph2`のような感じ
  /// 日付，法令の種類，法令ID，改正法令ID，条番号，段落番号
  fn eli_uri(&self) -> String {
    format!(
      "https://github.com/puripuri2100-research/eli/{:0>4}/{:0>2}/{:0>2}/{}/{}/{}{}",
      self.date.get_ad_year(),
      self.date.get_month(),
      self.date.get_day(),
      self.law_type_str(),
      self.law_id,
      if let Some(s) = self.article_number_str() {
        format!("/{s}")
      } else {
        String::new()
      },
      if let Some(s) = self.paragraph_number_str() {
        format!("/{s}")
      } else {
        String::new()
      }
    )
  }
}

pub fn egov_xml_parse(
  buf: &[u8],
  date: Date,
  law_name: Option<String>,
  law_id: String,
  patch_id: Option<String>,
) -> Result<(HashMap<String, Law>, Vec<Triple>)> {
  let parsed_law = japanese_law_xml_schema::parse_xml(buf)?;
  let law_id_text = parsed_law.law_num;
  let mut law = Law::new(date, law_name, law_id, law_id_text, parsed_law.law_type);
  if let Some(patch_id) = patch_id {
    law.set_patch_id(patch_id);
  }
  let mut law_data = HashMap::new();
  law_data.insert(String::new(), law.clone());

  let mut v_triple = Vec::new();

  // 編番号・章番号・条番号などを登録
  let toc_list = toc_list_from_main_provision(&parsed_law.law_body.main_provision);
  for toc in toc_list.iter() {
    let mut law_tmp = law.clone();
    law_tmp.set_numbers_from_toc(toc);
    law_data.insert(law_tmp.number_text(), law_tmp.clone());
    let parent = law_tmp.parent();
    v_triple.push(EliOntology::HasPart.triple(parent.clone(), law_tmp.clone()));
    v_triple.push(EliOntology::IsPartOf.triple(law_tmp.clone(), parent.clone()));
  }

  // 段落番号を登録する
  let (with_number_articles, paragraphs) =
    with_number_article_list_from_main_provision(&parsed_law.law_body.main_provision);
  for a in with_number_articles.iter() {
    let mut law_tmp = law.clone();
    law_tmp.set_numbers(a);
    for para in a.get_article().paragraph.iter() {
      let mut law_tmp2 = law_tmp.clone();
      law_tmp2.set_paragraph_number(para.num.clone());
      law_tmp2.set_paragraph_text(text_from_paragraph_list(std::slice::from_ref(para)));
      law_data.insert(law_tmp2.number_text(), law_tmp2.clone());
      v_triple.push(EliOntology::HasPart.triple(law_tmp.clone(), law_tmp2.clone()));
      v_triple.push(EliOntology::IsPartOf.triple(law_tmp2.clone(), law_tmp.clone()));
    }
  }
  for para_list in paragraphs.iter() {
    for para in para_list.iter() {
      let mut law_tmp = law.clone();
      law_tmp.set_paragraph_number(para.num.clone());
      law_tmp.set_paragraph_text(text_from_paragraph_list(std::slice::from_ref(para)));
      law_data.insert(para.num.paragraph_text(), law_tmp.clone());
      v_triple.push(EliOntology::HasPart.triple(law.clone(), law_tmp.clone()));
      v_triple.push(EliOntology::IsPartOf.triple(law_tmp.clone(), law.clone()));
    }
  }
  Ok((law_data, v_triple))
}

fn ord_article_number(a: &ArticleNumber, b: &ArticleNumber) -> Ordering {
  if a.base_number == b.base_number {
    for (a_e, b_e) in a.eda_numbers.iter().zip(&b.eda_numbers) {
      if a_e != b_e {
        return a_e.cmp(b_e);
      }
    }
    a.eda_numbers.len().cmp(&b.eda_numbers.len())
  } else {
    a.base_number.cmp(&b.base_number)
  }
}

fn ord_article(a: &Law, b: &Law) -> Ordering {
  let a_num = &a.article_number;
  let b_num = &b.article_number;
  match (a_num, b_num) {
    (None, Some(_)) => {
      // Noneは前文の可能性が高い
      Ordering::Less
    }
    (Some(_), None) => Ordering::Greater,
    (None, None) => {
      let a_p = &a.paragraph_number;
      let b_p = &b.paragraph_number;
      match (a_p, b_p) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a_pa), Some(b_pa)) => ord_article_number(a_pa, b_pa),
      }
    }
    (Some(a_a), Some(b_a)) => {
      let o = ord_article_number(a_a, b_a);
      if o == Ordering::Equal {
        let a_p = &a.paragraph_number;
        let b_p = &b.paragraph_number;
        match (a_p, b_p) {
          (None, None) => Ordering::Equal,
          (None, Some(_)) => Ordering::Less,
          (Some(_), None) => Ordering::Greater,
          (Some(a_pa), Some(b_pa)) => ord_article_number(a_pa, b_pa),
        }
      } else {
        o
      }
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Position {
  start: usize,
  end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct FindLawName {
  position: Position,
  find_law: Option<Law>,
  match_string: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Find {
  /// 参照先
  to: Law,
  /// 参照元
  from: Law,
  /// fromの中でのテキスト内の位置
  position: Position,
}

/// 参照情報を抽出する
/// - target: 解析対象の法令の情報
/// - law_map: 法令名や法令IDのテキストをkeyとし，法令全体を表すLawをvalueとするhashmap
pub fn parse_ref(target: &HashMap<String, Law>, law_map: &HashMap<String, Law>) -> Vec<Find> {
  // 段落を順番で並べ替える
  let mut paragraph_list = target
    .values()
    .filter(|l| l.paragraph_text.is_some())
    .cloned()
    .collect::<Vec<_>>();
  paragraph_list.sort_by(ord_article);

  // テキストから
  // - 法令名の出現箇所
  // - 略称が定義されている箇所
  // - 同法・同令が出現する箇所
  // を検索する
  let mut law_name_list = Vec::new();

  let mut result = Vec::new();

  for paragraph in paragraph_list.iter() {
    trace!(
      "[START] parse paragraph {:?} - {:?}",
      paragraph.article_number_str(),
      paragraph.paragraph_number_str()
    );
    if let Some(text) = &paragraph.paragraph_text {
      // 正式名称の一覧を持ってテキスト内検索を行う
      let mut find_law_name_result = find_law_name(text, law_map, &law_name_list);

      // 略称の定義箇所を検索する
      let find_abb_def_result = find_abb_def(text);
      // 今までの項で見つかった法令名と，この項で見つかった略称を紐付けていく
      let mut linked_abb_def_result = find_abb_def_result
        .iter()
        .filter_map(|l| linking_abb_and_full_name(l, &find_law_name_result))
        .collect::<Vec<_>>();

      // 同法に紐付けさせるためのリスト
      let mut linked_abb_def_result2 = linked_abb_def_result.clone();
      find_law_name_result.append(&mut linked_abb_def_result2);

      // 同法・同令の出現位置を検索する
      let find_douhou_result = find_douhou(text);
      // 今までの項で見つかった法令名と略称の情報と，この項で見つかった「同法」・「同令」を紐付けていく
      let mut linked_douhou_result = find_douhou_result
        .iter()
        .filter_map(|l| linking_abb_and_full_name(l, &find_law_name_result))
        .collect::<Vec<_>>();

      find_law_name_result.append(&mut linked_douhou_result);

      for find_law_name in find_law_name_result.iter() {
        if let Some(l) = &find_law_name.find_law {
          // 条項の検索をする
          let mut to_law = l.clone();
          let end = find_joukou(text, &find_law_name.position, &mut to_law);
          // 結果を返す
          result.push(Find {
            to: to_law.clone(),
            from: paragraph.clone(),
            position: Position {
              start: find_law_name.position.start,
              end,
            },
          });
        }
      }

      // 略称は他の項でも見るので追加
      law_name_list.append(&mut linked_abb_def_result);
    }
    trace!(
      "[END] parse paragraph {:?} - {:?}",
      paragraph.article_number_str(),
      paragraph.paragraph_number_str()
    );
  }
  result
}

/// 法令名の検索
fn find_law_name(
  text: &str,
  law_map: &HashMap<String, Law>,
  find_lst: &[FindLawName],
) -> Vec<FindLawName> {
  let text_chars = text.chars().collect::<Vec<_>>();
  let byte_to_char_map = byte_to_char_index_map(text);
  let mut lst = Vec::new();
  let mut v1 = law_map
    .iter()
    .map(|(k, v)| (k.clone(), v.clone()))
    .collect::<Vec<_>>();
  let mut v2 = find_lst
    .iter()
    .map(|v| (v.match_string.clone(), v.find_law.clone().unwrap()))
    .collect::<Vec<_>>();
  v1.append(&mut v2);
  for (find_law_name, law) in v1.iter() {
    for (start, s) in text.match_indices(find_law_name.as_str()) {
      let end = start + s.len();
      let start = byte_to_char_map[start];
      let end = byte_to_char_map[end];
      let match_text = s;
      // law_nameが「法」や「令」のときは隣の文字をチェックする
      // 隣の文字も含めて「同法」・「方法」・「法人」・「命令」、「政令」・「同令」・「法令」だった時は普遍的過ぎて法令名ではないことが多いので除外
      // 「〇〇年法律第〇〇号」や「〇〇年〇〇省令第〇〇号」や「〇〇年〇〇委員会規則第〇〇号」や「〇〇年〇〇院規則第〇〇号」なども排除
      let mut is_universal_name = false;
      if *find_law_name == "法"
        && start != 0
        && (text_chars[start - 1] == '方'
          || text_chars[start - 1] == '同'
          || text_chars[start - 1] == '旧')
      {
        is_universal_name = true
      }
      if find_law_name.ends_with('法') && end < text_chars.len() - 1 && text_chars[end] == '人' {
        is_universal_name = true
      }
      if *find_law_name == "法"
        && end < text_chars.len() - 1
        && (text_chars[end] == '令' || text_chars[end] == '律')
      {
        is_universal_name = true
      }
      if find_law_name.ends_with('法')
        && end < text_chars.len() - 2
        && text_chars[end] == '律'
        && text_chars[end + 1] == '第'
      {
        is_universal_name = true
      }
      if *find_law_name == "令"
        && start != 0
        && (text_chars[start - 1] == '命'
          || text_chars[start - 1] == '政'
          || text_chars[start - 1] == '同'
          || text_chars[start - 1] == '法'
          || text_chars[start - 1] == '省'
          || text_chars[start - 1] == '府'
          || text_chars[start - 1] == '勅'
          || text_chars[start - 1] == '旧')
      {
        is_universal_name = true
      }
      if *find_law_name == "令"
        && end < text_chars.len() - 1
        && text_chars[end] == '第'
        && start != 0
        && (text_chars[start - 1] == '省'
          || text_chars[start - 1] == '政'
          || text_chars[start - 1] == '勅'
          || text_chars[start - 1] == '府')
      {
        is_universal_name = true
      }
      if find_law_name.ends_with('則')
        && end < text_chars.len() - 1
        && text_chars[end] == '第'
        && start > 2
        && (text_chars[start - 1] == '規')
        && (text_chars[start - 2] == '院' || text_chars[start - 2] == '会')
      {
        is_universal_name = true
      }

      if end < text_chars.len() - 1 && text_chars[end] == '」' {
        is_universal_name = true
      }

      if !is_universal_name {
        let find = FindLawName {
          position: Position { start, end },
          match_string: match_text.to_string(),
          find_law: Some(law.clone().clone()),
        };
        lst = resolve_duplicates(&lst, &find);
      }
    }
  }
  // 「内閣は、消防施設強化促進法（昭和二十八年法律第八十七号）第三条の規定に基き、この政令を制定する。」
  // のような文における，法令番号の抽出を抑制したい．
  // 具体的には，次のパターンに該当するかどうかをチェックする．
  // - 同一の法令を指し示す法令名のendと法令番号のstartの差が1
  // - 当該endと当該startの間の文字が'（'
  // 該当したときに法令番号側を削除する
  resolve_name_and_number(&mut lst, text);

  // 最終的な法令名探索結果
  lst
}

/// 「内閣は、消防施設強化促進法（昭和二十八年法律第八十七号）第三条の規定に基き、この政令を制定する。」
/// のような文における，法令名と法令番号の重複を解消するために，法令番号を削除する．
fn resolve_name_and_number(lst: &mut Vec<FindLawName>, text: &str) {
  let chars = text.chars().collect::<Vec<char>>();
  // 削除対象
  let mut remove_index: Vec<usize> = Vec::new();
  let mut iter = lst.iter().enumerate().peekable();
  while let Some((i, law_name)) = iter.next() {
    if let Some(&(j, next_law_name)) = iter.peek() {
      let (i2, law1, law2) = if law_name.position.end < next_law_name.position.start {
        (j, law_name, next_law_name)
      } else {
        (i, next_law_name, law_name)
      };
      if law1.position.end.abs_diff(law2.position.start) == 1 // 差が1
        && chars.get(law1.position.end) == Some(&'（') // 間にある文字が全角かっこ
        && law1.find_law.clone().map(|l| l.get_law_id())
        == law2.find_law.clone().map(|l| l.get_law_id())
        && !law1.match_string.ends_with("号") // 前側が号で終わらず
        && law2.match_string.ends_with("号")
      // 後ろ側が号で終わる
      {
        // 削除対象に登録
        remove_index.push(i2)
      }
    }
  }
  for i in remove_index.iter() {
    lst.remove(*i);
  }
}

#[test]
fn check_resolve_name_and_number() {
  let s = "陸上交通事業調整法（以下「法」という。）第二条第一項の政令で定める審議会等は、交通政策審議会とする。ただし、法第二条第一項の規定に基づき、国土交通大臣が都市計画法（昭和四十三年法律第百号）第四条第二項に規定する都市計画区域内において調整の区域を決定しようとするときは、当該調整の区域について交通政策審議会及び社会資本整備審議会とする。";
  let mut v = vec![
    FindLawName {
      position: Position { start: 0, end: 25 },
      match_string: String::from("陸上交通事業調整法"),
      find_law: Some(Law::new(
        Date::new_ad(2000, 1, 1),
        Some(String::from("陸上交通事業調整法")),
        String::from("313AC0000000071"),
        String::from("昭和十三年法律第七十一号"),
        LawType::Act,
      )),
    },
    FindLawName {
      position: Position { start: 82, end: 93 },
      match_string: String::from("昭和四十三年法律第百号"),
      find_law: Some(Law::new(
        Date::new_ad(2000, 1, 1),
        Some(String::from("都市計画法")),
        String::from("343AC0000000100"),
        String::from("昭和四十三年法律第百号"),
        LawType::Act,
      )),
    },
    FindLawName {
      position: Position { start: 76, end: 81 },
      match_string: String::from("都市計画法"),
      find_law: Some(Law::new(
        Date::new_ad(2000, 1, 1),
        Some(String::from("都市計画法")),
        String::from("343AC0000000100"),
        String::from("昭和四十三年法律第百号"),
        LawType::Act,
      )),
    },
    FindLawName {
      position: Position { start: 13, end: 14 },
      match_string: String::from("法"),
      find_law: Some(Law::new(
        Date::new_ad(2000, 1, 1),
        Some(String::from("陸上交通事業調整法")),
        String::from("313AC0000000071"),
        String::from("昭和十三年法律第七十一号"),
        LawType::Act,
      )),
    },
  ];

  resolve_name_and_number(&mut v, s);

  println!("81: {:?}", s.chars().nth(81));

  let v2 = vec![
    FindLawName {
      position: Position { start: 0, end: 25 },
      match_string: String::from("陸上交通事業調整法"),
      find_law: Some(Law::new(
        Date::new_ad(2000, 1, 1),
        Some(String::from("陸上交通事業調整法")),
        String::from("313AC0000000071"),
        String::from("昭和十三年法律第七十一号"),
        LawType::Act,
      )),
    },
    FindLawName {
      position: Position { start: 76, end: 81 },
      match_string: String::from("都市計画法"),
      find_law: Some(Law::new(
        Date::new_ad(2000, 1, 1),
        Some(String::from("都市計画法")),
        String::from("343AC0000000100"),
        String::from("昭和四十三年法律第百号"),
        LawType::Act,
      )),
    },
    FindLawName {
      position: Position { start: 13, end: 14 },
      match_string: String::from("法"),
      find_law: Some(Law::new(
        Date::new_ad(2000, 1, 1),
        Some(String::from("陸上交通事業調整法")),
        String::from("313AC0000000071"),
        String::from("昭和十三年法律第七十一号"),
        LawType::Act,
      )),
    },
  ];
  assert_eq!(v, v2)
}

// 略称の定義を検索
fn find_abb_def(text: &str) -> Vec<FindLawName> {
  let text_chars = text.chars().collect::<Vec<_>>();
  let byte_to_char_map = byte_to_char_index_map(text);
  let mut lst = Vec::new();
  let abb_re = Regex::new(r"以下「([^」]*(法|令|規則))」").unwrap();
  for m in abb_re.find_iter(text) {
    // "以下「"の分足す
    let start = m.start() + "以下「".len();
    // 鉤括弧の分引く
    let end = m.end() - "」".len();
    // charでのインデックスにマップする
    let start = byte_to_char_map[start];
    let end = byte_to_char_map[end];
    let abb = text_chars[start..end].iter().collect::<String>();
    // 方法や命令は今回の対象ではないので除く
    if !(abb.ends_with("方法") || abb.ends_with("命令")) {
      let find = FindLawName {
        position: Position { start, end },
        match_string: abb,
        find_law: None,
      };
      lst = resolve_duplicates(&lst, &find);
    }
  }
  lst
}

// 「同法」と「同令」で再度検索する
fn find_douhou(text: &str) -> Vec<FindLawName> {
  let text_chars = text.chars().collect::<Vec<char>>();
  let byte_to_char_map = byte_to_char_index_map(text);
  let mut lst = Vec::new();
  let douhou_re = Regex::new(r"同法|同令").unwrap();
  for m in douhou_re.find_iter(text) {
    let start = byte_to_char_map[m.start()];
    let end = byte_to_char_map[m.end()];
    let match_text = m.as_str();
    if !((end < text_chars.len() - 1 && text_chars[end] == '人')
      || (end < text_chars.len() - 2 && text_chars[end] == '律' && text_chars[end + 1] == '第'))
    {
      let find = FindLawName {
        position: Position { start, end },
        match_string: match_text.to_string(),
        find_law: None,
      };
      lst.push(find);
    }
  }
  lst
}

/// 条項番号を検索する
/// 法令名の後の括弧がきを飛ばし，その後に「第一条」のような文字列が出るのを期待する
/// "第"が出なかったら法令名だけなので処理を打ち切り
/// 引数として可変のLawを受け取って内部の情報を更新する
/// 返り値は最終的な範囲のend
fn find_joukou(text: &str, position: &Position, law: &mut Law) -> usize {
  let mut s = String::new();
  let mut paren_depth = 0_usize;
  let target_c = [
    '第', '条', '項', 'の', 'ノ', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '百',
    '千',
  ];
  let mut end = position.end;
  for (i, c) in text.chars().enumerate() {
    if i < position.end {
      continue;
    }
    if c == '（' {
      paren_depth += 1;
      continue;
    }
    if c == '）' {
      paren_depth -= 1;
      continue;
    }
    if paren_depth > 0 {
      continue;
    }
    if paren_depth == 0 && target_c.contains(&c) {
      s.push(c);
      end = i;
      continue;
    }
    break;
  }
  // 末尾が'の', 'ノ'ならばそれを取り除く
  if s.ends_with('の') {
    s = s.trim_end_matches('の').to_string();
    end -= 1;
  }
  if s.ends_with('ノ') {
    s = s.trim_end_matches('ノ').to_string();
    end -= 1;
  }
  trace!("find joukou number string: {s}");
  for a in s.split("第") {
    if !a.is_empty() {
      let s2 = format!("第{a}");
      trace!("find joukou number string(split): {s2}");
      let num = parse_article_number(&s2);
      trace!("parsed article number: {num:?}");
      if let Some(num) = num {
        if a.ends_with("条") {
          law.article_number = Some(num)
        } else if a.ends_with("項") {
          law.paragraph_number = Some(num)
        } else if a.ends_with("編") {
          law.part_number = Some(num)
        } else if a.ends_with("章") {
          law.chapter_number = Some(num)
        } else if a.ends_with("節") {
          law.section_number = Some(num)
        } else if a.ends_with("款") {
          law.subsection_number = Some(num)
        } else if a.ends_with("目") {
          law.division_number = Some(num)
        }
      }
    }
  }
  end
}

// 各charの始まりに該当するバイト位置をcharの位置に変換するためのマップ
fn byte_to_char_index_map(text: &str) -> Vec<usize> {
  // 各バイト位置に対する char インデックス
  let mut map = vec![0; text.len() + 1];
  let mut char_index = 0;
  for (byte_index, _) in text.char_indices() {
    map[byte_index] = char_index;
    char_index += 1;
  }
  map[text.len()] = char_index;
  map
}

/// 範囲が重複した法令について、重複を解消する
/// 原則として範囲が大きい方が優先
/// 同じ範囲だった場合は後から見つかったものを優先（法令名中の略称の場合なので優先される）
/// 入力のfind_lstは常に解消済みのものであるとする
fn resolve_duplicates(find_lst: &[FindLawName], find: &FindLawName) -> Vec<FindLawName> {
  let mut lst = Vec::new();
  let mut ok = false;
  for f in find_lst.iter() {
    if ok {
      // 重複する物が見つかったのであとはスルー
      lst.push(f.clone())
    } else if f.position.start == find.position.start && f.position.end == find.position.end {
      // 完全に一致するものが見つかったので置き換え
      if !ok {
        lst.push(find.clone());
        ok = true;
      }
    } else if f.position.start >= find.position.start && f.position.end <= find.position.end {
      // 見つかったものの方が小さかったので置き換え
      if !ok {
        lst.push(find.clone());
        ok = true
      }
    } else if f.position.start <= find.position.start && f.position.end >= find.position.end {
      // 見つかったものの方が大きかったので置き換えない
      lst.push(f.clone());
      ok = true
    } else {
      // 無関係だった
      lst.push(f.clone())
    }
  }
  // 重複は無かったので追加
  if !ok {
    lst.push(find.clone())
  }
  lst
}

// find_abb_defとsearch_douhouの実行結果と、find_law_nameの実行結果を比較して、抽出位置が直前のものを紐づける
fn linking_abb_and_full_name(
  abb_info: &FindLawName,
  full_name_info_list: &[FindLawName],
) -> Option<FindLawName> {
  let mut result: Option<FindLawName> = None;
  for full_name_info in full_name_info_list.iter() {
    if full_name_info.position.end <= abb_info.position.start {
      if let Some(ref old_result) = result
        && full_name_info.position.end < old_result.position.end
      {
        // すでに見つかったものよりも遠い場合は上書きしない
        continue;
      } else {
        result = Some(full_name_info.clone())
      }
    }
  }
  if let Some(result) = result {
    Some(FindLawName {
      find_law: result.find_law.clone(),
      ..abb_info.clone()
    })
  } else {
    None
  }
}

#[test]
fn check_linking() {
  let f = FindLawName {
    position: Position { start: 20, end: 27 },
    match_string: String::new(),
    find_law: None,
  };
  let lst = vec![
    FindLawName {
      position: Position { start: 5, end: 7 },
      match_string: String::new(),
      find_law: Some(Law::new(
        Date::new_ad(2025, 11, 26),
        None,
        String::from("test1"),
        String::new(),
        LawType::Act,
      )),
    },
    FindLawName {
      position: Position { start: 8, end: 10 },
      match_string: String::new(),
      find_law: Some(Law::new(
        Date::new_ad(2025, 11, 26),
        None,
        String::from("test2"),
        String::new(),
        LawType::Act,
      )),
    },
    FindLawName {
      position: Position { start: 29, end: 31 },
      match_string: String::new(),
      find_law: Some(Law::new(
        Date::new_ad(2025, 11, 26),
        None,
        String::from("test3"),
        String::new(),
        LawType::Act,
      )),
    },
  ];
  let result = linking_abb_and_full_name(&f, &lst);
  assert!(result.is_some());
  assert_eq!(
    result.unwrap().find_law.map(|l| l.get_law_id()),
    Some(String::from("test2"))
  );
}
