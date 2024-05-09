#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};

use crate::state::{
    read_bets_ticket, read_config, read_curr_avail_tickets, remove_bets_ticket, save_bets_ticket,
    save_config, BetDetail, Config,
};
use ticket_auction::ticket_manager::{
    ExecuteMsg as TicketExecuteMsg, QueryMsg as TicketQueryMsg, TicketInfoResponse,
    TicketWorkerPair, TicketsResponse,
};
use ticket_auction::{
    auction_manager::{ExecuteMsg, InstantiateMsg, MigrateMsg, PlaceBetMsg, QueryMsg},
    error::TAError,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Save the config
    let config = Config {
        admin_board: info.sender.to_string(),
        ticket_manager: msg.ticket_manager,
    };

    save_config(deps.storage, config)?;

    // Initialize variables
    save_bets_ticket(deps.storage, 0, vec![])?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        // Transaction initiated by user_board contract.
        ExecuteMsg::PlaceBet(msg) => place_bet(deps, info, msg),

        // Transaction intiated by admin_board contract.
        ExecuteMsg::DecideWinningBet { tid } => decide_winning_bet(deps, env, info, tid),
    }
}

// Place the bet for the given ticket id
// Invoked by user_board contract
fn place_bet(deps: DepsMut, info: MessageInfo, msg: PlaceBetMsg) -> StdResult<Response> {
    // Validation 1: Check if funds are provided.
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Validation 2: Check if the ticket id is valid.
    let config = read_config(deps.storage)?;
    let ticket_manager = config.ticket_manager;
    let tickets: TicketsResponse = deps.querier.query_wasm_smart(
        ticket_manager,
        &to_json_binary(&TicketQueryMsg::QueryTickets {})?,
    )?;
    let is_exist = tickets.tickets.iter().any(|v| v.id == msg.ticket_id);
    if !is_exist {
        return Err(TAError::NotFound.into());
    }

    // Validation 3: Check if tx sender is valid wallet
    if deps.api.addr_canonicalize(info.sender.as_str()).is_err() {
        return Err(TAError::InvalidAddress.into());
    }

    // Save bet.
    let mut bets = read_bets_ticket(deps.storage, msg.ticket_id)?;
    bets.push(BetDetail {
        worker: info.sender,
        bet_amt: msg.bet_amount,
    });
    save_bets_ticket(deps.storage, msg.ticket_id, bets)?;

    Ok(Response::new().add_attributes(vec![attr("method", "place_bet")]))
}

// Decide the winning bet for given ticket id
// Invoked by admin_board contract
fn decide_winning_bet(deps: DepsMut, env: Env, info: MessageInfo, tid: u64) -> StdResult<Response> {
    // Validation 1. Check if tx sender is admin.
    let config = read_config(deps.storage)?;
    if info.sender.to_string() != config.admin_board {
        return Err(TAError::NotAuthorized.into());
    }

    // Validation 2. Check if any funds are provided.
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Validation 3. Given ticket id is valid for decision(If bet_finish_timestamp is passed)
    let ticket_info: TicketInfoResponse = deps.querier.query_wasm_smart(
        config.ticket_manager.clone(),
        &to_json_binary(&TicketQueryMsg::QueryTicketInfo { tid })?,
    )?;
    if env.block.time.seconds() < ticket_info.bet_finish_timestamp {
        return Err(TAError::BetNotFinished.into());
    }

    // Decide winning bet
    // Get the bets for the ticket
    let curr_bets = read_bets_ticket(deps.storage, tid)?;

    // Choose the winning bet(lowest bet amount)
    let winning_bet = match curr_bets.iter().min_by(|x, y| x.bet_amt.cmp(&y.bet_amt)) {
        Some(v) => v,
        None => return Err(TAError::NotFound.into()),
    };

    // Clear the bets data & prepare the msgs to return collaterals.
    remove_bets_ticket(deps.storage, tid)?;
    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.ticket_manager,
        msg: to_json_binary(&TicketExecuteMsg::SaveTicketWorker(TicketWorkerPair {
            tid,
            worker: winning_bet.worker.to_string(),
        }))?,
        funds: vec![],
    })];

    // TODO: Prepare msgs to release the stakes of failed bet.

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "win_bet")]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BetAvailableTickets => to_json_binary(&query_bet_avail_tickets(deps)?),
        QueryMsg::CurrActiveBets { tid } => to_json_binary(&query_curr_active_bets(deps, tid)?),
    }
}

// Query the bet available tickets.
fn query_bet_avail_tickets(deps: Deps) -> StdResult<Vec<u64>> {
    // Query the tickets(ids) which currently have bets.
    let keys = read_curr_avail_tickets(deps.storage)?;
    Ok(keys)
}

// Query the current bets for ticket id.
fn query_curr_active_bets(deps: Deps, tid: u64) -> StdResult<Vec<BetDetail>> {
    // Query the current bets for ticket(tid)
    let curr_bets = read_bets_ticket(deps.storage, tid)?;
    Ok(curr_bets)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
