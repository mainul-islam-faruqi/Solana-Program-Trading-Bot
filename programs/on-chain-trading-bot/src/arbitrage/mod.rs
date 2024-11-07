use anchor_lang::prelude::*;
use crate::dex::{raydium::*, jupiter::*, serum::*};
use crate::errors::TradingBotError;
use crate::types::{TokenPair, PriceData};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ArbitrageRoute {
    pub route_type: RouteType,
    pub token_pair: TokenPair,
    pub entry_dex: DexType,
    pub exit_dex: DexType,
    pub expected_profit: u64,
    pub min_profit: u64,
    pub max_slippage: u16,
    pub deadline: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RouteType {
    RaydiumJupiter,
    JupiterSerum,
    SerumRaydium,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum DexType {
    Raydium,
    Jupiter,
    Serum,
}

pub struct ArbitrageManager;

impl ArbitrageManager {
    // Find arbitrage opportunities across DEXs
    pub fn find_arbitrage_opportunities(
        ctx: Context<ArbitrageSearch>,
        token_pair: TokenPair,
        min_profit: u64,
    ) -> Result<Vec<ArbitrageRoute>> {
        // Get prices from all DEXs
        let prices = Self::get_dex_prices(ctx.accounts)?;

        // Validate price data
        Self::validate_price_data(&prices)?;

        // Calculate potential arbitrage routes
        let routes = Self::calculate_arbitrage_routes(
            prices,
            min_profit,
            token_pair,
        )?;

        // Filter profitable routes
        let profitable_routes = routes.into_iter()
            .filter(|route| route.expected_profit >= route.min_profit)
            .collect();

        Ok(profitable_routes)
    }

    // Execute arbitrage trade
    pub fn execute_arbitrage(
        ctx: Context<ExecuteArbitrage>,
        route: ArbitrageRoute,
    ) -> Result<()> {
        // Verify deadline
        require!(
            Clock::get()?.unix_timestamp <= route.deadline,
            TradingBotError::DeadlineExceeded
        );

        // Execute trades based on route type
        match route.route_type {
            RouteType::RaydiumJupiter => {
                Self::execute_raydium_jupiter_arb(ctx, &route)?;
            },
            RouteType::JupiterSerum => {
                Self::execute_jupiter_serum_arb(ctx, &route)?;
            },
            RouteType::SerumRaydium => {
                Self::execute_serum_raydium_arb(ctx, &route)?;
            },
        }

        Ok(())
    }

    // Helper functions
    fn get_dex_prices(accounts: &ArbitrageSearch) -> Result<DexPrices> {
        // Get Raydium price
        let raydium_price = RaydiumDex::get_price(
            accounts.raydium_market,
            accounts.price_feed,
        )?;

        // Get Jupiter price
        let jupiter_price = JupiterDex::get_price(
            accounts.jupiter_market,
            accounts.price_feed,
        )?;

        // Get Serum price
        let serum_price = SerumDex::get_price(
            accounts.serum_market,
            accounts.price_feed,
        )?;

        Ok(DexPrices {
            raydium: raydium_price,
            jupiter: jupiter_price,
            serum: serum_price,
        })
    }

    fn validate_price_data(prices: &DexPrices) -> Result<()> {
        // Check price staleness
        let current_time = Clock::get()?.unix_timestamp;
        let max_staleness = 60; // 60 seconds

        require!(
            current_time - prices.raydium.timestamp <= max_staleness &&
            current_time - prices.jupiter.timestamp <= max_staleness &&
            current_time - prices.serum.timestamp <= max_staleness,
            TradingBotError::StalePriceFeed
        );

        Ok(())
    }

    fn calculate_arbitrage_routes(
        prices: DexPrices,
        min_profit: u64,
        token_pair: TokenPair,
    ) -> Result<Vec<ArbitrageRoute>> {
        let mut routes = Vec::new();

        // Check Raydium -> Jupiter arbitrage
        if let Some(route) = Self::check_route_profitability(
            prices.raydium,
            prices.jupiter,
            min_profit,
            RouteType::RaydiumJupiter,
            token_pair.clone(),
        )? {
            routes.push(route);
        }

        // Check Jupiter -> Serum arbitrage
        if let Some(route) = Self::check_route_profitability(
            prices.jupiter,
            prices.serum,
            min_profit,
            RouteType::JupiterSerum,
            token_pair.clone(),
        )? {
            routes.push(route);
        }

        // Check Serum -> Raydium arbitrage
        if let Some(route) = Self::check_route_profitability(
            prices.serum,
            prices.raydium,
            min_profit,
            RouteType::SerumRaydium,
            token_pair,
        )? {
            routes.push(route);
        }

        Ok(routes)
    }

    fn check_route_profitability(
        price_a: PriceData,
        price_b: PriceData,
        min_profit: u64,
        route_type: RouteType,
        token_pair: TokenPair,
    ) -> Result<Option<ArbitrageRoute>> {
        let price_diff = if price_b.price > price_a.price {
            price_b.price - price_a.price
        } else {
            return Ok(None);
        };

        let expected_profit = price_diff
            .checked_mul(10000)
            .ok_or(TradingBotError::Overflow)?
            .checked_div(price_a.price)
            .ok_or(TradingBotError::Overflow)?;

        if expected_profit >= min_profit {
            Ok(Some(ArbitrageRoute {
                route_type,
                token_pair,
                entry_dex: Self::get_entry_dex(&route_type),
                exit_dex: Self::get_exit_dex(&route_type),
                expected_profit,
                min_profit,
                max_slippage: 100, // 1%
                deadline: Clock::get()?.unix_timestamp + 60, // 60 seconds
            }))
        } else {
            Ok(None)
        }
    }

    fn get_entry_dex(route_type: &RouteType) -> DexType {
        match route_type {
            RouteType::RaydiumJupiter => DexType::Raydium,
            RouteType::JupiterSerum => DexType::Jupiter,
            RouteType::SerumRaydium => DexType::Serum,
        }
    }

    fn get_exit_dex(route_type: &RouteType) -> DexType {
        match route_type {
            RouteType::RaydiumJupiter => DexType::Jupiter,
            RouteType::JupiterSerum => DexType::Serum,
            RouteType::SerumRaydium => DexType::Raydium,
        }
    }
}

#[derive(Accounts)]
pub struct ArbitrageSearch<'info> {
    #[account(mut)]
    pub raydium_market: AccountInfo<'info>,
    #[account(mut)]
    pub jupiter_market: AccountInfo<'info>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: Verified in program
    pub price_feed: AccountInfo<'info>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteArbitrage<'info> {
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub raydium_market: AccountInfo<'info>,
    #[account(mut)]
    pub jupiter_market: AccountInfo<'info>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub owner: Signer<'info>,
}

struct DexPrices {
    raydium: PriceData,
    jupiter: PriceData,
    serum: PriceData,
} 