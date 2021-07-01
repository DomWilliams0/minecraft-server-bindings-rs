mod schema;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Schema;
    use std::fs::File;

    #[test]
    fn test_json() {
        let json = File::open("test-protocol.json").expect("file not found");
        let mut schema = Schema::new(json).expect("failed");
        schema.per_state(|name, state| {
            eprintln!("state {}", name);
            state.per_clientbound(|packet| {
                eprintln!("client {:?}", packet);
            });
            state.per_serverbound(|packet| {
                eprintln!("server {:?}", packet);
            });
        });

        assert!(false);
    }
}
