use crate::load_setup;
use crate::structs;
use dirs::home_dir;
use std::{
    io::{self, Write},
    process::{Command, Stdio},
    str,
};
/// 入力されたコマンドを解析し、適切な処理を実行する関数。
/// - `ps1`: 現在のディレクトリ情報を保持する`dir`構造体（ミュータブル）
/// - `input`: ユーザーからの入力文字列
/// - `setup`: 設定データ（`Config`構造体）を受け取る
pub fn input(ps1: &mut structs::dir, input: &str, setup: load_setup::Config) -> String {
    // 入力文字列を空白で分割し、Vec<&str>型のコマンドリストに変換
    let cmd = input.split_whitespace().collect::<Vec<&str>>();

    // コマンドの最初の要素に基づいて処理を分岐
    if let Some(command) = cmd.first() {
        match *command {
            // cdコマンド：ディレクトリを変更
            "cd" => {
                // 2つ目の引数があれば指定のディレクトリに移動、なければホームディレクトリに移動
                if cmd.len() == 2 {
                    ps1.cd(&cmd[1].to_string(), &setup);
                } else {
                    let home = home_dir().unwrap();
                    ps1.cd(&home.to_str().unwrap().to_string(), &setup);
                }
                return "".to_string();
            }
            // exitコマンド：シェルを終了
            "exit" => {
                println!("exit");
                return "exit".to_string();
            }
            // 空の入力は無視
            "" => return "".to_string(),
            // lsコマンド：ディレクトリの内容を表示
            "ls" => {
                if let Ok(o) = Command::new("ls")
                    .args(&cmd[1..])
                    .arg(setup.ls_config) // 設定ファイルで指定されたオプションを追加
                    .arg("-C")             // ディレクトリとファイルを横並びに表示
                    .output()
                {
                    // 出力をUTF-8として解釈し、日本語を含む内容を表示
                    let stdout = str::from_utf8(&o.stdout).unwrap();
                    print!("{}", stdout); // printlnではなくprintで余分な改行を回避
                    io::stdout().flush().unwrap();
                }
                return "".to_string();
            }
            // その他のコマンド：外部コマンドとして実行
            _ => {}
        }

        // 外部コマンドの実行
        match Command::new(command)
            .args(&cmd[1..])  // 引数を設定
            .stdin(Stdio::inherit())   // インタラクティブに動作させる
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(mut o) => {
                o.wait().unwrap(); // コマンドの終了を待機
            }
            Err(e) => {
                // コマンドが見つからない場合のエラーメッセージを出力
                if e.kind() == io::ErrorKind::NotFound {
                    eprintln!("cocoa: {}: command not found", command);
                } else {
                    eprintln!("{}", e);
                }
            }
        }
    }
    "".to_string() // コマンドが実行されなかった場合は空文字列を返す
}