use copper::common::{Dummy, Executable};
use copper::server::util::Server;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DummyAggregator;

impl Executable<Vec<String>, String> for DummyAggregator {
    fn exec(&self, _args: Vec<String>) -> String {
        String::new()
    }
}

const LOCALHOST: &'static str = "127.0.0.1";
const SERVER_LOAD: usize = 100;

#[test]
fn multiple_clients_load() {
    let _server1 = Server::new(8080, Box::new(Dummy), Box::new(DummyAggregator));
    for _i in 1..SERVER_LOAD {
        copper::client::automatic_session(LOCALHOST, 8080);
    }
}
