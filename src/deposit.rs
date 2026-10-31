//! OP Stack deposit transaction (EIP-2718 type 0x7E).
//!
//! Deposits are L1 → L2 messages initiated by the `OptimismPortal` contract.

/// The transaction type byte for OP Stack deposits.
pub const DEPOSIT_TX_TYPE: u8 = 0x7E;

/// An OP Stack deposit transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepositTransaction {
    /// Source hash: uniquely identifies the deposit origin.
    pub source_hash: [u8; 32],
    /// From address (20 bytes) — the depositing account on L1.
    pub from: [u8; 20],
    /// To address (20 bytes) — target on L2 (None for contract creation).
    pub to: Option<[u8; 20]>,
    /// Mint value (ETH minted on L2, in wei — 32 bytes big-endian).
    pub mint: [u8; 32],
    /// Value transferred (wei — 32 bytes big-endian).
    pub value: [u8; 32],
    /// Gas limit.
    pub gas: u64,
    /// Whether this is a system transaction.
    pub is_system_tx: bool,
    /// Call data.
    pub data: Vec<u8>,
}

impl DepositTransaction {
    /// Encode this deposit tx to the EIP-2718 envelope format.
    ///
    /// `0x7E || RLP([source_hash, from, to, mint, value, gas, is_system_tx, data])`
    pub fn encode(&self) -> Vec<u8> {
        let mut out = vec![DEPOSIT_TX_TYPE];
        // Stub: append field lengths as a simple length-prefixed encoding
        out.extend_from_slice(&self.source_hash);
        out.extend_from_slice(&self.from);
        if let Some(to) = &self.to {
            out.push(1);
            out.extend_from_slice(to);
        } else {
            out.push(0);
        }
        out.extend_from_slice(&self.mint);
        out.extend_from_slice(&self.value);
        out.extend_from_slice(&self.gas.to_be_bytes());
        out.push(self.is_system_tx as u8);
        out.extend_from_slice(&self.data);
        out
    }
}

/// A deposit receipt — the L2 outcome of processing a deposit tx.
#[derive(Debug, Clone)]
pub struct DepositReceipt {
    /// Whether the deposit was executed successfully on L2.
    pub success: bool,
    /// Gas used.
    pub gas_used: u64,
    /// Deposit nonce (monotonically increasing per depositor).
    pub deposit_nonce: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_deposit() -> DepositTransaction {
        DepositTransaction {
            source_hash: [0xabu8; 32],
            from: [0x01u8; 20],
            to: Some([0x02u8; 20]),
            mint: [0u8; 32],
            value: [0u8; 32],
            gas: 21_000,
            is_system_tx: false,
            data: vec![],
        }
    }

    #[test]
    fn encode_starts_with_type_byte() {
        let encoded = sample_deposit().encode();
        assert_eq!(encoded[0], DEPOSIT_TX_TYPE);
    }

    #[test]
    fn encode_deterministic() {
        let tx = sample_deposit();
        assert_eq!(tx.encode(), tx.encode());
    }
}
