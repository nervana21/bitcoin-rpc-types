//! Hash or Height type for Bitcoin RPC APIs
//!
//! This module provides the `HashOrHeight` enum which represents either a block hash
//! (bitcoin::BlockHash) or block height (integer) for Bitcoin RPC APIs where methods
//! can accept either identifier to specify a particular block.

use bitcoin::BlockHash;
use serde::{Deserialize, Serialize};

/// Represents either a block hash or a block height
///
/// This type is used in some Bitcoin RPC APIs where methods can accept
/// either a block hash (bitcoin::BlockHash) or a block height (u32) to
/// identify a specific block.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HashOrHeight {
    /// Block hash
    Hash(BlockHash),
    /// Block height as a non-negative integer
    Height(u32),
}

impl HashOrHeight {
    /// Returns true if this represents a block hash
    pub fn is_hash(&self) -> bool { matches!(self, Self::Hash(_)) }

    /// Returns true if this represents a block height
    pub fn is_height(&self) -> bool { matches!(self, Self::Height(_)) }

    /// Returns the block hash if this is a Hash variant, otherwise None
    pub fn as_hash(&self) -> Option<&BlockHash> {
        if let Self::Hash(hash) = self {
            Some(hash)
        } else {
            None
        }
    }

    /// Returns the block height if this is a Height variant, otherwise None
    pub fn as_height(&self) -> Option<u32> {
        if let Self::Height(height) = self {
            Some(*height)
        } else {
            None
        }
    }
}

impl From<BlockHash> for HashOrHeight {
    fn from(hash: BlockHash) -> Self { Self::Hash(hash) }
}

impl From<u32> for HashOrHeight {
    fn from(height: u32) -> Self { Self::Height(height) }
}

#[cfg(test)]
mod tests {
    use bitcoin::BlockHash;

    use super::*;

    #[test]
    fn test_hash_or_height_is_hash() {
        let hash = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
            .parse::<BlockHash>()
            .unwrap();
        let hash_or_height = HashOrHeight::Hash(hash);
        assert!(hash_or_height.is_hash());
        assert!(!hash_or_height.is_height());

        let height = HashOrHeight::Height(123);
        assert!(!height.is_hash());
        assert!(height.is_height());
    }

    #[test]
    fn test_hash_or_height_is_height() {
        let height = HashOrHeight::Height(42);
        assert!(height.is_height());
        assert!(!height.is_hash());

        let hash = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
            .parse::<BlockHash>()
            .unwrap();
        let hash_or_height = HashOrHeight::Hash(hash);
        assert!(!hash_or_height.is_height());
        assert!(hash_or_height.is_hash());
    }

    #[test]
    fn test_hash_or_height_as_hash() {
        let hash = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
            .parse::<BlockHash>()
            .unwrap();
        let hash_or_height = HashOrHeight::Hash(hash);

        // Test getting hash from Hash variant
        let retrieved_hash = hash_or_height.as_hash();
        assert!(retrieved_hash.is_some());
        assert_eq!(retrieved_hash.unwrap(), &hash);

        // Test getting hash from Height variant
        let height = HashOrHeight::Height(123);
        assert!(height.as_hash().is_none());
    }

    #[test]
    fn test_hash_or_height_as_height() {
        let height = HashOrHeight::Height(42);

        // Test getting height from Height variant
        let retrieved_height = height.as_height();
        assert!(retrieved_height.is_some());
        assert_eq!(retrieved_height.unwrap(), 42);

        // Test getting height from Hash variant
        let hash = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
            .parse::<BlockHash>()
            .unwrap();
        let hash_or_height = HashOrHeight::Hash(hash);
        assert!(hash_or_height.as_height().is_none());
    }

    #[test]
    fn test_from_blockhash() {
        let hash = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
            .parse::<BlockHash>()
            .unwrap();
        let hash_or_height = HashOrHeight::from(hash);

        assert!(hash_or_height.is_hash());
        assert!(!hash_or_height.is_height());
        assert_eq!(hash_or_height.as_hash().unwrap(), &hash);
    }

    #[test]
    fn test_from_u32() {
        let height = 12345u32;
        let hash_or_height = HashOrHeight::from(height);

        assert!(hash_or_height.is_height());
        assert!(!hash_or_height.is_hash());
        assert_eq!(hash_or_height.as_height().unwrap(), height);
    }
}
