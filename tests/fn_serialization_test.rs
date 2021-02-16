use serde::{Deserialize, Serialize};
use serde_traitobject as s;

use copper::util::{deserialize, serialize, Executable};

#[derive(Serialize, Deserialize)]
struct Printer;

impl Executable<(), ()> for Printer {
    fn exec(&self, _args: ()) -> () {
        println!("I am serializable!")
    }
}

#[test]
fn executable_after_serialization_manual() {
    let printer = Printer;

    let erased: s::Box<dyn s::Any> = s::Box::new(printer);
    let serialized = serde_json::to_string(&erased).expect("Unable to serialize the function!");

    let deserialized: s::Box<dyn s::Any> =
        serde_json::from_str(&serialized).expect("Unable to deserialize the function!");

    let downcast: Box<Printer> =
        Box::<dyn std::any::Any>::downcast(deserialized.into_any()).expect("Unable to downcast");

    (&downcast).exec(());
}

#[derive(Serialize, Deserialize)]
struct Incrementer;

impl Executable<(i32, i32), i32> for Incrementer {
    fn exec(&self, args: (i32, i32)) -> i32 {
        args.0 + args.1
    }
}

#[test]
fn executable_after_serialization_with_args_manual() {
    let incrementer = Incrementer;

    let erased: s::Box<dyn s::Any> = s::Box::new(incrementer);
    let serialized = serde_json::to_string(&erased).expect("Unable to serialize the function!");

    let deserialized: s::Box<dyn s::Any> =
        serde_json::from_str(&serialized).expect("Unable to deserialize the function!");

    let downcast: Box<Incrementer> =
        Box::<dyn std::any::Any>::downcast(deserialized.into_any()).expect("Unable to downcast");

    assert_eq!(Incrementer.exec((3, 5)), (&downcast).exec((3, 5)));
}

#[test]
fn executable_after_serialization_no_args_auto() {
    let func = Printer;
    let serialized = serialize(func);
    let deserialized: Box<Printer> = deserialize(serialized);
    deserialized.exec(());
}

#[test]
fn executable_after_serialization_with_args_auto() {
    let func = Incrementer;
    let serialized = serialize(func);
    let deserialized: Box<Incrementer> = deserialize(serialized);
    assert_eq!(deserialized.exec((1, 2)), 3);
}
