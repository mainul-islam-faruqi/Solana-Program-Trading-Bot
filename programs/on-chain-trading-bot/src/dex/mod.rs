pub mod raydium;
pub mod jupiter;
pub mod serum;
pub mod common;

use anchor_lang::prelude::*;
use crate::errors::TradingBotError;

pub struct CrossDexRouter;

impl CrossDexRouter {
    // Route order across DEXs
    pub fn route_order_across_dexs(
        ctx: Context<CrossDexOrder>,
        amount: u64,
        routes: Vec<DexRoute>,
    ) -> Result<()> {
        // Verify routes
        require!(!routes.is_empty(), TradingBotError::InvalidRoutes);

        // Execute trades through each DEX
        let mut current_amount = amount;
        for route in routes {
            match route.dex_type {
                DexType::Raydium => {
                    current_amount = Self::execute_raydium_route(
                        ctx.accounts.clone(),
                        route,
                        current_amount,
                    )?;
                },
                DexType::Jupiter => {
                    current_amount = Self::execute_jupiter_route(
                        ctx.accounts.clone(),
                        route,
                        current_amount,
                    )?;
                },
                DexType::Serum => {
                    current_amount = Self::execute_serum_route(
                        ctx.accounts.clone(),
                        route,
                        current_amount,
                    )?;
                },
            }
        }

        Ok(())
    }

    // Aggregate liquidity across DEXs
    pub fn aggregate_liquidity(
        ctx: Context<LiquidityAggregation>,
        token_pair: TokenPair,
    ) -> Result<Vec<LiquiditySource>> {
        let mut sources = Vec::new();

        // Get Raydium liquidity
        if let Ok(raydium_liquidity) = Self::get_raydium_liquidity(
            ctx.accounts.clone(),
            token_pair.clone(),
        ) {
            sources.push(raydium_liquidity);
        }

        // Get Jupiter liquidity
        if let Ok(jupiter_liquidity) = Self::get_jupiter_liquidity(
            ctx.accounts.clone(),
            token_pair.clone(),
        ) {
            sources.push(jupiter_liquidity);
        }

        // Get Serum liquidity
        if let Ok(serum_liquidity) = Self::get_serum_liquidity(
            ctx.accounts.clone(),
            token_pair.clone(),
        ) {
            sources.push(serum_liquidity);
        }

        Ok(sources)
    }

    // Compare prices across DEXs
    pub fn compare_prices(
        ctx: Context<PriceComparison>,
        token_pair: TokenPair,
    ) -> Result<Vec<DexPrice>> {
        let mut prices = Vec::new();

        // Get Raydium price
        if let Ok(raydium_price) = raydium::RaydiumDex::get_price(
            &ctx.accounts.raydium_price_feed,
            60,
        ) {
            prices.push(DexPrice {
                dex: DexType::Raydium,
                price: raydium_price,
            });
        }

        // Get Jupiter price
        if let Ok(jupiter_price) = jupiter::JupiterDex::get_price(
            &ctx.accounts.jupiter_price_feed,
            60,
        ) {
            prices.push(DexPrice {
                dex: DexType::Jupiter,
                price: jupiter_price,
            });
        }

        // Get Serum price
        if let Ok(serum_price) = serum::SerumDex::get_price(
            &ctx.accounts.serum_price_feed,
            60,
        ) {
            prices.push(DexPrice {
                dex: DexType::Serum,
                price: serum_price,
            });
        }

        Ok(prices)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DexRoute {
    pub dex_type: DexType,
    pub token_in: Pubkey,
    pub token_out: Pubkey,
    pub minimum_out: u64,
    pub slippage_bps: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum DexType {
    Raydium,
    Jupiter,
    Serum,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenPair {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquiditySource {
    pub dex: DexType,
    pub pool_id: Pubkey,
    pub liquidity: u64,
    pub fee_tier: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DexPrice {
    pub dex: DexType,
    pub price: Price,
} 