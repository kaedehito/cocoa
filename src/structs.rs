#[allow(non_camel_case_types)]
/// 現在の状態を構成する構造体
/// now_dir、hostname、user_nameで構成されている
/// now_dirには現在のカレントディレクトリ、hostnameには使用しているPCのhostname。
/// user_nameには使用しているユーザー名が入る
pub struct dir {
    pub now_dir: String,
    pub hostname: String,
    pub user_name: String,
}
