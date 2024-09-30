# PS88

<p align="center">
<img src="./logo.svg" width="240" alt="PS88 logo">
</p>

Programmable Synthesizer 88

JavaScript で波形を生成できるシンセサイザー。

CLAP / VST3 として動作するため、作曲ソフトから利用できる。

TODO: ここにデモの gif を載せる

```js
const audio = (ctx) => {
  // TODO: 鍵盤を押すと正弦波が鳴る、みたいな簡単なサンプルを書く
}
```

# 使い方

TODO: 書く

# ビルド 

## Windows / MacOS / Linux

```
cargo install --git https://github.com/robbert-vdh/nih-plug --rev f43fd5ccd6521227558d1a3949b612690f225123 xtask
xtask bundle ps88 --release
```

実行すると `target/bundled/` に以下が生成される。

- `ps88.clap`
    - clap ファイル
- `ps88.vst3`
    - vst3 ファイル
- `ps88` or `ps88.exe`
    - 単独実行可能な実行ファイル
    - `ps88 -h` で使い方を表示できる

# やりたいことリスト

- 細々した機能
    - [ ] 画面のデザインを考える
    - [ ] 画面遷移の図を作る
    - [ ] js で任意のデータをプラグインのホスト側のストレージに保存/読込できるようにする
    - [ ] 開いている js ファイルのパスを記憶し、次開いた時にファイルがあればそれを読むようにする
        - ファイルが存在しなかったとしても特にエラーにはせず、パスの参照を解除するのみ
    - [ ] js から envelope を読み取れるようにする
        - envelope は 4 つ固定とする
        - js 側から任意の envelope を設定できるようにすることもできそうだけど、複雑になりそう
    - [ ] js をもっと使いやすい感じに整える
        - 鍵盤押したらその音の正弦波が鳴る、みたいなコードは 1 行ぐらいで書けるようにしたい
        - 事前に読み込まれるライブラリ的なものを用意してもいいかも
            - MIDI イベントを元に鍵盤の状態を管理してくれるやつとか
            - ノコギリ波を生成してくれるやつとか
    - [ ] js で UI を構築できるようにする
        - [ ] N 角形の線、面を描画できる
        - [ ] テキストを描画できる
        - [ ] マウスイベントを受け取ることができる
        - [ ] ボタンやつまみ、グラフ表示などの標準ライブラリを用意する
    - [ ] console.log が画面に出るようにする
    - [ ] エラーメッセージが画面に出るようにする
    - [ ] 何らかのオプションでログをテキストファイルにも出力されるようにしたい
    - [ ] エラー周りの整理
        - [ ] 一度コンパイルエラー/ランタイムエラーになったら js が変更されるまで実行しないようにする
        - [ ] unwrap はパニックの元なので、適切にエラーハンドリングされるようにする
        - [ ] コードの一番大元のところで panic をキャッチして、そのトレースが画面上で確認できるようにする
            - Rust には [`panic::set_hook`](https://doc.rust-lang.org/std/panic/struct.PanicInfo.html#method.location) という仕組みがあり、 panic 時に任意の処理を実行できるらしい
    - [ ] `ps88/runtime/*` がごちゃついてきたのでリファクタリング
    - [ ] js の高度なデバッガー機能
        - [ ] ブレークポイント/ステップ実行
        - [ ] Chrome のパフォーマンスプロファイラみたいなツール
    - [ ] テスト拡充
    - [ ] GitHub Actions で CI 構築
    - [ ] チュートリアルのページを作る
- その他
    - [ ] 依存ライブラリを更新する
        - [ ] rusty\_v8
        - [ ] nih-plug
    - [ ] GUI のレンダリングを opengl から wgpu に切り替える
        - https://github.com/BillyDM/egui-baseview/pull/18 が nih-plug に取り込まれれば簡単に実現できそう
    - [ ] GUI 上に js エディタを置く
        - 現時点では MacOS の Reaper 上からだとキー入力を受け付けられないため、それを直す必要がある
        - https://github.com/RustAudio/baseview/issues/169 で解決される可能性はある
- 非常に大変な目標
    - [ ] 楽器の共有サイトを作る
        - イメージとしては Unity Hub みたいなもの
        - 機能
            - 楽器やエフェクトの投稿
            - 楽器やエフェクトの検索
            - いいね、ブックマーク機能
            - ブラウザ上での試奏機能
            - ブラウザ上での簡易的な MIDI 編集機能
            - 投稿した作品のサンプルとして再生する MIDI の登録
                - 動画で言うところのサムネイルみたいなイメージ
            - ユーザ登録
            - 作品のプラグインへのインポート
    - [ ] プロモーションビデオを作る
    - [ ] PS88 でお金を稼ぐ
    - [ ] PS88 で生計を立てる
