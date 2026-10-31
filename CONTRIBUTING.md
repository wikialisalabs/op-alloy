# Hướng dẫn Đóng góp — `op-alloy`

**`op-alloy`** là lớp tích hợp OP Stack — mọi thay đổi cần được verified với OP Stack specification chính thức.

---

## Mục lục

1. [Môi trường Phát triển](#1-môi-trường-phát-triển)
2. [Cấu trúc Repository](#2-cấu-trúc-repository)
3. [Quy trình Đóng góp](#3-quy-trình-đóng-góp)
4. [Thêm OP Stack Upgrade](#4-thêm-op-stack-upgrade)
5. [Viết Tests](#5-viết-tests)
6. [Commit Convention](#6-commit-convention)
7. [Pull Request Checklist](#7-pull-request-checklist)

---

## 1. Môi trường Phát triển

```bash
git clone https://github.com/wikialisalabs/op-alloy.git
cd op-alloy
rustup show
cargo build
cargo test
```

---

## 2. Cấu trúc Repository

```
op-alloy/
├── src/
│   ├── lib.rs            # Crate root
│   ├── deposit.rs        # DepositTransaction (type 0x7E)
│   ├── l2_block.rs       # L2BlockHeader
│   └── rollup_config.rs  # RollupConfig, sequentichain_config()
├── Cargo.toml
├── README.md
└── CONTRIBUTING.md
```

---

## 3. Quy trình Đóng góp

1. Mọi thay đổi liên quan đến encoding format phải link tới OP Stack spec: https://specs.optimism.io
2. Fork → branch `feat/<tên>`.
3. Implement + tests.
4. CI local pass.
5. Mở PR.

---

## 4. Thêm OP Stack Upgrade

Khi OP Stack ra hardfork mới (e.g. Granite, Holocene):

1. Thêm field mới vào `RollupConfig` trong `src/rollup_config.rs`:
   ```rust
   pub holocene_time: Option<u64>,
   ```
2. Thêm activation method:
   ```rust
   pub fn is_holocene_active(&self, timestamp: u64) -> bool {
       self.holocene_time.map_or(false, |t| timestamp >= t)
   }
   ```
3. Cập nhật `sequentichain_config()` với timestamp thực tế (hoặc `None` nếu chưa schedule).
4. Viết tests.

---

## 5. Viết Tests

```bash
cargo test
cargo test --all-features
```

### Yêu cầu tối thiểu

| Thay đổi | Tests bắt buộc |
|----------|----------------|
| DepositTransaction | Encode type byte, encode determinism |
| RollupConfig field | Active at activation time, inactive before |
| L2BlockHeader | Hash determinism |

---

## 6. Commit Convention

```
feat(op-alloy): add Holocene upgrade to RollupConfig
feat(deposit): add deposit_nonce to DepositReceipt encoding
fix(rollup): correct sequentichain ecotone timestamp
test(deposit): add system transaction encode test
```

---

## 7. Pull Request Checklist

```
[ ] cargo fmt --all                          → no diff
[ ] cargo clippy --all-features -- -D warnings → 0 warnings
[ ] cargo test --all-features                → all pass
[ ] OP Stack spec linked for encoding changes ✓
[ ] Activation timestamps from official source ✓
[ ] PR linked to Issue                        ✓
```

---

## Liên hệ

- **Email:** root@wikialisalabs.axvn.vn
- **Issues:** https://github.com/wikialisalabs/op-alloy/issues
- **OP Stack Specs:** https://specs.optimism.io

*Cảm ơn bạn đã đóng góp! 🦀🔴*
