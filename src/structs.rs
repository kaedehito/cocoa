#[allow(non_camel_case_types)]
/// シェルの現在の状態を表す構造体。
/// 構成要素は`now_dir`（カレントディレクトリ）、`hostname`（ホスト名）、`user_name`（ユーザー名）。
/// - `now_dir`: 現在のカレントディレクトリのパスを格納する。
/// - `hostname`: 使用中のPCのホスト名を格納する。
/// - `user_name`: 現在のユーザー名を格納する。
pub struct dir {
    pub now_dir: String,
    pub hostname: String,
    pub user_name: String,
}