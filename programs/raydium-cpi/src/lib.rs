use anchor_lang::prelude::*;
pub mod context;

#[cfg(feature = "devnet")]
declare_id!("devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH");
#[cfg(not(feature = "devnet"))]
declare_id!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");
use context::*;

#[program]
pub mod raydium_cpi {
    use super::*;

    pub fn create_pool(
        _ctx: Context<CreatePool>,
        _sqrt_price_x64: u128,
        _open_time: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Creates a new position wrapped in a NFT, support Token2022
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `tick_lower_index` - The low boundary of market
    /// * `tick_upper_index` - The upper boundary of market
    /// * `tick_array_lower_start_index` - The start index of tick array which include tick low
    /// * `tick_array_upper_start_index` - The start index of tick array which include tick upper
    /// * `liquidity` - The liquidity to be added, if zero, and the base_flage is specified, calculate liquidity base amount_0_max or amount_1_max according base_flag, otherwise open position with zero liquidity
    /// * `amount_0_max` - The max amount of token_0 to spend, which serves as a slippage check
    /// * `amount_1_max` - The max amount of token_1 to spend, which serves as a slippage check
    /// * `base_flag` - if the liquidity specified as zero, true: calculate liquidity base amount_0_max otherwise base amount_1_max
    ///
    pub fn open_position_v2(
        _ctx: Context<OpenPositionV2>,
        _tick_lower_index: i32,
        _tick_upper_index: i32,
        _tick_array_lower_start_index: i32,
        _tick_array_upper_start_index: i32,
        _liquidity: u128,
        _amount_0_max: u64,
        _amount_1_max: u64,
        _with_matedata: bool,
        _base_flag: Option<bool>,
    ) -> Result<()> {
        Ok(())
    }
}
