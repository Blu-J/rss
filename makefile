sql_create:
	sqlx database drop -y
	sqlx database create
	sqlx migrate run

sql_run:
	sqlx migrate run