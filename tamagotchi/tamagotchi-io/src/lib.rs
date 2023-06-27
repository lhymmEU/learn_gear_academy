#![no_std]
use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, exec};
use scale_info::TypeInfo;

const HUNGER_PER_BLOCK: u64 = 1;
const ENERGY_PER_BLOCK: u64 = 2;
const BOREDOM_PER_BLOCK: u64 = 2;
const FILL_PER_SLEEP: u64 = 1000;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;

pub struct ProgramMetadata;

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
   Name,
   Age,
   Feed,
   Play,
   Sleep,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
   Name(String),
   Age(u64),
   Fed,
   Entertained,
   Slept,
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub rested: u64,
    pub rested_block: u64,
}

impl Tamagotchi {
    pub fn fed(&mut self) {
        let hunger: u64 = (exec::block_timestamp() - self.fed_block) * HUNGER_PER_BLOCK / 1000;
        if self.fed > hunger {
            self.fed -= hunger;
        } else {
            self.fed = 1;
        }

        self.fed += FILL_PER_FEED;
        if self.fed > 10000 {
            self.fed = 10000;
        }
        self.fed_block = exec::block_timestamp();
    }

    pub fn entertained(&mut self) {
        let happy: u64 = (exec::block_timestamp() - self.entertained_block) * BOREDOM_PER_BLOCK / 1000;
        if self.entertained > happy {
            self.entertained -= happy;
        } else {
            self.entertained = 1;
        }

        self.entertained += FILL_PER_ENTERTAINMENT;
        if self.entertained > 10000 {
            self.entertained = 10000;
        }
        self.entertained_block = exec::block_timestamp();
    }

    pub fn rested(&mut self) {
        let energy: u64 = (exec::block_timestamp() - self.rested_block) * ENERGY_PER_BLOCK / 1000;
        if self.rested > energy {
            self.rested -= energy;
        } else {
            self.rested = 1;
        }

        self.rested += FILL_PER_SLEEP;
        if self.rested > 10000 {
            self.rested = 10000;
        }
        self.rested_block = exec::block_timestamp();
    }
}

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Tamagotchi;
}
