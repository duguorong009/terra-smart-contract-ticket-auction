#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, Uint128,
};
use cw_storage_plus::U64Key;

use crate::state::{read_config, read_stakes, store_config, store_stakes, Config};
use ticket_auction::collateral_manager::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, QueryStakeStatusMsg, ReleaseStakeMsg,
};
use ticket_auction::error::TAError;
use ticket_auction::ticket_manager::{QueryMsg as TicketQueryMsg, TicketInfoResponse};

const BASE_DENOM: &str = "uluna";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Validate if funds were sent
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Store the tx sender as system admin.
    let admin_board = info.sender.to_string();
    let ticket_manager = msg.ticket_manager.to_string();
    let user_board = msg.user_board;

    store_config(
        deps.storage,
        Config {
            admin_board,
            ticket_manager,
            user_board,
        },
    )?;

    // Initialize the variables
    store_stakes(deps.storage, U64Key::from(0), vec![])?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("contract", env.contract.address)
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        // Transactions initiated by user_board contract.
        ExecuteMsg::LockStake { tid } => execute_lock_stake(deps, env, info, tid),

        // Transactions initiated by admin_board contract. (user_board -> ticket_manager -> here).
        ExecuteMsg::ReleaseStake(msg) => execute_release_stake(deps, env, info, msg),
    }
}

fn execute_lock_stake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tid: u64,
) -> StdResult<Response> {
    // validation 1: Check if the tx sender is user_board
    let config = read_config(deps.storage)?;
    if info.sender != config.user_board {
        return Err(TAError::NotAuthorized.into());
    }

    // validation 2: Check if the sent funds are sufficient for ticket
    let ticket_info = query_ticket(deps.as_ref(), tid)?;
    let base_coin = info
        .funds
        .into_iter()
        .filter(|c| c.denom == *BASE_DENOM)
        .collect::<Vec<Coin>>();
    if base_coin.len() != 1 && base_coin[0].amount != Uint128::from(ticket_info.collateral) {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Store the stake data.
    let mut workers = read_stakes(deps.storage, U64Key::from(tid))?;
    workers.push(info.sender);

    store_stakes(deps.storage, U64Key::from(tid), workers)?;

    Ok(Response::new().add_attribute("method", "lock_stake"))
}

fn execute_release_stake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ReleaseStakeMsg,
) -> StdResult<Response> {
    // Validate if the tx sender is admin.
    let config = read_config(deps.storage)?;
    if info.sender != config.admin_board {
        return Err(TAError::NotAuthorized.into());
    }

    // Remove stake record from STAKES
    let workers = read_stakes(deps.storage, U64Key::from(msg.tid))?;
    let workers = workers
        .into_iter()
        .filter(|w| *w != msg.worker)
        .collect::<Vec<Addr>>();
    store_stakes(deps.storage, U64Key::from(msg.tid), workers)?;

    // Build the message to release the stake.
    let messages: Vec<CosmosMsg> = vec![CosmosMsg::Bank(BankMsg::Send {
        to_address: msg.worker.to_string(),
        amount: vec![Coin {
            denom: BASE_DENOM.to_string(),
            amount: msg.amt,
        }],
    })];
    Ok(Response::new()
        .add_messages(messages)
        .add_attributes(vec![attr("method", "release stake")]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryTicket { tid } => to_binary(&query_ticket(deps, tid)?),
        QueryMsg::QueryStakeStatus(msg) => to_binary(&query_stake_status(deps, msg)?),
    }
}

fn query_ticket(deps: Deps, tid: u64) -> StdResult<TicketInfoResponse> {
    let config = read_config(deps.storage)?;
    let ticket_info_response: TicketInfoResponse = deps.querier.query_wasm_smart(
        config.ticket_manager,
        &to_binary(&TicketQueryMsg::QueryTicketInfo { tid })?,
    )?;
    Ok(ticket_info_response)
}

fn query_stake_status(deps: Deps, msg: QueryStakeStatusMsg) -> StdResult<bool> {
    let stakes = read_stakes(deps.storage, U64Key::from(msg.tid))?;
    let is_staked = stakes.into_iter().any(|w| w == msg.worker);
    Ok(is_staked)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
