# minecraft-server-bindings

Generated async Minecraft server protocol bindings from [minecraft-data](https://github.com/PrismarineJS/minecraft-data).

## TODOs
* [ ] Implement missing field types
* [ ] Implement complex packet types with conditional fields
* [ ] Feature for sync/async
* [ ] Test all versions

## Adding new versions

* Clone https://github.com/PrismarineJS/minecraft-data
* `cargo run --bin generator -- --protocol-dir minecraft-data/data/pc/$NEW_VERSION -o .`
* `cargo fmt`
* Declare feature in [packets/Cargo.toml](packets/Cargo.toml)
* Add feature guard in [packets/src/lib.rs](packets/src/lib.rs)
