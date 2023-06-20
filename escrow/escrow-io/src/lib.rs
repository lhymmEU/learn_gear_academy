#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitEscrow>;
    type Handle = InOut<EscrowAction, EscrowEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Escrow;
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowAction {
    Deposit,
    ConfirmDelivery,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowEvent {
    FundsDeposited,
    DeliveryConfirmed,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum EscrowState {
    AwaitingPayment,
    AwaitingDelivery,
    Closed,
}

impl Default for EscrowState {
    fn default() -> Self {
        EscrowState::AwaitingPayment
    }
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Escrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
    pub state: EscrowState,
}

// impl Escrow {
//     pub fn deposit(&mut self) {
//         assert_eq!(
//             self.state,
//             EscrowState::AwaitingPayment,
//             "State must be AwaitingPayment"
//         );
//         assert_eq!(
//             self.buyer,
//             msg::source(),
//             "The message sender must be a buyer"
//         );
//         assert_eq!(
//             self.price,
//             msg::value(),
//             "The attached value must be equal to set price"
//         );

//         self.state = EscrowState::AwaitingDelivery;
//         msg::reply(EscrowEvent::FundsDeposited, 0).expect("Error in reply EscrowEvent::FundsDeposited");
//     }

//     pub fn confirm_delivery(&mut self) {

//     }
// }