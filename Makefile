migrate:
	sea-orm-cli migrate refresh

entities:
	sea-orm-cli generate entity \
        -o database/src/entity

fmt:
	cargo +nightly fmt --all
	cargo machete --fix || true
