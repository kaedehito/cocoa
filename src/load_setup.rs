use dirs::home_dir;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};
use toml::de::Error;

#[derive(Deserialize, Clone, Debug)]
/// tomlの取得するデータ
///```toml
/// start_up_text = ""
/// ls_cofig = ""
/// cd_error_message = ""
///
///```
pub struct Config {
    pub start_up_text: String,
    pub ls_config: String,
    pub cd_error_message: String,
    // pub alias: Aliases,
}
#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Aliases {
    pub alias: HashMap<String, String>,
}

/// .cocoa_rcを読み込み、tomlをパースする。
/// Result型を返すためunwrapかif let Errが必要。
/// Configを返す。Configのフィールドはすべてpubになっているので自由に使える
pub fn setup() -> Result<Config, Error> {
    // homeを取得。取得できなかった場合panicを起こす
    let home = home_dir().unwrap();

    // .cocoa_rcのpath
    let config_file = format!("{}/.cocoa_rc", home.display());
    // Path型に変換
    let path = Path::new(&config_file);

    if !path.exists() {
        // .cocoa_rcが存在しなかった場合、以下の構成を返す.
        return Ok(Config {
            start_up_text: "welcome to cocoa".to_string(),
            ls_config: "--color=always".to_string(),
            cd_error_message: "No such file or directory".to_string(),
        });
    }

    // ファイルの読み込み
    let config_content = fs::read_to_string(path).expect("ファイルが見つかりません");

    // TOML文字列を構造体にパース
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}
