//! OP Stack rollup configuration.

/// Rollup configuration for an OP Stack chain.
#[derive(Debug, Clone)]
pub struct RollupConfig {
    /// Chain ID of this L2 network.
    pub l2_chain_id: u64,
    /// Chain ID of the L1 (settlement) network.
    pub l1_chain_id: u64,
    /// L1 address of the `OptimismPortal` contract (20 bytes).
    pub optimism_portal: [u8; 20],
    /// L1 address of the `L2OutputOracle` contract (20 bytes).
    pub l2_output_oracle: [u8; 20],
    /// Block time (seconds) for this L2.
    pub block_time: u64,
    /// Maximum sequencer drift (seconds) — how far ahead the sequencer can be from L1.
    pub max_sequencer_drift: u64,
    /// Regolith activation timestamp (None = not activated).
    pub regolith_time: Option<u64>,
    /// Canyon activation timestamp (None = not activated).
    pub canyon_time: Option<u64>,
    /// Delta activation timestamp.
    pub delta_time: Option<u64>,
    /// Ecotone activation timestamp.
    pub ecotone_time: Option<u64>,
    /// Fjord activation timestamp.
    pub fjord_time: Option<u64>,
}

impl RollupConfig {
    /// Return whether the Ecotone upgrade is active at the given timestamp.
    pub fn is_ecotone_active(&self, timestamp: u64) -> bool {
        self.ecotone_time.map_or(false, |t| timestamp >= t)
    }

    /// Return whether the Fjord upgrade is active at the given timestamp.
    pub fn is_fjord_active(&self, timestamp: u64) -> bool {
        self.fjord_time.map_or(false, |t| timestamp >= t)
    }
}

/// SEQUENTICHAIN mainnet rollup config (placeholder values).
pub fn sequentichain_config() -> RollupConfig {
    RollupConfig {
        l2_chain_id: 42_161,       // placeholder
        l1_chain_id: 1,
        optimism_portal: [0u8; 20],
        l2_output_oracle: [0u8; 20],
        block_time: 2,
        max_sequencer_drift: 600,
        regolith_time: Some(0),
        canyon_time: Some(1_704_992_401),
        delta_time: Some(1_708_560_000),
        ecotone_time: Some(1_710_374_401),
        fjord_time: Some(1_720_627_201),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecotone_activation() {
        let cfg = sequentichain_config();
        assert!(cfg.is_ecotone_active(cfg.ecotone_time.unwrap() + 1));
        assert!(!cfg.is_ecotone_active(0));
    }

    #[test]
    fn fjord_activation() {
        let cfg = sequentichain_config();
        assert!(cfg.is_fjord_active(cfg.fjord_time.unwrap()));
    }
}
