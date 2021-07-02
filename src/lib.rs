#![allow(dead_code)]

mod generator;
mod schema;
mod types;

#[cfg(test)]
mod tests {
    use crate::generator::ModuleGenerator;
    use crate::schema::Schema;
    use std::error::Error;
    use std::fs::File;

    #[test]
    fn test_json() -> Result<(), Box<dyn Error>> {
        let json = File::open("test-protocol.json").expect("file not found");
        let schema = Schema::new(json).expect("failed");
        let mut generator = ModuleGenerator::new("/tmp/packetgen")?;
        schema.per_state(|name, state| {
            let mut state_gen = generator.emit_state(name)?;
            state.per_packet(|packet| state_gen.emit_packet(&packet))
        })?;
        Ok(())
    }
}
