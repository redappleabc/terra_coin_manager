use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, DepsMut, StdResult, Uint128, Coin};
use cw_storage_plus::{Item, Map, U128Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub cw20_addr: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pot {
    /// target_addr is the address that will receive the pot
    pub target_addr: Addr,
    /// threshold_amount is the token threshold amount
    pub threshold: Uint128,
    /// collected keeps information on how much is collected for this pot.
    pub collected: Uint128,
}
/// POT_SEQ holds the last pot ID
pub const POT_SEQ: Item<Uint128> = Item::new("pot_seq");
pub const POTS: Map<U128Key, Pot> = Map::new("pot");

pub fn save_pot(deps: DepsMut, pot: &Pot) -> StdResult<()> {
    // increment id if exists, or return 1
    let id = POT_SEQ.load(deps.storage)?;
    let id = id.checked_add(Uint128::new(1))?;
    POT_SEQ.save(deps.storage, &id)?;

    // save pot with id
    POTS.save(deps.storage, id.u128().into(), pot)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BackerState{
    pub backer_wallet: String,
    pub amount: Coin,
}
// pub const BACKERSTATES: Map<U128Key, BackerState> = Map::new("backer");
// pub const BACKERSTATE_SEQ: Item<Uint128> = Item::new("backer_seq");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectState{
    pub project_id: Uint128,
    pub project_wallet: String,
    pub backer_states:Vec<BackerState>,
}

//pub const ProjectState_SEQ: Item<Uint128> = Item::new("prj_seq");
pub const PROJECTSTATES: Map<U128Key, ProjectState> = Map::new("prj");
pub const PROJECTCONTRACTS: Map<String, bool> = Map::new("proj_contracts");