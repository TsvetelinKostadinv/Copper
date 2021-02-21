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

#[test]
fn start_parallell_servers() {
    let _server1 = Server::new(8080, Box::new(Dummy), Box::new(DummyAggregator));
    let _server2 = Server::new(8080, Box::new(Dummy), Box::new(DummyAggregator));
}
