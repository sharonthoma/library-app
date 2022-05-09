# Near Library App
- This a library project in which a user can signup for the library and the user can be able to borrow books and return them.

## Required Software
- Rust + cargo
- Node + npm
- NEAR CLI 3.1

## Quick near calls from the terminal

1. **Register for the library** - `near call sharonthoma.testnet add_user '{"username": "<myusername>", "fullname": "<My Full Name>", "address": "<Box 3232 St>"}' --accountId youraccount.testnet`

2. **Register a new book** - `near call sharonthoma.testnet add_book '{"title": "<Book title>", "author": "<Book author>", "available": "<200>"}' --accountId youraccount.testnet`

3. **Borrow a book** - `near call sharonthoma.testnet add_book '{"user_id": <1>, "book_id": <0>, "available": "<200>"}' --accountId youraccount.testnet`

4. **Get my borrowed books** - `near call sharonthoma.testnet get_borrowed_books --accountId youraccount.testnet`

5. **Get book** - `near call sharonthoma.testnet get_book '{"id": <100>}' --accountId youraccount.testnet`

6. **Delete book** - `near call sharonthoma.testnet delete_book '{"id": <100>}' --accountId youraccount.testnet`

7. **Get user** - `near call sharonthoma.testnet get_user '{"user_id": <10>}' --accountId youraccount.testnet`

8. **Delete user** - `near call sharonthoma.testnet delete_user '{"user_id": <10>}' --accountId youraccount.testnet`

9. **Delete borrower** - `near call sharonthoma.testnet delete_borrower '{"id": <5>}' --accountId youraccount.testnet`

10. **Return book** - `near call sharonthoma.testnet return_book '{"user_id": <80>, "borrow_id": <4>}' --accountId youraccount.testnet`

# Author
Sharon Thomas
