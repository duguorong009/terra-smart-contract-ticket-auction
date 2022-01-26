
Ticket-Auction: User(Worker) Board
  This contract is responsible for fronting the user(worker) and routing the users'
  actions to corresponding contract.
  Its role is similar to controller in MVC model.
  There are following methods and corresponding queries.
    - Lock Stake
        User locks the stake(collateral) in order to place bet on ticket.
        This message is routed to collateral_manager
    - Place Bet
        User place the bet on ticket on which he/she would like to work.
        This message is routed to auction_manager

    - Submit result
        User submits the result of ticket he worked on.
        This message is routed to ticket_manager contract.
      
Further improvements:
  The contract should have much more features since it fronts the user.
  For example, the following functionalities should exist.
    Query the bet status
    Update the bet amount
    Cancel the bet
