
Ticket-Auction: Collateral Manager
  This contract is in charge of storing & releasing the worker's stake(collateral) for ticket.
  Here, it is assumed that the stake amount is fixed for ticket(eg. 100 uluna for ticket 1, 200 for ticket 2).
  There are 2 methods and corresponding queries.
    LockStake
      Invoked by user_board contract
      Save the stake(collateral) in the contract & record the result.

    ReleaseStake
      Invoked by admin_board contract.
      Release the stake(collateral) for the user.

Further improvements
  Extend the contract with more utilities.
