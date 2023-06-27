#![no_std]
use gstd::{msg, prelude::*, exec, debug};
use tamagotchi_io::{TmgAction, TmgEvent, Tamagotchi};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

// this function will handle the initialization of the contract
#[no_mangle]
extern "C" fn init() {
    unsafe {
        TAMAGOTCHI = Some(Tamagotchi { 
            name: String::from("Tamasheepie"), 
            date_of_birth: exec::block_timestamp(),
            owner: msg::source(),
            fed: 10000 as u64,
            fed_block: exec::block_timestamp(),
            entertained: 10000 as u64,
            entertained_block: exec::block_timestamp(),
            rested: 10000 as u64, 
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
    
    match input_action {
        TmgAction::Name => {
            debug!("Tamagotchi name requested!");
            let gotchi_name = gotchi.name.clone();
            msg::reply(TmgEvent::Name(gotchi_name), 0).expect("Name request failed.");
        },
        TmgAction::Age => {
            debug!("Tamagotchi age requested!");
            let gotchi_age = exec::block_timestamp() - gotchi.date_of_birth;
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