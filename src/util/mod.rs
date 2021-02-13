//! All utility functions are in this module

use serde_traitobject as s;

/// Serialization for functions

pub trait Executable<Args, Res>: s::Serialize + s::Deserialize
where
    Args: s::Serialize + s::Deserialize,
    Res: s::Serialize + s::Deserialize,
{
    fn exec(&self, args: Args) -> Res;
}

pub fn serialize<Args, Res, T>(func: T) -> String
where
    Args: s::Serialize + s::Deserialize,
    Res: s::Serialize + s::Deserialize,
    T: Executable<Args, Res> + 'static,
{
    let erased: s::Box<dyn s::Any> = s::Box::new(func);
    let serialized = serde_json::to_string(&erased).expect("Unable to serialize the function!");
    serialized
}

pub fn deserialize<Args, Res, T>(serialized: String) -> Box<T>
where
    Args: s::Serialize + s::Deserialize,
    Res: s::Serialize + s::Deserialize,
    T: Executable<Args, Res> + 'static,
{
    let deserialized: s::Box<dyn s::Any> =
        serde_json::from_str(&serialized).expect("Unable to deserialize the function!");

    let downcast: Box<T> =
        Box::<dyn std::any::Any>::downcast(deserialized.into_any()).expect("Unable to downcast");
    downcast
}
