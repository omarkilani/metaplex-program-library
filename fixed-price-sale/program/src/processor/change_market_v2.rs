use crate::{
    error::ErrorCode,
    state::{MarketState, PiecesMode},
    utils::*,
    ChangeMarketV2,
};
use anchor_lang::prelude::*;

impl<'info> ChangeMarketV2<'info> {
    pub fn process(
        &mut self,
        new_name: Option<String>,
        new_description: Option<String>,
        mutable: Option<bool>,
        new_price: Option<u64>,
        pieces_mode: Option<PiecesMode>,
    ) -> Result<()> {
        let market = &mut self.market;
        let clock = &self.clock;

        // Check, that `Market` is in `Suspended` state
        if market.state != MarketState::Suspended {
            return Err(ErrorCode::MarketInInvalidState.into());
        }

        // Check, that `Market` is not in `Ended` state
        if let Some(end_date) = market.end_date {
            if clock.unix_timestamp as u64 > end_date {
                return Err(ErrorCode::MarketIsEnded.into());
            }
        }

        // Check, that `Market` is mutable
        if !market.mutable {
            return Err(ErrorCode::MarketIsImmutable.into());
        }

        if let Some(new_name) = new_name {
            if new_name.len() > NAME_MAX_LEN {
                return Err(ErrorCode::NameIsTooLong.into());
            }

            market.name = puffed_out_string(new_name, NAME_MAX_LEN);
        }

        if let Some(new_description) = new_description {
            if new_description.len() > DESCRIPTION_MAX_LEN {
                return Err(ErrorCode::DescriptionIsTooLong.into());
            }

            market.description = puffed_out_string(new_description, DESCRIPTION_MAX_LEN);
        }

        if let Some(mutable) = mutable {
            market.mutable = mutable;
        }

        if let Some(new_price) = new_price {
            if new_price == 0 {
                return Err(ErrorCode::PriceIsZero.into());
            }

            market.price = new_price;
        }

        if let Some(pieces_mode) = pieces_mode {
            match pieces_mode {
                PiecesMode::Limited(value) => market.pieces_in_one_wallet = Some(value),
                PiecesMode::Unlimited => market.pieces_in_one_wallet = None,
            }
        }

        Ok(())
    }
}