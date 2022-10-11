# 🦀を持つcmusプレーヤーのためのDiscord Rich Presence

[![crates.io](https://img.shields.io/crates/v/cmus-rpc-rs?style=for-the-badge)](https://crates.io/crates/cmus-rpc-rs)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/3e0d24aa2c1441e484622b8540193cdf)](https://app.codacy.com/gh/anas-elgarhy/cmus-rpc-rs?utm_source=github.com&utm_medium=referral&utm_content=Anas-Elgarhy/cmus-rpc&utm_campaign=Badge_Grade_Settings)
[![CodeFactor](https://www.codefactor.io/repository/github/anas-elgarhy/cmus-rpc-rs/badge)](https://www.codefactor.io/repository/github/anas-elgarhy/cmus-rpc)

<img alt="image 1" src="../Screenshots/1_0.1.0.png">
<img alt="image 2" src="../Screenshots/2_0.1.0.png">

- 必要とする cmus

## インストール

- から crates.io
    ```bash
    cargo install cmus-rpc-rs 
    ```
- から Arch User Repository(AUR): `yay -S cmus-rpc-rs`


### オプション:

| オプション                       | 説明                                                  | 値                                                                                    |
| ---------------------------- | ------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| `-h` or `--help`             | ヘルプを表示                                                | -                                                                                         |
| `-V` or `--version`          | バージョンを表示                                             | -                                                                                         |
| `-d` or `--debug`            | デバッグモード                                                   | -                                                                                         |
| `-l` or `--link`             | cmusとの連携（cmusが起動していない場合はプログラムを閉じてください） | -                                                                                         |
| `-c` or `--config`           | カスタム パスを構成ファイルに設定する                               | 設定ファイルへのパス .json                                                                 |
| `-i` or `--interval`         | チェック間隔の設定                                  | 間隔時間 (秒)                                                                   |
| `-s` or `--sleep`            | アクティビティがないときにスリープを設定する                          | スリープ時間 (秒)                                                                      |
| `--p1f` or `--partOneFormat` | 最初の部分の書式を設定する                            | 最初の部分のフォーマット                                                                     |
| `--p2f` or `--partTowFormat` | 2 番目の部分の書式を設定する                           | 第 2 部の形式                                                                    |
| `--li` or `--largeImage`     | プレゼンス用のカスタム大きな画像（表紙）を設定する                  | 大きな画像名 [デフォルトアプリで利用可能な画像](./assets/cover/)                       |
| `--pi` or `--playingImage`   | プレゼンスのカスタム再生イメージを設定する                        | 再生アイコン名 [デフォルトアプリで利用可能なプレイ画像](./assets/play_icons/)         |
| `--pai` or `--pausedImage`   | プレゼンス用のカスタム一時停止画像を設定する                         | 一時停止中のアイコン名[利用可能なアイコン](./assets/pause_icons/)                                 |
| `--pt` or `--playingText`    | プレゼンスのカスタム再生アイコン ALT を設定する                     | 再生アイコンの代替テキスト                                                                     |
| `--pat` or `--pausedText`    | プレゼンスのカスタム一時停止アイコンの代替を設定する              | 一時停止アイコンの代替テキスト                                                                      |
| `--b1t` or `--buttonOneText` | ボタン 1 テキスト (ラベル) を設定                                   | ボタン 1 つのラベル (空の場合、構成ファイルに値がない場合、ボタンは非表示になります) |
| `--b1u` or `--buttonOneUrl`  | ボタン 1 の URL を設定                                           | ボタン 1 の URL (構成ファイルに値がなく、空の場合、ボタンは非表示になります)   |
| `--b2t` or `--buttonTwoText` | ボタン 2 のテキスト (ラベル) を設定                                   | ボタン 1 つのラベル (空の場合、構成ファイルに値がない場合、ボタンは非表示になります) |
| `--b2u` or `--buttonTwoUrl`  | ボタン 2 の URL を設定                                           | ボタン 1 の URL (構成ファイルに値がなく、空の場合、ボタンは非表示になります)   |

### 例:

```bash
cmus-rpc-rs --p1f %title%
```

```bash
cmus-rpc-rs --p1f "%artist% - %title%" --p2f "%album% - %date%"
```

```bash
cmus-rpc-rs --p1f "Anas listening to %title%" --p2f "From %artist%"
```

### cmus 起動時に自動実行する方法

- 次の行を shellrc ファイルに追加します。 `.bashrc` または `.zshrc`

```bash
    alias cmus = 'cmus-rpc-rs --link &>/dev/null & cmus'
```

### で利用可能

[![GitHub](https://img.shields.io/badge/GitHub-Main%20repo-brightgreen?style=for-the-badge&logo=GitHub)](https://github.com/anas-elgarhy/cmus-rpc-rs)
[![GitLab](https://img.shields.io/badge/GitLab-Mirror%20repo-brightgreen?style=for-the-badge&logo=GitLab)](https://gitlab.com/anas-elgarhy/cmus-rpc-rs)
[![BitBucket](https://img.shields.io/badge/BitBucket-Mirror%20repo-brightgreen?style=for-the-badge&logo=BitBucket)](https://bitbucket.org/anas_elgarhy/cmus-rpc-rs)
[![Codeberg](https://img.shields.io/badge/Codeberg-Mirror%20repo-brightgreen?style=for-the-badge&logo=Codeberg)](https://codeberg.org/anas-elgarhy/cmus-rpc-rs)

### 手段

- [`cmus-remote` tool](https://github.com/cmus/cmus) cmusに問い合わせる
- [Discord Rich Presence](https://github.com/nickofolas/discord-rich-presence) Discord の IPC と接続するためのシンプルなクロスプラットフォーム クレートです。
- [dirs-rs](https://github.com/dirs-dev/dirs-rs) Linux、macOS、および Windows のそれぞれの規則に従って、config/cache/data パスを提供する低レベル ライブラリ。
- [clap](https://github.com/clap-rs/clap) Rust 用のフル機能の高速コマンド ライン引数パーサー。

[![Quality gate](https://sonarcloud.io/api/project_badges/quality_gate?project=anas-elgarhy_cmus-rpc-rs)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)

[![SonarCloud](https://sonarcloud.io/images/project_badges/sonarcloud-black.svg)](https://sonarcloud.io/summary/new_code?id=anas-elgarhy_cmus-rpc-rs)

> これは錆を使った私の最初のプロジェクトです 😆, star yoo で私をサポートしてください 💙🦀

[![License MIT](https://img.shields.io/badge/license-MIT-green.svg)](https://spdx.org/licenses/MIT.html)
