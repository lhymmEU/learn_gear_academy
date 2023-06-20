#![no_std]
use gstd::{msg, prelude::*, ActorId};

static mut ESCROW: Option<Escrow> = None;

#[no_mangle]
extern "C" fn init() {
    let init_config: InitEscrow = msg::load().expect("Unable to load init message");
    let escrow = Escrow {
        seller: init_config.seller,
        buyer: init_config.buyer,
        price: init_config.price,
        state: EscrowState::AwaitingPayment,
    };

    unsafe {
        ESCROW = Some(escrow)
    };
}

#[no_mangle]
extern "C" fn handle() {
    let action: EscrowAction = msg::load().expect("Unable to load action message");
    let escrow: &mut Escrow = unsafe {
        ESCROW.as_mut().expect("The contract is not initialized.")
    };

    match action {
        EscrowAction::Deposit => escrow.deposit(),
        EscrowAction::ConfirmDelivery => escrow.confirm_delivery(),
    }
}

#[no_mangle]
extern "C" fn state() {
    let escrow = unsafe {
        ESCROW.get_or_insert(Default::default())
    };
    msg::reply(escrow, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
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

impl Escrow {
    pub fn deposit(&mut self) {
        assert_eq!(
            self.state,
            EscrowState::AwaitingPayment,
            "State must be AwaitingPayment"
        );
        assert_eq!(
            self.buyer,
            msg::source(),
            "The message sender must be a buyer"
        );
        assert_eq!(
            self.price,
            msg::value(),
            "The attached value must be equal to set price"
        );

        self.state = EscrowState::AwaitingDelivery;
        msg::reply(EscrowEvent::FundsDeposited, 0).expect("Error in reply EscrowEvent::FundsDeposited");
    }

    pub fn confirm_delivery(&mut self) {
        assert_eq!(
            self.seller,
            msg::source(),
            "The message sender must be a seller"
        );
        assert_eq!(
            self.state,
            EscrowState::AwaitingDelivery,
            "State must be AwaitingDelivery"
        );

        self.state = EscrowState::Closed;
        msg::send(self.seller, EscrowAction::ConfirmDelivery, self.price).expect("Error in send funds to seller");
        msg::reply(EscrowEvent::DeliveryConfirmed, 0).expect("Error in reply EscrowEvent::DeliveryConfirmed");
    }
}
