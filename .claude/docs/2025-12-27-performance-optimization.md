# Performance Optimization Opportunities

調査日: 2025-12-27

## 概要

svt の高速化余地を調査した結果をまとめる。

## 既に実装済みの最適化

| 項目 | 説明 | 場所 |
|------|------|------|
| スレッド分離 | Worker/Writer/Main の3スレッド構成 | 全体 |
| 最新リクエスト優先 | `drain_to_latest()` で古いリクエスト破棄 | worker.rs |
| ナビゲーション遅延 | `nav_latch` で高速ナビ時の処理スキップ | main.rs |
| エポック管理 | 古い非同期タスクの無視 | app.rs, sender.rs |
| SIMD Base64 | `base64_simd` クレート使用 | kgp.rs |
| 矩形差分計算 | dirty area 管理 | sender.rs |
| Zlib 圧縮 | KGP 転送データの圧縮 | kgp.rs |
| DynamicImage Arc 化 | デコード画像の参照カウント共有 | worker.rs |
| encoded_chunks Arc 化 | エンコード済みデータの参照カウント共有 | worker.rs, app.rs, sender.rs |
| HashMap キャッシュ | `render_cache` を HashMap + VecDeque (LRU) に変更 | app.rs |
| Tile 合成の並列化 | rayon で並列デコード・リサイズ | worker.rs |
| Resize フィルタ設定 | Single: `resize_filter` (default: Triangle), Tile: `tile_filter` (default: Nearest) | worker.rs, config.rs |
| Tile サムネイルキャッシュ | LRU キャッシュ (500 エントリ) でサムネイル再利用 | worker.rs |

---

## 保留: 効果限定的

### 3. as_raw().clone() の削減

**現状の問題:**
```rust
// kgp.rs:140-142
(v.as_raw().clone(), 24)  // ピクセルデータ全コピー
```

**改善案:**
- 借用で処理できる場合は Cow 使用
- Zlib 圧縮時は避けられないが、非圧縮時は借用可能

**効果:** メモリ削減
**複雑度:** 中
**状態:** 保留（実装複雑で効果限定的）

---

## 未実装: 小改善

### 8. terminal::size() 呼び出し統合

**現状の問題:**
- main.rs で 5 箇所呼び出し (L140, 180, 322, 339, 384)

**改善案:**
- ループ冒頭で 1 回取得して変数に保持

**効果:** 微小
**複雑度:** 低

---

### 9. base64 encode_to_vec 使用

**現状の問題:**
```rust
// kgp.rs:154
base64_simd::STANDARD.encode_to_string(&data).into_bytes()
```

**改善案:**
```rust
base64_simd::STANDARD.encode_to_vec(&data)
```

**効果:** 微小（String 経由の変換を削減）
**複雑度:** 低

---

### 10. status キャッシュキー保持

**現状の問題:**
- 毎 tick で render_cache を線形走査して status を計算

**改善案:**
- 直近のキャッシュキーを保持して、変更がなければ再計算スキップ

**効果:** 微小
**複雑度:** 低

---

## 推奨実装順序

### 完了 ✓
1. #1 DynamicImage Arc 化
2. #2 encoded_chunks Arc 化
3. #5 HashMap キャッシュ
4. #6 Tile 合成の並列化 (rayon)
5. #7 Tile 高速フィルタ (+ Single モード対応)
6. #4 Tile サムネイルキャッシュ

### 次のステップ
7. #8, #9, #10 の小改善

### 保留
- #3: 実装複雑で効果限定的
