# ESP32向けrust

## 概要

Espressifが公開している[ESP32/ESP8266向けのLLVM](https://github.com/espressif/llvm-xtensa)をバックエンドとして使えるようにしたRustのソースコード

## 変更内容

### LLVMの変更

Rust 1.28.0に対応するリビジョン([1.28.0タグ](https://github.com/ciniml/rust/commit/9634041f0e8c0f3191d2867311276f19d0a42564))のsrc/llvmで使用している[リビジョン](https://github.com/ciniml/rust-llvm-xtensa/commit/1c817c7a0c828b8fc8e8e462afbe5db41c7052d1)
にEspressifのLLVMの[esp-develop](https://github.com/ciniml/rust-llvm-xtensa/commit/06445bd6203b8aeedc8a83e27c37ed17ad620d18)ブランチをマージしています。
[マージしたリビジョン](https://github.com/ciniml/rust-llvm-xtensa/commit/1978d12092e4bbcd33f5204d6d4922f90063e1b1)

一部ソースコード中のコメントなどが衝突しましたが、特に問題なくマージできます。

### librustc_llvmの変更

LLVMのバックエンドの初期化処理が`librustc_llvm`crateに記述されています。LLVMのバックエンドの初期化処理はターゲットごとに分かれているため、Xtensaのバックエンドの初期化処理を追加する必要があります。
また、`librustc_llvm`のビルド用コードにもXtensa用のバックエンドをリンクするように処理を追加する必要があります。

`src/librustc_llvm/build.rs`の`optional_components`に`xtensa`を追加してXtensaバックエンドをリンクします。[コード](https://github.com/ciniml/rust/commit/376c28f6d694abd740c6fe8b9f7f55f3d4658227#diff-b0767d8366aa7e0a83a847d53032dfd4R85)

`src/librustc_llvm/lib.rs`の`initialize_targets`関数にXtensaバックエンド初期化関数を呼び出すコードを追加します。[コード](https://github.com/ciniml/rust/commit/376c28f6d694abd740c6fe8b9f7f55f3d4658227#diff-85aa9e626e803ce4f4f28165e437040bR383)

### librustc_targetにターゲットを追加

#### ABIの追加

librustc_targetにはターゲットのABI情報が含まれています。ターゲットのABI情報が見つからないと、ABIが分からないというエラーでrustcが停止します。

ABI情報は基本的に各ターゲットごとに`compute_abi_info`関数を実装し、指定された関数情報に対して適切なABIを選択して返すようにします。
とりあえずMSP430向けの実装をベースにレジスタ幅が32bitであることを前提に変更したものをXtensaのABI情報として追加します。 [コード](https://github.com/ciniml/rust/commit/376c28f6d694abd740c6fe8b9f7f55f3d4658227#diff-63ab7b66afab3dabdce3f50b2cc643c9)
ファイルは`src/librustc_target/abi/call/xtensa.rs`です。

ターゲットごとの`compute_abi_info`は`src/librustc_target/abi/call/mod.rs`の`adjust_for_cabi`関数から呼び出されているので、Xtensa用の実装も他のターゲット同様に呼び出されるようにします。 [コード](https://github.com/ciniml/rust/commit/376c28f6d694abd740c6fe8b9f7f55f3d4658227#diff-ca7ebaf48755bce9c4e33a1c2d2b9e52R502)

#### ターゲットの定義の追加

ターゲット定義は`--target`オプションを使ってJSON形式のファイルとして後から与えることも出来ますが、いずれにせよrustcをビルドするので、ターゲット定義も組み込んでしまいます。
ファイルは`src/librustc_target/spec/xtensa_esp32_elf.rs`です。

ターゲット定義の中で重要かつ難しい部分が、`data_layout`の内容です。この部分はLLVMの型が実際にどのようなレイアウトとなるのかを定義します。
ESP32用の定義 `e-m:e-p:32:32-i1:8:32-i8:8:32-i16:16:32-i64:64-f64:64-a:0:32-n32`  は[Xtensa用LLVMのソースコード](https://github.com/espressif/llvm-xtensa/blob/esp-develop/lib/Target/Xtensa/XtensaTargetMachine.cpp#L37)に含まれているので、そこから抜き出してきます。

現在のXtensa用LLVMバックエンドは直接オブジェクト・ファイルを出力できないので、アセンブリ言語のファイルを出力し、gccを使ってオブジェクト・ファイルを出力するように指定します。
`no_integrated_as`を`true`に設定し、`linker`に`xtensa-esp32-elf-gcc`を指定します。[コード](https://github.com/ciniml/rust/commit/376c28f6d694abd740c6fe8b9f7f55f3d4658227#diff-5dc2d1c868898cb6abb8104a14c94f76R37)

定義を記述したら、`src/librustc_target/spec/mod.rs` の[組み込みターゲット定義リストに追加](https://github.com/ciniml/rust/commit/376c28f6d694abd740c6fe8b9f7f55f3d4658227#diff-45afaeaca1fcebbc65b0315c52bdf799R371)します。

## ビルド

`config.toml.example` を `config.toml` として、必要に応じて `prefix` を編集してインストール先を変更します。

その後、`./x.py install` を実行すると `prefix` で指定したパスにrustcがインストールされます。

`rustup toolchain link xtensa-1.28.0 /path/to/prefix` として、rustupに登録しておくと便利です。
