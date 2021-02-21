use copper::common::{deserialize, serialize, Dummy, Type, Msg};

// #[derive(Serialize, Deserialize)]
// struct Printer;

// impl Executable<(), ()> for Printer {
//     fn exec(&self, _args: ()) -> () {
//         println!("I am serializable!")
//     }
// }

// #[test]
// #[ignore]
// fn executable_after_serialization_manual() {
//     let printer = Printer;

//     let erased: s::Box<dyn s::Any> = s::Box::new(printer);
//     let serialized = serde_json::to_string(&erased).expect("Unable to serialize the function!");

//     let deserialized: s::Box<dyn s::Any> =
//         serde_json::from_str(&serialized).expect("Unable to deserialize the function!");

//     let downcast: Box<Printer> =
//         Box::<dyn std::any::Any>::downcast(deserialized.into_any()).expect("Unable to downcast");

//     (&downcast).exec(());
// }

// #[derive(Serialize, Deserialize)]
// struct Incrementer;

// impl Executable<(i32, i32), i32> for Incrementer {
//     fn exec(&self, args: (i32, i32)) -> i32 {
//         args.0 + args.1
//     }
// }

// #[test]
// #[ignore]
// fn executable_after_serialization_with_args_manual() {
//     let incrementer = Incrementer;

//     let erased: s::Box<dyn s::Any> = s::Box::new(incrementer);
//     let serialized = serde_json::to_string(&erased).expect("Unable to serialize the function!");

//     let deserialized: s::Box<dyn s::Any> =
//         serde_json::from_str(&serialized).expect("Unable to deserialize the function!");

//     let downcast: Box<Incrementer> =
//         Box::<dyn std::any::Any>::downcast(deserialized.into_any()).expect("Unable to downcast");

//     assert_eq!(Incrementer.exec((3, 5)), (&downcast).exec((3, 5)));
// }

#[test]
fn executable_after_serialization_auto() {
    let msg = Msg {
        type_msg: Type::Task,
        res: "".into(),
        func: Box::new(Dummy),
    };
    let serialized = serialize(msg);
    let deserialized: Box<Msg> = deserialize(serialized);
    deserialized.func.exec(());
}
