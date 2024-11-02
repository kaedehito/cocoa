use crate::structs;
use colored::{self, Colorize};

// dir構造体の実装
impl structs::Dir {
    /// プロンプト文字列を生成して返す関数。
    /// テンプレート形式は次の通り：
    /// ```
    /// username@hostname now_dir$
    /// ```
    /// 例: usernameが`kaede`、hostnameが`archlinux`で、カレントディレクトリがホームディレクトリの場合：
    /// ```
    /// kaede@archlinux ~$
    /// ```
    /// カラー表示を行うため、ユーザー名やホスト名、ディレクトリ名には異なる色を適用。
    pub fn display_ps1(&self) -> String {
        // プロンプトのフォーマットを作成し、それぞれの要素に色を適用
        let format = format!(
            "{}{}{} {}{} ",
            self.user_name.cyan(),     // ユーザー名をシアンにカラーリング
            "@".green(),               // @を緑色に
            self.hostname.magenta(),   // ホスト名をマゼンタに
            self.now_dir.bright_red(), // カレントディレクトリを明るい赤色に
            "$".bright_white()         // プロンプトの終端`$`を明るい白色に
        );
        format
    }
}
