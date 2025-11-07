use crate::eli;
use anyhow::Result;
use japanese_law_id::Date;
use japanese_law_xml_schema::{
  article_number::ArticleNumber,
  law::LawType,
  utils::{
    Toc, WithNumberArticle, text_from_paragraph_list, toc_list_from_main_provision,
    with_number_article_list_from_main_provision,
  },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Law {
  date: Date,
  law_id: String,
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
  pub fn new(date: Date, law_id: String, law_type: LawType) -> Self {
    Self {
      date,
      law_id,
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

  pub fn children(&self) {}

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
      "/eli/{:0>4}/{:0>2}/{:0>2}/{}/{}/{}{}",
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
  law_id: String,
  law_type: LawType,
  patch_id: Option<String>,
) -> Result<Vec<Law>> {
  let law_data = japanese_law_xml_schema::parse_xml(buf)?;
  let mut law = Law::new(date, law_id, law_type);
  if let Some(patch_id) = patch_id {
    law.set_patch_id(patch_id);
  }
  let mut v = Vec::new();
  v.push(law.clone());

  // 編番号・章番号・条番号などを登録
  let toc_list = toc_list_from_main_provision(&law_data.law_body.main_provision);
  for toc in toc_list.iter() {
    let mut law_tmp = law.clone();
    law_tmp.set_numbers_from_toc(toc);
    v.push(law_tmp);
  }

  // 段落番号を登録する
  let (with_number_articles, paragraphs) =
    with_number_article_list_from_main_provision(&law_data.law_body.main_provision);
  for a in with_number_articles.iter() {
    let mut law_tmp = law.clone();
    law_tmp.set_numbers(a);
    for para in a.get_article().paragraph.iter() {
      law_tmp.set_paragraph_number(para.num.clone());
      law_tmp.set_paragraph_text(text_from_paragraph_list(std::slice::from_ref(para)));
      v.push(law_tmp.clone());
    }
  }
  for para_list in paragraphs.iter() {
    for para in para_list.iter() {
      let mut law_tmp = law.clone();
      law_tmp.set_paragraph_number(para.num.clone());
      law_tmp.set_paragraph_text(text_from_paragraph_list(std::slice::from_ref(para)));
      v.push(law_tmp);
    }
  }
  Ok(v)
}
