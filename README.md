
Rust Alpha ShooterRust Alpha Shooter は、Rust言語とmacroquadエンジンで構築された、レトロスタイルの縦スクロールシューティングゲーム（STG）です。シンプルな操作性ながら、パーティクル演出や弾幕ボスバトルなど、本格的なアーケード体験を提供します。🚀 特徴スムーズなゲームプレイ: macroquadの非同期ループによる、高フレームレートで軽快な動作。ダイナミックな演出: 敵撃破時の爆発パーティクル、スクロールする星空、ボスの出現警告メッセージ。本格的なボス戦: スコアが1000点を超えると巨大ボスが登場。サイン波による移動と、全方位への弾幕攻撃を搭載。ゲーム状態管理: タイトル画面、プレイ、ゲームオーバー、クリア画面の完全なステートマシン実装。🕹 操作方法キーアクション矢印キー (←/→)自機の左右移動Spaceショットの発射 / ゲームスタートRタイトル画面に戻る（リザルト画面時）🛠 技術スタックLanguage: RustGame Engine: macroquad (v0.4以降対応)Architecture:GameState 列挙型によるシーン管理Vec<T> を活用したオブジェクトプール（弾丸、敵、パーティクルの管理）デルタタイム（get_frame_time）に基づく時間制御📦 ビルドと実行前提条件Rust (Cargo) がインストールされていること。実行手順Bash# リポジトリをクローン
git clone https://github.com/your-username/rust-alpha-shooter.git
cd rust-alpha-shooter

# 実行
cargo run --release
📝 ソースコードの構成データ構造: Bullet, Enemy, Particle 等の構造体でゲームオブジェクトを定義。背景システム: 100個の Star 構造体によるパララックス風の星空。当たり判定: ベクトルの距離計算（length()）を用いた円形衝突判定。無敵時間: 被弾後の invincible_timer による点滅処理。🎨 今後のロードマップ (TODO)[ ] 複数の武器パワーアップアイテム[ ] BGMおよび効果音の追加[ ] ステージ2以降の追加（敵パターンの多様化）⚖ ライセンスこのプロジェクトは MITライセンス の下で公開されています。
<img width="992" height="785" alt="スクリーンショット 2026-05-09 193116" src="https://github.com/user-attachments/assets/9093af7c-4b1b-4ae3-98c7-0fca7717f158" />
