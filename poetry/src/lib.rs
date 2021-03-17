#[cfg(feature = "default")]
use poetry_data::POETRY;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::{
    fmt::{Display, Formatter, Result},
    ops::Add,
};

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
#[derive(Debug)]
pub struct Poetry {
    /// 标题
    title: &'static str,

    /// 作者
    author: &'static str,

    /// 朝代
    dynasty: &'static str,

    /// 诗句
    sentences: Vec<&'static str>,
}

impl Poetry {
    pub fn random() -> Self {
        let lines: Vec<&str> = get_lines(Self::source());

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

        // let mut content = reg(content, title, true);
        // reg(content, dynastry, true);
        // reg(content, author, true);
        // reg(content, sentence, false);

        // if let Some(title) = title {
        //     content += title;
        //     content += ".*"
        // }
        // content += r"?\|.*";

        // if let Some(dynastry) = dynastry {
        //     content += dynastry;
        //     content += ".*"
        // }
        // content += r"?\|.*";

        // if let Some(author) = author {
        //     content += author;
        //     content += ".*"
        // }
        // content += r"?\|.*";

        // if let Some(sentence) = sentence {
        //     content += sentence;
        //     content += ".*"
        // };

        println!("{}", content);

        let re = Regex::new(&content).unwrap();
        Some(
            re.captures_iter(Self::source())
                .map(|caps| Self::from(caps.get(0).unwrap().as_str()))
                .collect(),
        )
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

#[cfg(not(feature = "default"))]
impl Source for Poetry {
    fn source() -> &'static str {
        "秋浦歌·炉火照天地|唐|李白|炉火照天地，红星乱紫烟。|赧郎明月夜，歌曲动寒川。"
    }
}

#[cfg(feature = "default")]
impl Source for Poetry {
    fn source() -> &'static str {
        POETRY
    }
}

fn get_lines(str: &str) -> Vec<&str> {
    str.split_terminator('\n')
        .map(|line: &str| -> &str {
            let l = line.len();
            if l > 0 && line.as_bytes()[l - 1] == b'\r' {
                &line[0..l - 1]
            } else {
                line
            }
        })
        .collect()
}

impl From<&'static str> for Poetry {
    fn from(line: &'static str) -> Self {
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

impl Display for Poetry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let _ = writeln!(f, "{}", self.title);
        let _ = writeln!(f, "{}.{}", self.dynasty, self.author);

        for sentence in self.sentences.iter() {
            let _ = writeln!(f, "{}", sentence);
        }

        Ok(())
    }
}

trait Source {
    fn source() -> &'static str;
}
