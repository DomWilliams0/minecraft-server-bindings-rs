## Adding new versions

* Clone <https://github.com/PrismarineJS/minecraft-data>
* `cargo run --bin generator-bin -- --protocol-dir minecraft-data/data/pc/$NEW_VERSION -o .`
* Declare feature in <packets/Cargo.toml>
* Add feature guard in <packets/src/lib.rs>
