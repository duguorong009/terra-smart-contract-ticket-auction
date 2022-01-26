Ticket-Auction: Auction Manager
  This contract is responsible for auction activity(user places the bet & admin decides winning bet).
  This contract is instantiated by admin_board contract.
  There are 2 methods which can be used for manaing auction.
  Also, it includes the necesary queries.
  
  -  Place Bet   
  -     Invoked by user_board contract.
  -     Record the bet for ticket
  
  -  Decide winning bet
        Invoked by admin_board contract
        Decide the winning bet & remove the bet history.
        Record the ticket-work pair (call the method in ticket_manager)
  
Further improvements:
  Expand the contract with more utility features.
