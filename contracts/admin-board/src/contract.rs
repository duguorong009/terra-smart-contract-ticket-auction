use std::ops::{Mul, Sub};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128, WasmMsg,
};

use crate::state::{read_config, store_config, Config};

use ticket_auction::{
    admin_board::{ExecuteMsg, InstantiateMsg, MigrateMsg, PostConfigMsg, QueryMsg, SlashMsg},
    auction_manager::InstantiateMsg as AuctionInstantiateMsg,
    collateral_manager::{InstantiateMsg as CollateralInstantiateMsg, ReleaseStakeMsg},
    error::TAError,
    ticket_manager::{
        AddTicketMsg, InstantiateMsg as TicketInstantiateMsg, TicketInfoResponse, UpdateTicketMsg,
    },
    user_board::InstantiateMsg as UserBoardInstantiateMsg,
};

use ticket_auction::auction_manager::ExecuteMsg as AuctionExecuteMsg;
use ticket_auction::collateral_manager::ExecuteMsg as CollateralExecuteMsg;
use ticket_auction::ticket_manager::{ExecuteMsg as TicketExecuteMsg, QueryMsg as TicketQueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Build & store the config.
    let config = Config {
        admin: info.sender.clone(),
        ticket_manager: None,
        collateral_manager: None,
        auction_manager: None,
        user_board: None,
    };

    store_config(deps.storage, config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        // Transactions initiated by admin wallet.
        ExecuteMsg::AddTicket(msg) => execute_add_ticket(deps, env, info, msg),
        ExecuteMsg::UpdateTicket(msg) => execute_update_ticket(deps, env, info, msg),
        ExecuteMsg::RemoveTicket { tid } => execute_remove_ticket(deps, env, info, tid),
        ExecuteMsg::DecideWinningBet { tid } => execute_decide_win_bet(deps, env, info, tid),

        // Transaction initiated by "ticket_manager" (user_board -> ticket_manager -> here)
        ExecuteMsg::ReleaseStakeWithSlash(msg) => {
            execute_release_stake_with_slash(deps, env, info, msg)
        }

        // Utilities
        ExecuteMsg::CreateTicketManager { code_id } => {
            execute_create_ticket_manager(deps, env, info, code_id)
        }
        ExecuteMsg::CreateUsrBoardManager { code_id } => {
            execute_create_user_board_manager(deps, env, info, code_id)
        }
        ExecuteMsg::CreateAuctionManager { code_id } => {
            execute_create_auction_manager(deps, env, info, code_id)
        }
        ExecuteMsg::CreateCollateralManager { code_id } => {
            execute_create_collateral_manager(deps, env, info, code_id)
        }

        ExecuteMsg::PostConfig(msg) => execute_post_config(deps, env, info, msg),
    }
}

// Call the "AddNewTicket" of "ticket_manager"
fn execute_add_ticket(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: AddTicketMsg,
) -> StdResult<Response> {
    // Validation 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Validation 2: Check if the tx sender is real admin wallet
    let config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(TAError::NotAuthorized.into());
    }

    // Call the method of "AddTicket" in ticket_manager
    let ticket_manager = match config.ticket_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: ticket_manager,
        msg: to_binary(&TicketExecuteMsg::AddTicket(msg))?,
        funds: vec![],
    })];

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "add a new ticket")]))
}

// Call the "UpdateNewTicket" of "ticket_manager"
fn execute_update_ticket(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateTicketMsg,
) -> StdResult<Response> {
    // Validation 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Validation 2: Check if the tx sender is real admin wallet
    let config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(TAError::NotAuthorized.into());
    }

    // Call the method of "AddTicket" in ticket_manager
    let ticket_manager = match config.ticket_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: ticket_manager,
        msg: to_binary(&TicketExecuteMsg::UpdateTicket(msg))?,
        funds: vec![],
    })];

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "update ticket")]))
}

// Call the "RemoveTicket" of "ticket_manager"
fn execute_remove_ticket(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tid: u64,
) -> StdResult<Response> {
    // Validation 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Validation 2: Check if the tx sender is real admin wallet
    let config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(TAError::NotAuthorized.into());
    }

    // Call the method of "AddTicket" in ticket_manager
    let ticket_manager = match config.ticket_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: ticket_manager,
        msg: to_binary(&TicketExecuteMsg::RemoveTicket { tid })?,
        funds: vec![],
    })];

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "remove ticket")]))
}

// Call the "DecideWinBet" of "auction_manager"
fn execute_decide_win_bet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tid: u64,
) -> StdResult<Response> {
    // Validation 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Validation 2: Check if the tx sender is real admin wallet
    let config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(TAError::NotAuthorized.into());
    }

    // Call the method of "DecideWinningBet" in auction_manager
    let auction_manager = match config.auction_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: auction_manager,
        msg: to_binary(&AuctionExecuteMsg::DecideWinningBet { tid })?,
        funds: vec![],
    })];

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "decide winning bet")]))
}

