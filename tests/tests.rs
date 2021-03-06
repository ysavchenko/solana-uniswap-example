#![cfg(feature = "test-bpf")]

use solana_program::{program_pack::Pack, pubkey::Pubkey, system_instruction};
use solana_program_template::*;
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token_swap::curve::{base::SwapCurve, fees::Fees};

pub fn program_test() -> ProgramTest {
    let mut program_test = ProgramTest::new(
        "solana-program-template",
        crate::id(),
        processor!(processor::Processor::process_instruction),
    );

    program_test.add_program(
        "spl_token_swap",
        spl_token_swap::id(),
        None,
    ); 

    program_test
}

#[tokio::test]
async fn uniswap_like_setup() {
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let swap = Keypair::new();
    let swap_pubkey = swap.pubkey();
    let mint_a = Keypair::new();
    let mint_b = Keypair::new();
    let mint_ab = Keypair::new();

    let account_a = Keypair::new();
    let account_b = Keypair::new();
    let account_ab = Keypair::new();
    let account_ab_fee = Keypair::new();

    let rent = banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(10_000);
    let (authority, bump_seed) =
        Pubkey::find_program_address(&[&swap_pubkey.to_bytes()[..32]], &spl_token_swap::id());

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &mint_a.pubkey(),
                rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint_a.pubkey(),
                &payer.pubkey(),
                None,
                6,
            )
            .unwrap(),
            system_instruction::create_account(
                &payer.pubkey(),
                &mint_b.pubkey(),
                rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint_b.pubkey(),
                &payer.pubkey(),
                None,
                6,
            )
            .unwrap(),
            system_instruction::create_account(
                &payer.pubkey(),
                &mint_ab.pubkey(),
                rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint_ab.pubkey(),
                &authority,
                None,
                6,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &mint_a, &mint_b, &mint_ab], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &account_a.pubkey(),
                rent,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &account_a.pubkey(),
                &mint_a.pubkey(),
                &authority,
            )
            .unwrap(),
            spl_token::instruction::mint_to(
                &spl_token::id(),
                &mint_a.pubkey(),
                &account_a.pubkey(),
                &payer.pubkey(),
                &[&payer.pubkey()],
                1000,
            )
            .unwrap(),
            system_instruction::create_account(
                &payer.pubkey(),
                &account_b.pubkey(),
                rent,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &account_b.pubkey(),
                &mint_b.pubkey(),
                &authority,
            )
            .unwrap(),
            spl_token::instruction::mint_to(
                &spl_token::id(),
                &mint_b.pubkey(),
                &account_b.pubkey(),
                &payer.pubkey(),
                &[&payer.pubkey()],
                1000,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &account_a, &account_b], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &account_ab.pubkey(),
                rent,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &account_ab.pubkey(),
                &mint_ab.pubkey(),
                &payer.pubkey(),
            )
            .unwrap(),
            system_instruction::create_account(
                &payer.pubkey(),
                &account_ab_fee.pubkey(),
                rent,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &account_ab_fee.pubkey(),
                &mint_ab.pubkey(),
                &payer.pubkey(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &account_ab, &account_ab_fee], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &swap_pubkey,
                rent,
                spl_token_swap::state::SwapVersion::LATEST_LEN as u64,
                &spl_token_swap::id(),
            ),
            spl_token_swap::instruction::initialize(
                &spl_token_swap::id(),
                &spl_token::id(),
                &swap_pubkey,
                &authority,
                &account_a.pubkey(),
                &account_b.pubkey(),
                &mint_ab.pubkey(),
                &account_ab_fee.pubkey(),
                &account_ab.pubkey(),
                bump_seed,
                Fees::default(),
                SwapCurve::default(),
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &swap], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
