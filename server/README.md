# Open-Kahoot Server

## Installation

1. Install libpq
    ```sh
		apt-get install libpq-dev
	```
2. Install diesel-cli
	```sh
		cargo install diesel_cli --no-default-features --features postgres
	```
3. Copy the .env.example file
	```sh
		cp .env.example .env
	```
4. Set the database url in .env to your postgres server
5. Run migrations
	```sh
		diesel run migrations
	```
6. Run the project
	```sh
		cargo run
	```