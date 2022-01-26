# Ticket Auction Project
This project is about creating the system which allows workers to bet on ticket, 
carry out the ticket task, submit the result & get the reward.

The project consists of 5 contracts.
  Admin_Board       :  admin contract for the system
  User_Board        :  contract for fronting worker(user).
  Auction_Manager   :  manage the auction(bet, win ...)
  Ticket_Manager    :  manage the tickets
  Collateral_Manager:  manage the collateral(stakes).

For the detail, please reference the README.md, which are located in every contract. ([root dir]/contracts/[contract ]/README.md)


# How to deploy & test the project
  1. Build & deploy the "admin-board" contract.
     Use instantiate message.

  2. Build & deploy the "ticket_manager" contract.
      First, upload the contract wasm to LocalTerra & get the code id.
      After that, deploy the contract using "CreateTicketManager" execution in "admin-board" contract. 
      Example: { "CreateTicketManager": { "code_id" : 23 }}
      Record the contract address.

  3. Build & deploy the "user_board" contract
      First, upload the contract wasm to LocalTerra & get the code id.
      After that, deploy the contract using "CreateUsrBoardManager" execution in "admin-board" contract. 
      Example:  { "CreateUsrBoardManager": { "code_id" : 23 }}
      Record the contract address.
    
  4. Before going through next process, register the addresses of 
      contracts above.
      Use the "PostConfig" execution in "admin_board"' contract.
      Example: 
      { 
        "PostConfg" : { 
          "ticket_manager": "terra...",
          "user_board": "terra...",
        }
      }

  5. Build & deploy the "auction_manager" contract
      First, upload the contract wasm to LocalTerra & get the code id.
      After that, deploy the contract using "CreateAuctionManager" execution in "admin-board" contract. 
      Example: { "CreateAuctionManager": { "code_id" : 23 } }
      Record the contract address.
    
  6. Build & deploy the "collateral_manager" contract
      First, upload the contract wasm to LocalTerra & get the code id.
      After that, deploy the contract using "CreateCollateralManager" execution in "admin-board" contract. 
      Example:  { "CreateCollateralManager":  { "code_id" : 23 }}
      Record the contract address.

  7. Finally, register the addresses of installed contracts.
      Use the "PostConfig" execution in "admin_board"' contract.
      Example: {
        "PostConfg": { 
          "ticket_manager": "terra...",
          "user_board": "terra...",
          "collateral_manager": "terra...",
          "auction_manager": "terra...",
        }
      }

