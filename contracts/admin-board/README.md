
Ticket-Auction: admin-board contract.
    This contract plays the role of wrapper or entry point for admin-related functions.
    Admin(wallet) is able to call the methods of other contract.
    For example, if the admin issues the transaction for adding a new ticket here,
    the transaction is forwarded to the corresponding method of "ticket_manager" contract
    by creating new message.

    Admin is able to :
        - Add a new ticket    (ticket_manager)
        - Update ticket       (ticket_manager)
        - Remove ticket       (ticket_manager)
        - Decide winning bet  (auction_manager)

    Admin is also able to query:
        - Ticket info         (ticket_manager)
        - Ticket worker       (ticket_manager)

    Method invoked by other contract:
        - Release stake with slash (from ticket_manager)

    In addition to that, this contract is responsible for instantiation & migration of other contracts.

Further improvements:
    Current contract includes only vital activities for admin wallet.
    This contract should include much more utility methods & queries for admin wallet.

