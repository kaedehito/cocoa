mod cd;
mod input;
mod prompt;
mod structs;
use dirs::home_dir;
mod load_setup;
use rustyline::error::ReadlineError;
use rustyline::{config::Config, CompletionType, DefaultEditor};
use std::env;

fn main() {
    // setup()を実行し、.cocoa_rcをparse。cfg変数にそれぞれの情報を入れる
    let cfg = load_setup::setup().unwrap();

    // setup()で取得したcfg変数からstart_up_textを取得し出力する
    println!("{}", cfg.start_up_text);
    // hostnameを取得
    let host_name = hostname::get().unwrap().to_str().unwrap().to_string();
    #[allow(unused)]
    // usernameを入れるための変数
    // 忌々しき所有権
    let mut username = String::new();
    // Linuxの場合はUSER変数を参照してusernameを取得する。windowsの場合はUSERNAMEを参照する。
    if let Ok(o) = env::var("USER").or_else(|_| env::var("USERNAME")) {
        username = o.clone();
    } else {
        // 何らかの理由で失敗したときはエラーメッセージを出力する
        // 現在はひとまずpanicする。場合によってはmatch式にして具体的なエラーメッセージを出す
        panic!("Failed to get user name");
    };
    // 現在のcurrent_dirを取得する
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    // ps1にdir型変数を定義する
    let mut ps1 = structs::dir {
        now_dir: current_dir.clone(),
        hostname: host_name,
        user_name: username,
    };

    // homeを取得
    let homedir = home_dir().unwrap();
    // .bash_historyを取得
    let format = format!("{}/.bash_history", homedir.display());

    // rustylineのconfig
    // ファイル補完したい
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    // DefaultEditorを使用
    // API変わっててつらい...
    let mut rl = DefaultEditor::with_config(config).unwrap();

    // 履歴戻る機能
    if rl.load_history(&format).is_err() {
        println!("No previous history.");
    }

    ps1.cd(&current_dir, &cfg);

    // メインループ
    loop {
        // promptを取得
        let display_ps1 = ps1.display_ps1();
        // rustylineを使って入力を入手
        let readline = rl.readline(&display_ps1);

        // matchを使ってエラーなどを取得
        match readline {
            Ok(o) => {
                // 入力された内容を履歴に保存
                rl.add_history_entry(o.as_str()).unwrap();
                // input関数に渡して処理する
                let s = input::input(&mut ps1, o.as_str(), cfg.clone());
                // input関数がexitを返してきた場合、履歴を保存して終了する
                if s == "exit" {
                    rl.save_history(&format).unwrap();
                    std::process::exit(0);
                }
            }
            // Ctrl+Cが押された際の処理
            Err(ReadlineError::Interrupted) => {
                // bashみたいに^Cって表示して無視したい
                println!("^C");
                continue;
            }
            // Ctrl+D(EOF)が押された際の処理
            Err(ReadlineError::Eof) => {
                // bashがCtrl+Dで終了したので終了する
                return;
            }
            // それ以外はpanicする
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
