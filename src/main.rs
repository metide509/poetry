#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};
use poetry::Poetry;

// fn main() {
//     let poetry = Poetry::random();
//     println!("{}", poetry);

//     let poetry = Poetry::find("静夜思");
//     for poetry in poetry.unwrap() {
//         println!("{}", poetry);
//     }
// }

//extern crate clap;

fn main() {
    let app = make_app();
    execute_command(app);
}

fn make_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(concat!("v", crate_version!()))
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .help_message("显示帮助信息")
        .version_message("显示版本信息")
        .after_help("输出 `fcli <command> --help` 查看更多命令用法。")
        .arg(
            Arg::with_name("AUTHOR")
                .short("a")
                .long("author")
                .value_name("AUTHOR")
                .takes_value(true)
                .help("作者"),
        )
        .arg(
            Arg::with_name("TITLE")
                .short("t")
                .long("title")
                .value_name("TITLE")
                .takes_value(true)
                .help("标题"),
        )
        .arg(
            Arg::with_name("DYNASTY")
                .short("d")
                .long("dynasty")
                .value_name("DYNASTY")
                .takes_value(true)
                .help("朝代"),
        )
        .arg(
            Arg::with_name("SENTENCE")
                .short("s")
                .long("sentence")
                .value_name("SENTENCE")
                .takes_value(true)
                .help("诗句"),
        )
        .arg(
            Arg::with_name("NUMBER")
                .short("n")
                .long("number")
                .value_name("NUMBER")
                .takes_value(true)
                .help("数量"),
        )
        .arg(Arg::with_name("RANDOM").long("random").help("随机"));

    app
}

fn execute_command(app: App) {
    let matches = app.get_matches();

    let title = matches.value_of("TITLE");
    let author = matches.value_of("AUTHOR");
    let dynasty = matches.value_of("DYNASTY");
    let sentence = matches.value_of("SENTENCE");
    let number = matches
        .value_of("NUMBER")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1);

    let is_random = matches.is_present("RANDOM");

    if is_random {
        for _i in 0..number {
            print_poetry(Poetry::random());
        }
    } else {
        let poetrys = Poetry::find(title, dynasty, author, sentence).unwrap();
        for p in poetrys {
            print_poetry(p);
        }
    }
}

fn print_poetry(p: Poetry) {
    let mut out = format!(
        "{}\r\n{}.{}",
        p.get_title(),
        p.get_dynasty(),
        p.get_author()
    );
    for sentence in p.get_sentences().iter() {
        out += format!("\r\n{}", sentence).as_str();
    }

    println!("{}\r\n", out)
}
