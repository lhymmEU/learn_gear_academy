use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*};
use scale_info::TypeInfo;
pub struct ProgramMetadata;

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
   Name,
   Age,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
   Name(String),
   Age(u64),
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
}

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Tamagotchi;
}
