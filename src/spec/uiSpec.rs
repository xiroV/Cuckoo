use ui::repl::Repl;
use cuckoo;
use cuckoo::config::{Config, ConfigReader};
use std::io::{Cursor, Read, Write};


fn get_repl_instance(services: cuckoo::ServiceBundle) -> Repl<Cursor<Vec<u8>>, Cursor<Vec<u8>>> {
    Repl::new(Cursor::new(Vec::new()), Cursor::new(Vec::new()), services)
}


#[test]
fn repl_can_be_initialized() {

    let service_bundle = cuckoo::ServiceBundle {
        config: Config::new()
    };

    let mut repl = get_repl_instance(service_bundle);

    assert!(repl.get_handlers().is_empty());
    assert!(repl.get_service_bundle().config.accounts.is_empty());
}
