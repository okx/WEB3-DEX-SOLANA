// Copyright (c) 2023-2025 OKX.com
// Licensed under the MIT License

use anchor_lang::prelude::*;
pub mod adapters;
pub mod constants;
pub mod error;
pub mod instructions;
pub mod utils;

pub use constants::*;
pub use instructions::*;

declare_id!("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma");

#[program]
pub mod dex_solana {
    use super::*;

    pub fn swap<'a>(ctx: Context<'_, '_, 'a, 'a, SwapAccounts<'a>>, data: SwapArgs) -> Result<u64> {
        instructions::swap_handler(ctx, data, 0)
    }

    pub fn swap2<'a>(
        ctx: Context<'_, '_, 'a, 'a, SwapAccounts<'a>>,
        data: SwapArgs,
        order_id: u64,
    ) -> Result<u64> {
        instructions::swap_handler(ctx, data, order_id)
    }

    pub fn commission_spl_swap<'a>(
        ctx: Context<'_, '_, 'a, 'a, CommissionSPLAccounts<'a>>,
        data: CommissionSwapArgs,
    ) -> Result<u64> {
        instructions::commission_spl_swap_handler(ctx, data, 0)
    }

    pub fn commission_spl_swap2<'a>(
        ctx: Context<'_, '_, 'a, 'a, CommissionSPLAccounts<'a>>,
        data: CommissionSwapArgs,
        order_id: u64,
    ) -> Result<u64> {
        instructions::commission_spl_swap_handler(ctx, data, order_id)
    }

    pub fn commission_sol_swap<'a>(
        ctx: Context<'_, '_, 'a, 'a, CommissionSOLAccounts<'a>>,
        data: CommissionSwapArgs,
    ) -> Result<u64> {
        instructions::commission_sol_swap_handler(ctx, data, 0)
    }

    pub fn commission_sol_swap2<'a>(
        ctx: Context<'_, '_, 'a, 'a, CommissionSOLAccounts<'a>>,
        data: CommissionSwapArgs,
        order_id: u64,
    ) -> Result<u64> {
        instructions::commission_sol_swap_handler(ctx, data, order_id)
    }

    pub fn from_swap_log<'a>(
        ctx: Context<'_, '_, 'a, 'a, FromSwapAccounts<'a>>,
        args: SwapArgs,
        bridge_to_args: BridgeToArgs,
        offset: u8,
        len: u8,
    ) -> Result<()> {
        instructions::from_swap_log_handler(ctx, args, bridge_to_args, offset, len)
    }

    // proxy swap
    pub fn proxy_swap<'a>(
        ctx: Context<'_, '_, 'a, 'a, ProxySwapAccounts<'a>>,
        data: SwapArgs,
        order_id: u64,
    ) -> Result<u64> {
        instructions::proxy_swap_handler(ctx, data, order_id)
    }

    pub fn commission_sol_proxy_swap<'a>(
        ctx: Context<'_, '_, 'a, 'a, CommissionSOLProxySwapAccounts<'a>>,
        data: SwapArgs,
        commission_rate: u16,
        commission_direction: bool,
        order_id: u64,
    ) -> Result<u64> {
        instructions::commission_sol_proxy_swap_handler(
            ctx,
            data,
            commission_rate,
            commission_direction,
            order_id,
        )
    }

    pub fn commission_spl_proxy_swap<'a>(
        ctx: Context<'_, '_, 'a, 'a, CommissionSPLProxySwapAccounts<'a>>,
        data: SwapArgs,
        commission_rate: u16,
        commission_direction: bool,
        order_id: u64,
    ) -> Result<u64> {
        instructions::commission_spl_proxy_swap_handler(
            ctx,
            data,
            commission_rate,
            commission_direction,
            order_id,
        )
    }

    pub fn commission_sol_from_swap<'a>(
        ctx: Context<'_, '_, 'a, 'a, CommissionSOLFromSwapAccounts<'a>>,
        args: SwapArgs,
        commission_rate: u16,
        bridge_to_args: BridgeToArgs,
        offset: u8,
        len: u8,
    ) -> Result<()> {
        instructions::commission_sol_from_swap_handler(
            ctx,
            args,
            commission_rate,
            bridge_to_args,
            offset,
            len,
        )
    }

    pub fn commission_spl_from_swap<'a>(
        ctx: Context<'_, '_, 'a, 'a, CommissionSPLFromSwapAccounts<'a>>,
        args: SwapArgs,
        commission_rate: u16,
        bridge_to_args: BridgeToArgs,
        offset: u8,
        len: u8,
    ) -> Result<()> {
        instructions::commission_spl_from_swap_handler(
            ctx,
            args,
            commission_rate,
            bridge_to_args,
            offset,
            len,
        )
    }
}
