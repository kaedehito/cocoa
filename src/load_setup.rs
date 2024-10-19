use dirs::home_dir;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};
use toml::de::Error;

#[derive(Deserialize, Clone, Debug)]
/// TOMLから取得する設定データを表す構造体
///```toml
/// start_up_text = ""
/// ls_config = ""
/// cd_error_message = ""
///```
pub struct Config {
    pub start_up_text: String,    // 起動時に表示するテキスト
    pub ls_config: String,        // lsコマンドの設定オプション
    pub cd_error_message: String, // cdコマンドのエラーメッセージ
    // pub alias: Aliases,        // コメントアウトされているが、将来的にaliasの機能を追加予定
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Aliases {
    pub alias: HashMap<String, String>, // コマンドのエイリアスを格納
}

/// .cocoa_rcファイルを読み込み、TOMLを解析してConfig構造体に変換する関数
/// この関数はResultを返すため、呼び出し側でunwrapやエラーハンドリングが必要
/// Configを返すため、そのフィールドは自由に使用できる
pub fn setup() -> Result<Config, Error> {
    // ホームディレクトリを取得。取得できない場合はpanicを発生させる
    let home = home_dir().unwrap();

    // .cocoa_rcファイルのパスを作成
    let config_file = format!("{}/.cocoa_rc", home.display());

    // パスをPath型に変換
    let path = Path::new(&config_file);

    // .cocoa_rcが存在しない場合、デフォルトの設定を返す
    if !path.exists() {
        return Ok(Config {
            start_up_text: "welcome to cocoa".to_string(),              // デフォルトの起動メッセージ
            ls_config: "--color=always".to_string(),                    // lsのデフォルト設定
            cd_error_message: "No such file or directory".to_string(),  // デフォルトのcdエラーメッセージ
        });
    }

    // .cocoa_rcファイルの内容を読み込む。ファイルが見つからない場合はpanicを発生
    let config_content = fs::read_to_string(path).expect("ファイルが見つかりません");

    // TOML文字列をConfig構造体に変換して返す
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}