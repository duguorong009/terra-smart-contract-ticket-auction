
Ticket-Auction: Ticket Manager
  This contract is for managing the tickets, ticket-worker pairs.
  The contract is also responsible for assessing the ticket result & 
  initiate the msgs of releasing stake(collateral).
  There are following methods:
    - Add Ticket
        Store the ticket in the storage
    - Remove Ticket
        Remove the ticket from the storage
    - Update Ticket
        Update the ticket content.

    - AssessSubmission
        Assess the worker submission(ticket result & consumed time), apply the slash
        & create the stake release messages.

    - Save ticket-worker pair
        Save a pair of the ticket and its assignee(worker) in the storage. 

Further improvements:
    Extend the contract with more features.
