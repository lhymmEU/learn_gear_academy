#![no_std]
use gstd::{msg, prelude::*, exec, debug, ActorId};
use codec::{Decode, Encode};
use scale_info::TypeInfo;

const HUNGER_PER_BLOCK: u64 = 1;
const ENERGY_PER_BLOCK: u64 = 2;
const BOREDOM_PER_BLOCK: u64 = 2;
const FILL_PER_SLEEP: u64 = 1000;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;


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
        let hunger: u64 = (exec::block_height() as u64 - self.fed_block) * HUNGER_PER_BLOCK;
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
        let happy: u64 = (exec::block_height() as u64 - self.entertained_block) * BOREDOM_PER_BLOCK;
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
        let energy: u64 = (exec::block_height() as u64 - self.rested_block) * ENERGY_PER_BLOCK;
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

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

// this function will handle the initialization of the contract
#[no_mangle]
extern "C" fn init() {
    unsafe {
        TAMAGOTCHI = Some(Tamagotchi { 
            name: String::from("Tamasheepie"), 
            date_of_birth: exec::block_timestamp(),
            owner: msg::source(),
            fed: 1,
            fed_block: exec::block_timestamp(),
            entertained: 1,
            entertained_block: exec::block_timestamp(),
            rested: 10000, 
            rested_block: exec::block_timestamp(),
        });
    }

    debug!("Tamagotchi created!");

    msg::reply(String::from("Tamagotchi created!"), 0).expect("Tamagotchi creation failed.");
}

// this function will handle the execution of the contract
#[no_mangle]
extern "C" fn handle() {
    let input_action: TmgAction = msg::load().expect("Error in loading tmg action.");
    let gotchi = unsafe {
        TAMAGOTCHI.as_mut().expect("Tamagotchi not initialized.")
    };
    let gotchi_name = gotchi.name.clone();
    let gotchi_age = exec::block_timestamp() - gotchi.date_of_birth;
    match input_action {
        TmgAction::Name => {
            debug!("Tamagotchi name requested!");
            msg::reply(TmgEvent::Name(gotchi_name), 0).expect("Name request failed.");
        },
        TmgAction::Age => {
            debug!("Tamagotchi age requested!");
            msg::reply(TmgEvent::Age(gotchi_age), 0).expect("Age request failed.");
        },
        TmgAction::Feed => {
            debug!("Tamagotchi fed!");
            gotchi.fed();
            msg::reply(TmgEvent::Fed, 0).expect("Feed request failed.");
        },
        TmgAction::Play => {
            debug!("Tamagotchi entertained!");
            gotchi.entertained();
            msg::reply(TmgEvent::Entertained, 0).expect("Play request failed.");
        },
        TmgAction::Sleep => {
            debug!("Tamagotchi slept!");
            gotchi.rested();
            msg::reply(TmgEvent::Slept, 0).expect("Sleep request failed.");
        },
    }
}

#[no_mangle]
extern "C" fn state() {
    let gotchi = unsafe {
        TAMAGOTCHI.as_mut().expect("Tamagotchi not initialized.")
    };

    msg::reply(gotchi, 0).expect("State request failed.");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0)
        .expect("Failed to share metahash");
}