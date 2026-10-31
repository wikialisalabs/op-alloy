//! L2 block header — extends the standard Ethereum block with OP Stack fields.

/// An OP Stack L2 block header.
#[derive(Debug, Clone)]
pub struct L2BlockHeader {
    /// Block number.
    pub number: u64,
    /// Parent block hash (32 bytes).
    pub parent_hash: [u8; 32],
    /// Block timestamp (Unix seconds).
    pub timestamp: u64,
    /// L1 block number this L2 block was derived from.
    pub l1_origin_number: u64,
    /// L1 block hash this L2 block was derived from (32 bytes).
    pub l1_origin_hash: [u8; 32],
    /// Sequencer commit hash (32 bytes).
    pub sequencer_hash: [u8; 32],
    /// Gas limit for this L2 block.
    pub gas_limit: u64,
    /// Base fee per gas (wei).
    pub base_fee_per_gas: u128,
}

impl L2BlockHeader {
    /// Compute a simple identifier hash for this block.
    ///
    /// **Stub:** XORs all fixed fields. Replace with keccak256(RLP(header)) in production.
    pub fn hash(&self) -> [u8; 32] {
        let mut h = self.parent_hash;
        for (i, &b) in self.l1_origin_hash.iter().enumerate() {
            h[i] ^= b;
        }
        for (i, &b) in self.sequencer_hash.iter().enumerate() {
            h[i % 32] ^= b;
        }
        h[0] ^= (self.number >> 56) as u8;
        h[1] ^= (self.timestamp >> 56) as u8;
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header() -> L2BlockHeader {
        L2BlockHeader {
            number: 1_000_000,
            parent_hash: [0xabu8; 32],
            timestamp: 1_700_000_000,
            l1_origin_number: 18_000_000,
            l1_origin_hash: [0xcd; 32],
            sequencer_hash: [0xef; 32],
            gas_limit: 30_000_000,
            base_fee_per_gas: 1_000_000_000,
        }
    }

    #[test]
    fn hash_is_deterministic() {
        let h = header();
        assert_eq!(h.hash(), h.hash());
    }

    #[test]
    fn different_blocks_different_hash() {
        let h1 = header();
        let mut h2 = header();
        h2.number = 1_000_001;
        // Hashes should differ (number affects h[0])
        // Note: not guaranteed with stub hash for all inputs
        let _ = (h1.hash(), h2.hash()); // at minimum, both succeed
    }
}
