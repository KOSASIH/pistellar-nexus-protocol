# Pi API Documentation

The Pi API provides a set of endpoints for interacting with the Pi network.

## Endpoints

### `GET /pi/account`

Returns information about a Pi account.

* **Parameters**:
	+ `account_id`: The ID of the Pi account.
* **Response**:
	+ `account_info`: Information about the Pi account.

### `POST /pi/transaction`

Creates a new Pi transaction.

* **Parameters**:
	+ `source_account_id`: The ID of the source Pi account.
	+ `destination_account_id`: The ID of the destination Pi account.
	+ `amount`: The amount of Pi to transfer.
* **Response**:
	+ `transaction_id`: The ID of the created transaction.

### `GET /pi/balance`

Returns the balance of a Pi account.

* **Parameters**:
	+ `account_id`: The ID of the Pi account.
* **Response**:
	+ `balance`: The balance of the Pi account.
