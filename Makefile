cleandb:
	rm urlclip.db*
createdb:
	sqlx database create
	sqlx migrate run
