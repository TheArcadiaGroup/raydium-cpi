use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::Metadata, token::Token, token_interface::{Mint, Token2022, TokenAccount, TokenInterface}};
// use solana_program::{program::invoke_signed, system_instruction};
#[derive(Accounts)]
pub struct CreatePool<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub pool_creator: Signer<'info>,

    /// Which config the pool belongs to.
    /// CHECK: read by raydium
    pub amm_config: UncheckedAccount<'info>,

    /// CHECK: init by raydium
    #[account(mut)]
    pub pool_state: UncheckedAccount<'info>,

    /// Token_0 mint, the key must grater then token_1 mint.
    #[account(
        constraint = token_mint_0.key() < token_mint_1.key(),
        mint::token_program = token_program_0
    )]
    pub token_mint_0: Box<InterfaceAccount<'info, Mint>>,

    /// Token_1 mint
    #[account(
        mint::token_program = token_program_1
    )]
    pub token_mint_1: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: init by raydium
    #[account(mut)]
    pub token_vault_0: UncheckedAccount<'info>,

    /// CHECK: init by raydium
    #[account(mut)]
    pub token_vault_1: UncheckedAccount<'info>,

    /// CHECK: Initialize by raydium
    #[account(mut)]
    pub observation_state: UncheckedAccount<'info>,

    /// CHECK: Initialize by raydium
    #[account(mut)]
    pub tick_array_bitmap: UncheckedAccount<'info>,

    /// Spl token program or token program 2022
    pub token_program_0: Interface<'info, TokenInterface>,
    /// Spl token program or token program 2022
    pub token_program_1: Interface<'info, TokenInterface>,
    /// To create a new program account
    pub system_program: Program<'info, System>,
    /// Sysvar for program account
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(tick_lower_index: i32, tick_upper_index: i32,tick_array_lower_start_index:i32,tick_array_upper_start_index:i32)]
pub struct OpenPositionV2<'info> {
    /// Pays to mint the position
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Receives the position NFT
    #[account(mut)]
    pub position_nft_owner: UncheckedAccount<'info>,

    /// Unique token mint address
    /// CHECK: to init by raydium
    #[account(mut)]
    pub position_nft_mint: Signer<'info>,

    /// Token account where position NFT will be minted
    /// This account created in the contract by cpi to avoid large stack variables
    /// CHECK: to init by raydium
    #[account(
        mut
    )]
    pub position_nft_account: UncheckedAccount<'info>,

    /// To store metaplex metadata
    /// CHECK: Safety check performed inside function body
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// Add liquidity for this pool
    /// CHECK: to init by raydium
    #[account(mut)]
    pub pool_state: UncheckedAccount<'info>,

    /// Store the information of market marking in range
    /// CHECK: to init by raydium
    #[account(
        mut
    )]
    pub protocol_position: UncheckedAccount<'info>,

    /// CHECK: Account to mark the lower tick as initialized
    #[account(
        mut
    )]
    pub tick_array_lower: UncheckedAccount<'info>,

    /// CHECK:Account to store data for the position's upper tick
    #[account(
        mut
    )]
    pub tick_array_upper: UncheckedAccount<'info>,

    /// personal position state
    /// CHECK: to init by raydium
    #[account(
        mut
    )]
    pub personal_position: UncheckedAccount<'info>,

    /// The token_0 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_0.mint
    )]
    pub token_account_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The token_1 account deposit token to the pool
    #[account(
        mut,
        token::mint = token_vault_1.mint
    )]
    pub token_account_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_0
    #[account(
        mut,
    )]
    pub token_vault_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The address that holds pool tokens for token_1
    #[account(
        mut,
    )]
    pub token_vault_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Sysvar for token mint and ATA creation
    pub rent: Sysvar<'info, Rent>,

    /// Program to create the position manager state account
    pub system_program: Program<'info, System>,

    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Program to create NFT metadata
    /// CHECK: Metadata program address constraint applied
    pub metadata_program: Program<'info, Metadata>,
    /// Program to create mint account and mint tokens
    pub token_program_2022: Program<'info, Token2022>,
    /// The mint of token vault 0
    #[account(
        address = token_vault_0.mint
    )]
    pub vault_0_mint: Box<InterfaceAccount<'info, Mint>>,
    /// The mint of token vault 1
    #[account(
        address = token_vault_1.mint
    )]
    pub vault_1_mint: Box<InterfaceAccount<'info, Mint>>,
}