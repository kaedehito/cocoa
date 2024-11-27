use crate::{load_setup, structs::Dir};
use dirs::home_dir;
use std::{env, io};

impl Dir {
    pub fn cd(&mut self, dir: &String, setup: &load_setup::Config) {
        // 現在のカレントディレクトリを取得
        let home_dir = home_dir().unwrap();
        let home_dir_str = home_dir.to_str().unwrap();

        // 指定されたディレクトリに移動
        if let Err(e) = env::set_current_dir(dir) {
            if e.kind() == io::ErrorKind::NotFound {
                eprintln!("cocoa: cd: {}: {}", dir, setup.cd_error_message);
            } else {
                eprintln!("cocoa: cd: {}: {}", dir, e);
            }
            return; // エラーが発生した場合は関数を終了
        }

        // カレントディレクトリを再取得
        let current_dir = env::current_dir().unwrap();
        let current_dir_str = current_dir.to_str().unwrap();

        // プロンプトを生成
        self.now_dir = if current_dir_str.starts_with(home_dir_str) {
            current_dir_str.replacen(home_dir_str, "~", 1)
        } else {
            current_dir_str.to_string()
        };
    }
}
