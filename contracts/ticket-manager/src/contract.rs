#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, WasmMsg,
};

use crate::state::{
    read_all_assigned_tickets, read_config, read_ticket_for_id, read_tickets,
    read_worker_for_ticket, store_config, store_tickets, store_tw_pair, Config,
};
use ticket_auction::admin_board::{ExecuteMsg as AdminExecuteMsg, SlashMsg};
use ticket_auction::error::TAError;
use ticket_auction::ticket_manager::{
    AddTicketMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, PostConfigMsg, QueryMsg, TWPairsReponse,
    Ticket, TicketInfoResponse, TicketResultMsg, TicketWorkerPair, UpdateTicketMsg,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Validation 1: Check if funds are provided.
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }

    // Store the tx initiator as system admin.
    let admin_board = info.sender.to_string();
    store_config(
        deps.storage,
        Config {
            admin_board,
            auction_manager: msg.auction_manager,
            user_board: msg.user_board,
        },
    )?;

    // Initialize the variables
    store_tickets(deps.storage, vec![])?;
    store_tw_pair(
        deps.storage,
        TicketWorkerPair {
            tid: 0,
            worker: "".to_string(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("contract", env.contract.address)
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        // Transacions initiated by admin(contract).
        ExecuteMsg::AddTicket(msg) => add_ticket(deps, env, info, msg),
        ExecuteMsg::UpdateTicket(msg) => update_ticket(deps, env, info, msg),
        ExecuteMsg::RemoveTicket { tid } => remove_ticket(deps, env, info, tid),

        // Transaction initiated by user_board.
        ExecuteMsg::AssessSubmission(msg) => assess_submission(deps, env, info, msg),

        // Transaction initiated by auction contract. (admin -> auction -> here)
        ExecuteMsg::SaveTicketWorker(msg) => save_ticket_worker(deps, info, msg),

        // Utilities
        ExecuteMsg::PostConfig(msg) => execute_post_config(deps, env, info, msg),
    }
}

// Add new ticket info to the storage
fn add_ticket(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: AddTicketMsg,
) -> StdResult<Response> {
    // Validate if the tx sender is admin.
    let config = read_config(deps.storage)?;
    if info.sender != config.admin_board {
        return Err(TAError::NotAuthorized.into());
    }

    // Store ticket
    let mut tickets = read_tickets(deps.storage)?;
    tickets.push(Ticket {
        id: msg.id,
        bet_finish_timestamp: msg.bet_finish_timestamp,
        close_timestamp: msg.close_timestamp,
        result: msg.result,
        collateral: msg.collateral,
    });
    store_tickets(deps.storage, tickets)?;
    Ok(Response::new().add_attributes(vec![
        attr("method", "store_ticket"),
        attr("result", "success"),
    ]))
}

// Update the ticket with given info.
fn update_ticket(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateTicketMsg,
) -> StdResult<Response> {
    // Validate if the tx sender is admin.
    let config = read_config(deps.storage)?;
    if info.sender != config.admin_board {
        return Err(TAError::NotAuthorized.into());
    }

    let ticket_id = msg.id;
    // Get the ticket with "id"
    let ticket_info = query_ticket(deps.as_ref(), ticket_id)?;

    let mut ticket = Ticket {
        id: ticket_info.id,
        bet_finish_timestamp: ticket_info.bet_finish_timestamp,
        close_timestamp: ticket_info.close_timestamp,
        result: ticket_info.result,
        collateral: ticket_info.collateral,
    };

    // Update the ticket info.
    if let Some(bet_finish_timestamp) = msg.bet_finish_timestamp {
        ticket.bet_finish_timestamp = bet_finish_timestamp;
    }
    if let Some(close_timestamp) = msg.close_timestamp {
        ticket.close_timestamp = close_timestamp;
    }
    if let Some(collateral) = msg.collateral {
        ticket.collateral = collateral;
    }
    if let Some(result) = msg.result {
        ticket.result = result;
    }

    // Update the tickets
    let tickets = read_tickets(deps.storage)?;
    let tickets = tickets
        .into_iter()
        .map(|t| if t.id == ticket_id { ticket.clone() } else { t })
        .collect::<Vec<Ticket>>();

    // Store the tickets
    store_tickets(deps.storage, tickets)?;
    Ok(Response::new().add_attribute("method", "update_ticket"))
}

// Remove the ticket from tickets with ticket id.
fn remove_ticket(deps: DepsMut, _env: Env, info: MessageInfo, id: u64) -> StdResult<Response> {
    // Validate if the tx sender is admin.
    let config = read_config(deps.storage)?;
    if info.sender != config.admin_board {
        return Err(TAError::NotAuthorized.into());
    }

    // Remove the ticket with id
    let tickets = read_tickets(deps.storage)?;
    let tickets = tickets
        .into_iter()
        .filter(|t| t.id != id)
        .collect::<Vec<Ticket>>();

    // Store the tickets
    store_tickets(deps.storage, tickets)?;
    Ok(Response::new().add_attribute("method", "remove_ticket"))
}

fn save_ticket_worker(
    deps: DepsMut,
    info: MessageInfo,
    msg: TicketWorkerPair,
) -> StdResult<Response> {
    // Validation 1: Check if the tx sender is "auction" address.
    let config = read_config(deps.storage)?;
    if config.auction_manager.is_none() {
        return Err(TAError::NotInitialized.into());
    }
    let auction_manager = config.auction_manager.unwrap();
    if info.sender != auction_manager {
        return Err(TAError::NotAuthorized.into());
    }

    // Validation 2: Check if ticket id is valid.
    let tickets = read_tickets(deps.storage)?;
    let is_ticket_exist = tickets.iter().any(|t| t.id == msg.tid);
    if !is_ticket_exist {
        return Err(TAError::NotFound.into());
    }

    // Save the ticket-worker pair(winning_bet)
    store_tw_pair(deps.storage, msg)?;

    Ok(Response::new().add_attributes(vec![attr("method", "save ticket-worker pair")]))
}

fn assess_submission(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: TicketResultMsg,
) -> StdResult<Response> {
    let config = read_config(deps.storage)?;
    // Validation 1. Check if funds is provided
    if !info.funds.is_empty() {
        return Err(TAError::UnnecessaryFunds.into());
    }
    // Validation 2.Check if the tx sender is front bot

    // Validation 3. Check if submitter is right worker.
    let assignee = read_worker_for_ticket(deps.storage, msg.tid)?;
    if msg.worker != assignee {
        return Err(TAError::NotAuthorized.into());
    }

    // Check the result & submission timestamp
    let slash_perc = Uint128::from(0u128);
    let ticket = match read_ticket_for_id(deps.storage, msg.tid) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    // Apply 50% slash when result no match.
    if ticket.result != msg.result {
        slash_perc.checked_add(Uint128::from(500u128))?;
    }
    // Apply 30% slash when timestamp passed.
    let timestamp = env.block.time.seconds();
    if timestamp > ticket.close_timestamp {
        slash_perc.checked_add(Uint128::from(300u128))?;
    }

    // Create msg to be sent to admin contract for applying slash perc.
    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.admin_board,
        msg: to_json_binary(&AdminExecuteMsg::ReleaseStakeWithSlash(SlashMsg {
            tid: msg.tid,
            worker: deps.api.addr_validate(msg.worker.as_str())?,
            slash_perc,
        }))?,
        funds: vec![],
    })];

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("method", "assess submission")]))
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
    if info.sender != config.admin_board {
        return Err(TAError::NotAuthorized.into());
    }

    config.auction_manager = msg.auction_manager;
    config.user_board = msg.user_board;

    store_config(deps.storage, config)?;

    Ok(Response::new().add_attribute("method", "PostConfig"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryTicketInfo { tid } => to_json_binary(&query_ticket(deps, tid)?),
        QueryMsg::QueryTickets {} => to_json_binary(&query_tickets(deps)?),
        QueryMsg::QueryTicketWorkerPairs {} => to_json_binary(&query_ticket_worker_pairs(deps)?),
        QueryMsg::QueryTicketWorker { tid } => to_json_binary(&query_ticket_worker(deps, tid)?),
    }
}

