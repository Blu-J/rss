sql_create:
	sqlx database drop -y
	sqlx database create
	sqlx migrate run

sql_run:
	sqlx migrate run

dev:
	cargo watch -s "pkill -9 rss" -cx "run"

install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	cargo install cargo-watch
	cargo install sqlx-cli