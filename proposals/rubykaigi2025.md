# RubyKaigi 2025 Proposal

## title:

Towards the possibility of implementing "componentize-rb"

## Abstract

<!-- 

Create an abstract of talk in about 1000 letters of English, following below:

- title:
  Towards the possibility of implementing "componentize-rb"
- toc:
  - WASMのエコシステムではComponent Modelの実装が重要視されている。
  - 筆者はmruby/edgeという、RubyのスクリプトをWASMプログラムにコンパイルするためのプロジェクトを進めている。
  - 今回、mruby/edgeをComponent Modelに対応させるため、いくつかのアプローチを検証し、実施した。
  - それらの方法について紹介しつつ、RubyのあるべきComponent Modelの形を議論する。

-->

In the current WebAssembly (WASM) ecosystem, the implementation of the Component Model is gaining significant attention. This proposal explores the potential of implementing "componentize-rb," a concept aimed at enhancing the modularity and reusability of Ruby code within the WASM environment. The author has been working on a project called mruby/edge, which compiles Ruby scripts into WASM programs. This talk will delve into various approaches tested and implemented to make mruby/edge compatible with the Component Model. By examining these methods, the presentation will discuss the ideal form of a Component Model for Ruby, providing insights and fostering discussions on how Ruby can evolve in the context of WASM's growing ecosystem.

## Pitch

以下のような内容を含むプレゼンテーションを行います。

- WASMのエコシステムにおけるComponent Modelの重要性
    - WASM Coreでのプログラム実行の概要
        - 問題となる点
            - プログラムの再利用が意外と大変
                - RPCの際、インタフェースの情報が十分でない
                - 構造体のような複雑な形をエクスポートできない
                - 特に文字列を扱いづらい
            - 端的に言えば、現状CのDLLを呼び出す程度の信頼度しかない
        - 「プログラムの再利用」の問題が解決すれば、WASMの言語agnosticな特性が大きく活かせるようになる
            - コアロジックはRuby、画像処理はRust、API通信はGo、といったように、言語ごとに得意な部分を活かすことができる
    - Component Modelの概要と課題へのアプローチ
        - RPCに近い発想のの堅牢なインタフェース
            - インタフェースのディスカバリや自動生成と相性がいい
            - cf. gRPC/protobuf
    - Component Modelの実装例
        - （Rust、C++他、Moonbitも紹介したい）
    - Pythonにおけるcomponetize-pyの話を強調する
        - Pythonに負けたくないですよね（？
- mruby/edgeの去年からのrecap
    - mruby/edgeの概要を改めて紹介
    - mruby/edgeのVMリライトと新しくできるようになったこと
        - インスタンスやクラスの定義
        - 他、ブロック実行など、TODO: 当日までになるべく作る
- Component Model に対応する、とは？
    - Component Modelに関する概念の整理
        - WIT
        - WASI preview 2
    - Rustでの「素直な」実装
        - wit-bindgenベースでComponentを作る
        - Pure Rustであるmruby/edgeを組み込むには？
    - componentize_any について
        - Component Modelのバイナリ仕様の分析
        - Core WASMをComponentにするハック
    - それぞれのメリデメ整理
    - mruby/edgeをComponent Modelのエコシステムに組み込んだデモ
        - wasmCloudとmruby/edgeのアプリケーションの連携を動かす
- 終わりに: （mrubyやCRubyが）Component Model に対応する、とは？
    - それぞれの言語について考えられるアプローチを整理
    - C資産のComponetizeの手段を整理
    - いくつかの提案