// Call the "ReleaseStake" of "collateral_manager"
fn execute_release_stake_with_slash(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SlashMsg,
) -> StdResult<Response> {
    // Validation 1: Check if the tx sender is ticket_manager
    let config = read_config(deps.storage)?;
    let ticket_manager = match config.ticket_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };
    if info.sender != ticket_manager {
        return Err(TAError::NotAuthorized.into());
    }

    // Query the stake amount for tid
    let ticket_info: TicketInfoResponse = deps.querier.query_wasm_smart(
        ticket_manager,
        &to_binary(&TicketQueryMsg::QueryTicketInfo { tid: msg.tid })?,
    )?;
    let stake_amount = ticket_info.collateral;

    // Query the release amount based on given slash percentage.
    let slash_perc = Decimal::from_ratio(msg.slash_perc, Uint128::from(1000u128));
    let slash_amt = Uint128::from(stake_amount as u128).mul(slash_perc);
    let release_amt = Uint128::from(stake_amount as u128).sub(slash_amt);

    // Call the method "ReleaseStake" of collaterral_manager
    let collateral_manager = match config.collateral_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };
    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collateral_manager,
        msg: to_binary(&CollateralExecuteMsg::ReleaseStake(ReleaseStakeMsg {
            tid: msg.tid,
            worker: msg.worker,
            amt: release_amt,
        }))?,
        funds: vec![],
    })];

    // TBD: Log every release result.(ticket, worker, stake, slash perc)

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "release stake with slash")]))
}

fn execute_create_ticket_manager(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    code_id: u64,
) -> StdResult<Response> {
    // Validaton 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }
    // Validation 2: Check if tx sender is admin wallet
    let config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(TAError::NotAuthorized.into());
    }
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id,
        msg: to_binary(&TicketInstantiateMsg {
            auction_manager: None,
            user_board: None,
        })?,
        funds: vec![],
        label: "".to_string(),
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "instantiate ticket manager"))
}

fn execute_create_user_board_manager(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    code_id: u64,
) -> StdResult<Response> {
    // Validaton 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }
    // Validation 2: Check if tx sender is admin wallet
    let config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(TAError::NotAuthorized.into());
    }
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id,
        msg: to_binary(&UserBoardInstantiateMsg {
            auction_manager: None,
            ticket_manager: None,
            collateral_manater: None,
        })?,
        funds: vec![],
        label: "".to_string(),
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "instantiate user_board manager"))
}

fn execute_create_auction_manager(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    code_id: u64,
) -> StdResult<Response> {
    // Validaton 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }
    // Validation 2: Check if tx sender is admin wallet
    let config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(TAError::NotAuthorized.into());
    }

    let ticket_manager = match config.ticket_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id,
        msg: to_binary(&AuctionInstantiateMsg { ticket_manager })?,
        funds: vec![],
        label: "".to_string(),
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "instantiate auction manager"))
}

fn execute_create_collateral_manager(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    code_id: u64,
) -> StdResult<Response> {
    // Validaton 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }
    // Validation 2: Check if tx sender is admin wallet
    let config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(TAError::NotAuthorized.into());
    }

    let ticket_manager = match config.ticket_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let user_board = match config.user_board {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id,
        msg: to_binary(&CollateralInstantiateMsg {
            ticket_manager,
            user_board,
        })?,
        funds: vec![],
        label: "".to_string(),
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "instantiate auction manager"))
}

fn execute_post_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: PostConfigMsg,
) -> StdResult<Response> {
    // Validaton 1: Check if the funds are provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }
    // Validation 2: Check if tx sender is admin wallet
    let mut config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(TAError::NotAuthorized.into());
    }

    config.auction_manager = msg.auction_manager;
    config.collateral_manager = msg.collateral_manager;
    config.ticket_manager = msg.ticket_manager;
    config.user_board = msg.user_board;

    store_config(deps.storage, config)?;

    Ok(Response::new().add_attribute("method", "PostConfig"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryTicketInfo { tid } => to_binary(&query_ticket_info(deps, tid)?),
        QueryMsg::QueryTicketWorker { tid } => to_binary(&query_ticket_worker(deps, tid)?),
    }
}

// Query the ticket info. (Call the "QueryTicketInfo" of "ticket_manager")
fn query_ticket_info(deps: Deps, tid: u64) -> StdResult<TicketInfoResponse> {
    let config = read_config(deps.storage)?;
    let ticket_manager = match config.ticket_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let ticket_info: TicketInfoResponse = deps.querier.query_wasm_smart(
        ticket_manager,
        &to_binary(&TicketQueryMsg::QueryTicketInfo { tid })?,
    )?;
    Ok(ticket_info)
}

// Query the worker assigned on the task (Call the "QueryTicketWorker" of "ticket_manger")
fn query_ticket_worker(deps: Deps, tid: u64) -> StdResult<String> {
    let config = read_config(deps.storage)?;
    let ticket_manager = match config.ticket_manager {
        Some(v) => v,
        None => return Err(TAError::NotInitialized.into()),
    };

    let worker: String = deps.querier.query_wasm_smart(
        ticket_manager,
        &to_binary(&TicketQueryMsg::QueryTicketWorker { tid })?,
    )?;
    Ok(worker)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}
