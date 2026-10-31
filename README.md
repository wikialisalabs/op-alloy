# op-alloy

> **Tầng 3 — Ecosystem Extensions** · OP Stack native types cho SEQUENTICHAIN

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Tổng quan

`op-alloy` cung cấp các kiểu dữ liệu và logic đặc thù OP Stack cho **SEQUENTICHAIN** — L2 network của Wikialisalabs Foundation.

| Module | Nội dung |
|--------|----------|
| `deposit` | `DepositTransaction` (type 0x7E), `DepositReceipt` |
| `l2_block` | `L2BlockHeader` với L1 origin tracking |
| `rollup_config` | `RollupConfig`, `sequentichain_config()` |

---

## Cài đặt

```toml
[dependencies]
op-alloy = { git = "https://github.com/wikialisalabs/op-alloy" }
```

## Sử dụng nhanh

```rust
use op_alloy::deposit::DepositTransaction;
use op_alloy::rollup_config::sequentichain_config;

// Kiểm tra hardfork
let cfg = sequentichain_config();
println!("Ecotone active: {}", cfg.is_ecotone_active(1_800_000_000));

// Mã hoá deposit tx
let tx = DepositTransaction {
    source_hash: [0u8; 32],
    from: [0x01u8; 20],
    to: Some([0x02u8; 20]),
    mint: [0u8; 32],
    value: [0u8; 32],
    gas: 21_000,
    is_system_tx: false,
    data: vec![],
};
let encoded = tx.encode();
assert_eq!(encoded[0], 0x7E); // type byte
```

---

## Kiểm thử

```bash
cargo test
cargo test --all-features
```

---

## Đóng góp

Xem [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Giấy phép

MIT © 2026 Wikialisalabs Foundation
