use alloy_consensus::BlockBody;
use reth_optimism_primitives::{transaction::OpTransaction, DepositReceipt};
use reth_primitives_traits::{NodePrimitives, SignedTransaction};

/// Helper trait to encapsulate common bounds on [`NodePrimitives`] for OP payload builder.
pub trait OpPayloadPrimitives:
    NodePrimitives<Receipt: DepositReceipt, SignedTx = Self::_TX, BlockBody = BlockBody<Self::_TX>>
{
    /// Helper AT to bound [`NodePrimitives::Block`] type without causing bound cycle.
    type _TX: SignedTransaction + OpTransaction;
}

impl<Tx, T> OpPayloadPrimitives for T
where
    Tx: SignedTransaction + OpTransaction,
    T: NodePrimitives<SignedTx = Tx, Receipt: DepositReceipt, BlockBody = BlockBody<Tx>>,
{
    type _TX = Tx;
}
