#![no_std]
use gstd::{msg, prelude::*, exec, debug};
use codec::{Decode, Encode};
use scale_info::TypeInfo;


#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
}

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

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

// this function will handle the initialization of the contract
#[no_mangle]
extern "C" fn init() {
    unsafe {
        TAMAGOTCHI = Some(Tamagotchi { name: String::from("Tamasheepie"), date_of_birth: exec::block_timestamp() });
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