# minecraft-server-bindings

Generated async Minecraft server protocol bindings from [minecraft-data](https://github.com/PrismarineJS/minecraft-data).

## TODOs
* [ ] Implement missing field types
* [ ] Implement complex packet types with conditional fields
* [ ] Feature for sync/async IO
  * [ ] Generic `Read` and `Write` instead of `Cursor`s and `Vec`s
* [ ] Test all versions

## Adding new versions

* Clone https://github.com/PrismarineJS/minecraft-data
* `cargo run --bin generator -- --protocol-dir minecraft-data/data/pc/$NEW_VERSION --out-dir .`
* Declare feature in [minecraft-server-protocol/Cargo.toml](minecraft-server-protocol/Cargo.toml)
* Add feature guard in [minecraft-server-protocol/src/lib.rs](minecraft-server-protocol/src/lib.rs)
