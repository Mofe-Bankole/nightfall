// use zcash_primitives::transaction::Transaction;
// use zcash_proofs::prover::LocalTxProver;

// pub struct BackendProver {
//     pub prover: LocalTxProver,
// }

// impl BackendProver {
//     pub fn initialize_prover() -> BackendProver {
//         BackendProver {
//             prover: LocalTxProver::bundled(),
//         }
//     }

//     pub fn prove_tnx(&self, tx: &Transaction) -> Result<(), anyhow::Error> {
//         self.prover.prove_transaction(tx)
//     }
// }
