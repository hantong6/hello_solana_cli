use std::str::FromStr;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::sysvar;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair, Signature};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;

fn main() {
    println!("Hello, world!");
}

#[derive(BorshDeserialize, BorshSerialize)]
enum TokenInstruction {
    CreateToken {decimals: u8},
    Mint {amount: u64}
}

#[test]
fn test_hello_solana() {
    let solana_client = RpcClient::new("http://localhost:8899".to_string());
    let payer = read_keypair_file("/home/hantong/.config/solana/id-local.json").unwrap();
    let program_id = Pubkey::from_str("8nPrchpGf8Jt4FCZy37BvBrMkU8EMAr9S3vKzTEnqoBm").unwrap();
    let mint = Keypair::new();
    println!("Mint: {:?}", mint.pubkey());
    let create_sign = create_token(&solana_client, &program_id, &payer, &mint, 6).unwrap();
    println!("create token sign: {:?}", create_sign);
    let target = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
    println!("target: {:?}", target);
    let mint_sign = mint_token(&solana_client, &program_id, &payer, &mint, target, 600).unwrap();
    println!("mint token sign: {:?}", mint_sign);
}

fn create_token(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    mint: &Keypair,
    decimals: u8
) -> Result<Signature, Box<dyn std::error::Error>> {
    let create_token_data = borsh::to_vec(&TokenInstruction::CreateToken { decimals }).unwrap();
    let create_token_acc = vec![
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(payer.pubkey(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];
    let create_token_ins = Instruction {
        program_id: *program_id,
        accounts: create_token_acc,
        data: create_token_data
    };
    let latest_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let create_token_tx =  Transaction::new_signed_with_payer(
        &[create_token_ins],
        Some(&payer.pubkey()),
        &[mint, payer],
        latest_blockhash
    );
    let res = rpc_client.send_and_confirm_transaction(&create_token_tx)?;
    Ok(res)
}

fn mint_token(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    mint: &Keypair,
    target: Pubkey,
    amount: u64
) -> Result<Signature, Box<dyn std::error::Error>> {
    let mint_token_data = borsh::to_vec(&TokenInstruction::Mint { amount }).unwrap();
    let mint_token_acc = vec![
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new(target, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
    ];
    let mint_token_ins = Instruction {
        program_id: *program_id,
        accounts: mint_token_acc,
        data: mint_token_data
    };
    let latest_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let mint_token_tx =  Transaction::new_signed_with_payer(
        &[mint_token_ins],
        Some(&payer.pubkey()),
        &[mint, payer],
        latest_blockhash
    );
    let res = rpc_client.send_and_confirm_transaction(&mint_token_tx)?;
    Ok(res)
}

