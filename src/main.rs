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
    // setup()を実行し、.cocoa_rcを解析してcfgに設定情報を格納
    let cfg = load_setup::setup().unwrap();

    // cfgからstart_up_textを取得し出力
    println!("{}", cfg.start_up_text);

    // ホスト名を取得
    let host_name = hostname::get().unwrap().to_str().unwrap().to_string();

    // ユーザー名の変数を定義（所有権に注意）
    let mut username = String::new();

    // LinuxならUSER変数、WindowsならUSERNAME変数からユーザー名を取得
    if let Ok(o) = env::var("USER").or_else(|_| env::var("USERNAME")) {
        username = o;
    } else {
        // ユーザー名取得に失敗した場合、パニックを起こす（将来的には詳細なエラーメッセージを実装）
        panic!("Failed to get user name");
    }

    // 現在のディレクトリを取得
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

    // PS1用の構造体を定義
    let mut ps1 = structs::dir {
        now_dir: current_dir.clone(),
        hostname: host_name,
        user_name: username,
    };

    // ホームディレクトリを取得
    let homedir = home_dir().unwrap();

    // 履歴ファイルのパスを取得（.bash_history）
    let history_file = format!("{}/.bash_history", homedir.display());

    // rustylineの設定（補完機能を有効化）
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    // rustylineのエディタを設定
    let mut rl = DefaultEditor::with_config(config).unwrap();

    // 履歴ファイルを読み込む（存在しない場合はエラーメッセージを表示）
    if rl.load_history(&history_file).is_err() {
        println!("No previous history.");
    }

    // PS1のディレクトリを更新
    ps1.cd(&current_dir, &cfg);

    // メインループ
    loop {
        // プロンプト文字列を表示
        let display_ps1 = ps1.display_ps1();

        // 入力を取得
        let readline = rl.readline(&display_ps1);

        // 入力結果に基づいて処理を分岐
        match readline {
            Ok(input) => {
                // 入力を履歴に追加
                rl.add_history_entry(input.as_str()).unwrap();

                // 入力内容を処理し、結果を取得
                let result = input::input(&mut ps1, input.as_str(), cfg.clone());

                // "exit"コマンドなら履歴を保存して終了
                if result == "exit" {
                    rl.save_history(&history_file).unwrap();
                    std::process::exit(0);
                }
            }
            // Ctrl+Cが押された場合の処理
            Err(ReadlineError::Interrupted) => {
                // Bashのように^Cを表示して入力を無視
                println!("^C");
                continue;
            }
            // Ctrl+Dが押された場合（EOF）の処理
            Err(ReadlineError::Eof) => {
                // EOF（Ctrl+D）で終了
                return;
            }
            // その他のエラーはパニックを発生
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}