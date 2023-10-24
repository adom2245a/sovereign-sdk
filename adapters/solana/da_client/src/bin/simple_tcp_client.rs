use account_proof_geyser::types::Update;
use account_proof_geyser::utils::{verify_leaves_against_bankhash};
use borsh::{BorshDeserialize};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;


// #[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
// pub struct Data {
//     pub pubkey: Pubkey,
//     pub hash: Hash,
//     pub account: AccountInfo,
// }
//
// #[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
// pub enum AccountDeltaProof {
//     /// Simplest proof for inclusion in the account delta hash
//     InclusionProof(Pubkey, (Data, Proof)),
//     /// Adjacency proof for non inclusion A C D E, non-inclusion for B means providing A and C
//     NonInclusionProofInner(Pubkey, ((Data, Proof), (Data, Proof))),
//     /// Left most leaf and proof
//     NonInclusionProofLeft(Pubkey, (Data, Proof)),
//     /// Right most leaf and proof. Also need to include hashes of all leaves to verify tree size
//     NonInclusionProofRight(Pubkey, (Data, Proof, Vec<Hash>)),
// }
//
// #[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
// pub struct BankHashProof {
//     pub proofs: Vec<AccountDeltaProof>,
//     pub num_sigs: u64,
//     pub account_delta_root: Hash,
//     pub parent_bankhash: Hash,
//     pub blockhash: Hash,
// }
//
// #[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
// pub struct Update {
//     pub slot: u64,
//     pub root: Hash,
//     pub proof: BankHashProof,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:10000").await?;

    // Using a large buffer for simplicity.
    // Replace this with framing or an other alternative
    let mut buffer = vec![0u8; 65536];

    loop {
        let n = stream.read(&mut buffer).await?;

        if n == 0 {
            break; // Connection closed.
        }

        let received_update: Update = Update::try_from_slice(&buffer[..n])?;

        let bankhash = received_update.root;
        let bankhash_proof = received_update.proof;
        let slot_num = received_update.slot;

        for p in bankhash_proof.proofs {
            if let Err(e) = verify_leaves_against_bankhash(p,
                                           bankhash,
                                           bankhash_proof.num_sigs,
                                           bankhash_proof.account_delta_root,
                                           bankhash_proof.parent_bankhash,
                                           bankhash_proof.blockhash) {
                println!("Error in slot {}: {:?}",slot_num,e);
            } else {
                println!("Proof verification succeeded for slot {}",slot_num);
            }
        }
    }

    Ok(())
}