use crate::load_setup;
use crate::structs;
use dirs::home_dir;
use std::{
    io::{self, Write},
    process::{Command, Stdio},
    str,
};
/// 入力されたコマンドを解釈、実行する関数。
/// ps1には&mut dirを、inputには入力する&str型を、setupにはload_setupファイルに定義されているConfig構造体を入力する
pub fn input(ps1: &mut structs::dir, input: &str, setup: load_setup::Config) -> String {
    // inputを空白で分割する。Vec<&str>型になる
    let cmd = input.split_whitespace().collect::<Vec<&str>>();

    // commandを解釈するmatch式。
    // cmd変数の最初の要素を取得し、matchで処理を変える
    // cdやexitコマンドを定義している。
    // また、lsも定義している
    if let Some(command) = cmd.first() {
        match *command {
            // 最初の要素がcdだった場合の処理
            "cd" => {
                // cmdのlenが2、もしくは2以上だった場合の処理
                if cmd.len() == 2 {
                    // cmdの二番目の要素に移動する
                    ps1.cd(&cmd[1].to_string(), &setup);
                } else {
                    // 2、2以上ではなかった際の処理
                    // homeを取得
                    let home = home_dir().unwrap();
                    // 二番目の要素が無かった場合、homeに戻る
                    ps1.cd(&home.to_str().unwrap().to_string(), &setup);
                }
                return "".to_string();
            }
            // exitだった場合の処理
            "exit" => {
                println!("exit");
                return "exit".to_string();
            }
            // 空白無視
            "" => return "".to_string(),
            // lsだった場合の処理
            "ls" => {
                // lsコマンドを実行。
                if let Ok(o) = Command::new("ls")
                    .args(&cmd[1..])
                    // ls_configのフラグ
                    .arg(setup.ls_config)
                    // .output()をつけるとlsがpipeしたと認識し、ディレクトリやファイルが縦に表示されるので-Cをつけて強制的に横並びに表示する
                    .arg("-C")
                    .output()
                {
                    // utf8に解釈して日本語を表示できるようにする
                    let stdout = str::from_utf8(&o.stdout).unwrap();
                    // printlnだと表示されたあと余分に開業されるのでprintを使用
                    print!("{}", stdout);
                    io::stdout().flush().unwrap();
                }
                return "".to_string();
            }
            // これら以外は何もしない
            _ => {}
        }

        // 先程matchしたもの以外はここで実行される
        match Command::new(command)
            .args(&cmd[1..])
            // inheritを付けてインタラクティブに動作するようにする
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(mut o) => {
                o.wait().unwrap();
            }
            Err(e) => {
                // コマンドが見つからなかった際にエラーメッセージを出力する
                if e.kind() == io::ErrorKind::NotFound {
                    eprintln!("cocoa: {}: command not found", command);
                } else {
                    eprintln!("{}", e);
                }
            }
        }
    }
    "".to_string()
}
