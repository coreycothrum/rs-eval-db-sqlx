## usage
### development
compilation may fail if database does not exist or migrations are out of sync w/ code

#### migrate manually
```sh
# cli must be installed already
cargo install sqlx-cli

cargo sqlx database create
cargo sqlx migrate run --source database/migrations/
```

#### create a new migration
```sh
cargo sqlx migrate add -r --timestamp --source database/migrations/
```
