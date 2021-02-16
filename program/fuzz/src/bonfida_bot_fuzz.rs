use bonfida_bot::{common::utils::{Context, mint_bootstrap}, state::FIDA_MINT_KEY};
use futures::executor::block_on;
use honggfuzz::fuzz;
use solana_program::{pubkey::Pubkey};
use solana_program_test::{ProgramTest, find_file, read_file};
use solana_sdk::{account::Account, signature::{Keypair, Signer}};

use bonfida_bot::common::simulation::Execution;

const SRM_MINT_KEY: &str = "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt";

#[tokio::main]
async fn main() {
    loop {
        let mut ctx = Context::init().await;
        let mints = ctx.get_mints();

        fuzz!(|e: Execution| {
            block_on(e.run(&mut ctx, &mints));
        });
    //     fuzz!(|v: Vec<u8>| {
    //         let mut sum = 0;
    //         for n in v {
    //             sum = sum + n
    //         }
    //     });
    }

}