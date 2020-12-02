# Bookmark-Directory

## Install手順
1. .zshrcに下記を追加する。
```
DATABASE_URL="$HOME/bd.db"
 eval "$(bookmark-directory init)"
```

2. プロジェクトルートで以下を実行する。
```
cargo build --release
```

3. バイナリファイルを以下に配置する。
```
cp ./target/release/bookmark-directory ~/.cargo/bin
```