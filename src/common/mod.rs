//! All utility functions are in this module

use serde_traitobject;
use serde::{Deserialize, Serialize};

/// Serialization for functions

pub trait Executable<Args, Res>:
    serde_traitobject::Serialize + serde_traitobject::Deserialize
where
    Args: serde_traitobject::Serialize + serde_traitobject::Deserialize,
    Res: serde_traitobject::Serialize + serde_traitobject::Deserialize,
{
    fn exec(&self, args: Args) -> Res;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Type{
    Task,
    Result,
    ThankYou,
}

#[derive(Serialize, Deserialize)]
pub struct Msg {
    pub type_msg: Type,
    pub res: String,
    #[serde(with = "serde_traitobject")]
    pub func: Box<dyn Executable<(), String>>,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Dummy;

impl Executable<(), String> for Dummy {
    fn exec(&self, _: ()) -> String {
        String::new()
    }
}

pub fn serialize(func: Msg) -> String
{
    let erased: serde_traitobject::Box<dyn serde_traitobject::Any> =
        serde_traitobject::Box::new(func);
    let serialized = serde_json::to_string(&erased).expect("Unable to serialize the function!");
    serialized
}

pub fn deserialize(serialized: String) -> Box<Msg>
{
    let deserialized: serde_traitobject::Box<dyn serde_traitobject::Any> =
        serde_json::from_str(&serialized).expect("Unable to deserialize the function!");

    let downcast: Box<Msg> =
        Box::<dyn std::any::Any>::downcast(deserialized.into_any()).expect("Unable to downcast");
    downcast
}
