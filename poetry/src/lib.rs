use data::POETRY;
use rand::{thread_rng, Rng};
use regex::Regex;

mod data;

/// 诗歌
/// 包含了以下几个部分
///
/// - 标题
/// - 作者
/// - 朝代
/// - 诗句
///
/// ## 示例
///
/// ```rust
/// let p = Poetry::random();
/// println!("{}", p);
/// ```
/// 输出
///
/// ```shell
/// 江城子·乙卯正月二十日夜记梦
/// 宋.苏轼
/// 十年生死两茫茫，不思量，自难忘。
/// 千里孤坟，无处话凄凉。
/// 纵使相逢应不识，尘满面，鬓如霜。
/// 夜来幽梦忽还乡，小轩窗，正梳妆。
/// 相顾无言，惟有泪千行。
/// 料得年年肠断处，明月夜，短松冈。
/// ```
pub struct Poetry<'a> {
    /// 标题
    title: &'a str,

    /// 作者
    author: &'a str,

    /// 朝代
    dynasty: &'a str,

    /// 诗句
    sentences: Vec<&'a str>,
}

impl<'a> Poetry<'a> {
    pub fn random() -> Self {
        let lines = lines(POETRY);

        let mut rng = thread_rng();
        let n = rng.gen_range(0..lines.len());

        let line = lines[n];
        Self::from(line)
    }

    pub fn find(
        title: Option<&str>,
        dynastry: Option<&str>,
        author: Option<&str>,
        sentence: Option<&str>,
    ) -> Option<Vec<Self>> {
        let mut content = String::from(".*");
        content += reg(title, true).as_str();
        content += reg(dynastry, true).as_str();
        content += reg(author, true).as_str();
        content += reg(sentence, false).as_str();

        let re = Regex::new(&content).unwrap();
        Some(
            re.captures_iter(POETRY)
                .map(|caps| Self::from(caps.get(0).unwrap().as_str()))
                .collect(),
        )
    }

    pub fn get_title(&self) -> &str {
        self.title
    }

    pub fn get_author(&self) -> &str {
        self.author
    }

    pub fn get_dynasty(&self) -> &str {
        self.dynasty
    }

    pub fn get_sentences(&self) -> Vec<&str> {
        self.sentences.to_vec()
    }
}

fn reg(v: Option<&str>, add_lasy: bool) -> String {
    let v = v.unwrap_or("");
    if add_lasy {
        format!(r"{}.*?\|.*", v)
    } else {
        format!(r"{}.*", v)
    }
}

fn lines(str: &str) -> Vec<&str> {
    str.split_terminator('\n')
        .map(|l| l.trim_end_matches("\r"))
        .collect()
}

impl<'a> From<&'a str> for Poetry<'a> {
    fn from(line: &'a str) -> Self {
        let poetry: Vec<&str> = line.split('|').collect();
        let sentences: Vec<&str> = poetry[3..].to_vec();

        Poetry {
            title: poetry[0],
            dynasty: poetry[1],
            author: poetry[2],
            sentences: sentences,
        }
    }
}