fn query_ticket(deps: Deps, id: u64) -> StdResult<TicketInfoResponse> {
    let tickets = read_tickets(deps.storage)?;
    let ticket = match tickets.into_iter().find(|t| t.id == id) {
        Some(t) => t,
        None => return Err(TAError::NotFound.into()),
    };
    Ok(TicketInfoResponse {
        id: ticket.id,
        bet_finish_timestamp: ticket.bet_finish_timestamp,
        close_timestamp: ticket.close_timestamp,
        result: ticket.result,
        collateral: ticket.collateral,
    })
}

fn query_tickets(deps: Deps) -> StdResult<Vec<Ticket>> {
    read_tickets(deps.storage)
}

fn query_ticket_worker_pairs(deps: Deps) -> StdResult<TWPairsReponse> {
    let tids = read_all_assigned_tickets(deps.storage)?;
    let tw_pairs = tids
        .into_iter()
        .map(|tid| {
            let worker = read_worker_for_ticket(deps.storage, tid).unwrap();
            TicketWorkerPair { tid, worker }
        })
        .collect::<Vec<TicketWorkerPair>>();
    Ok(TWPairsReponse { pairs: tw_pairs })
}

fn query_ticket_worker(deps: Deps, tid: u64) -> StdResult<String> {
    let worker = match read_worker_for_ticket(deps.storage, tid) {
        Ok(w) => w,
        Err(_) => return Err(TAError::NotFound.into()),
    };
    Ok(worker)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
