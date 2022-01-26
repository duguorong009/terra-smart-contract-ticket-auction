#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};
use ticket_auction::auction_manager::{
    PlaceBetMsg as AuctionPlaceBetMsg, QueryMsg as AuctionQueryMsg,
};
use ticket_auction::ticket_manager::{
    ExecuteMsg as TicketExecuteMsg, QueryMsg as TicketQueryMsg, TicketResultMsg,
};

use crate::state::{read_config, store_config, Config};
use ticket_auction::collateral_manager::{
    ExecuteMsg as CollateralExecuteMsg, QueryMsg as CollateralQueryMsg, QueryStakeStatusMsg,
};
use ticket_auction::error::TAError;
use ticket_auction::user_board::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, PlaceBetMsg, PostConfigMsg, QueryMsg,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Validation 1: Check if the funds is provided.
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Get the config parameters
    let config = Config {
        admin_board: info.sender.to_string(),
        ticket_manager: msg.ticket_manager,
        collateral_manager: msg.collateral_manater,
        auction_manager: msg.auction_manager,
    };

    store_config(deps.storage, config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        // Transactions initiated by user(worker).
        ExecuteMsg::LockStake { tid } => execute_lock_stake(deps, info, tid),
        ExecuteMsg::PlaceBet(msg) => execute_place_bet(deps, info, msg),
        ExecuteMsg::SubmitResult(msg) => execute_submit_result(deps, info, msg),

        // Utilities
        ExecuteMsg::PostConfig(msg) => execute_post_config(deps, info, msg),
    }
}

fn execute_lock_stake(deps: DepsMut, info: MessageInfo, tid: u64) -> StdResult<Response> {
    // Validation 1: Check if the funds are provided.
    if info.funds.is_empty() {
        return Err(TAError::InsufficientFunds.into());
    }
    // Validation 2: Check if tx sender is valid wallet.
    if deps.api.addr_canonicalize(info.sender.as_str()).is_err() {
        return Err(TAError::InvalidAddress.into());
    }

    // Call the method of "collateral_manager"
    let config = read_config(deps.storage)?;
    let collateral_manager = match config.collateral_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };
    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collateral_manager,
        msg: to_binary(&CollateralExecuteMsg::LockStake { tid })?,
        funds: info.funds,
    })];

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "lock stake")]))
}

fn execute_place_bet(deps: DepsMut, info: MessageInfo, msg: PlaceBetMsg) -> StdResult<Response> {
    // Validation 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Validation 2: Check if the info.sender(worker) already locked stake
    let worker = info.sender.clone();
    let tid = msg.ticket_id;
    let config = read_config(deps.storage)?;
    let collateral_manager = match config.collateral_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };
    let is_staked: bool = deps.querier.query_wasm_smart(
        collateral_manager,
        &to_binary(&CollateralQueryMsg::QueryStakeStatus(QueryStakeStatusMsg {
            tid,
            worker,
        }))?,
    )?;

    if !is_staked {
        return Err(TAError::NotStaked.into());
    }

    let auction_manager = match config.auction_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };
    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: auction_manager,
        msg: to_binary(&AuctionPlaceBetMsg {
            ticket_id: tid,
            workder: info.sender.to_string(),
            bet_amount: msg.bet_amount,
        })?,
        funds: vec![],
    })];

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "place bet")]))
}

fn execute_submit_result(
    deps: DepsMut,
    info: MessageInfo,
    msg: TicketResultMsg,
) -> StdResult<Response> {
    // Validation 1: Check if the funds are provided.
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }
    // Validation 2: Check if tx send is the right worker for ticket
    let tid = msg.tid;
    let worker = info.sender;
    let config = read_config(deps.storage)?;
    let ticket_manager = match config.ticket_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };
    let right_worker: String = deps.querier.query_wasm_smart(
        ticket_manager.clone(),
        &to_binary(&TicketQueryMsg::QueryTicketWorker { tid })?,
    )?;

    if worker != right_worker {
        return Err(TAError::NotAuthorized.into());
    }

    // Call the method of "ticket_manager".
    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: ticket_manager,
        msg: to_binary(&TicketExecuteMsg::AssessSubmission(TicketResultMsg {
            tid,
            worker: msg.worker,
            result: msg.result,
        }))?,
        funds: vec![],
    })];

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "submit result")]))
}

fn execute_post_config(
    deps: DepsMut,
    info: MessageInfo,
    msg: PostConfigMsg,
) -> StdResult<Response> {
    // Validaton 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }
    // Validation 2: Check if tx sender is admin wallet
    let mut config = read_config(deps.storage)?;
    if info.sender != config.admin_board {
        return Err(TAError::NotAuthorized.into());
    }

    config.auction_manager = msg.auction_manager;
    config.collateral_manager = msg.collateral_manager;
    config.ticket_manager = msg.ticket_manager;

    store_config(deps.storage, config)?;

    Ok(Response::new().add_attribute("method", "PostConfig"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryBetAvailTickets {} => to_binary(&query_bet_avail_tickets(deps)?),
        QueryMsg::QueryStakeStatus(msg) => to_binary(&query_stake_status(deps, msg)?),
    }
}

fn query_bet_avail_tickets(deps: Deps) -> StdResult<Vec<u64>> {
    let config = read_config(deps.storage)?;
    let auction_manager = match config.auction_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };
    let avail_tickets: Vec<u64> = deps.querier.query_wasm_smart(
        auction_manager,
        &to_binary(&AuctionQueryMsg::BetAvailableTickets {})?,
    )?;
    Ok(avail_tickets)
}

fn query_stake_status(deps: Deps, msg: QueryStakeStatusMsg) -> StdResult<bool> {
    let config = read_config(deps.storage)?;
    let collateral_manager = match config.collateral_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let stake_status: bool = deps.querier.query_wasm_smart(
        collateral_manager,
        &to_binary(&CollateralQueryMsg::QueryStakeStatus(msg))?,
    )?;
    Ok(stake_status)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
