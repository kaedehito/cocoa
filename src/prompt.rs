use crate::structs;
use colored::{self, Colorize};

// dirを実装
impl structs::dir {
    /// promptを返す。
    ///```text
    /// username@hostname now_dir$
    ///```
    /// がテンプレート。
    /// usernameがkaedeで、hostnameがarchlinux。カレントディレクトリがuserのhomeだとすると
    ///```text
    /// kaede@archlinux ~$
    ///```
    /// のように表示される
    pub fn display_ps1(&self) -> String {
        let format = format!(
            "{}{}{} {}{} ",
            self.user_name.cyan(),
            "@".green(),
            self.hostname.magenta(),
            self.now_dir.bright_red(),
            "$".bright_white()
        );
        format
    }
}
