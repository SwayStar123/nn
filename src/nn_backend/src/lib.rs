use std::cell::RefCell;
use std::rc::Rc;

mod network;
mod matrix;

use ic_canister::{
    generate_idl, query, update, Idl, MethodType, Canister, PreUpdate, init
};
use ic_storage::{IcStorage, stable::Versioned};
use ic_exports::ic_cdk::export::candid::{CandidType, Deserialize, Principal};

use network::Network;

#[derive(Default, CandidType, Deserialize, IcStorage)]
pub struct MyCanisterState {
    network: Network,
}

impl Versioned for MyCanisterState {
    type Previous = ();

    fn upgrade((): ()) -> Self {
        Self::default()
    }
}

#[derive(Clone, Canister)]
pub struct MyCanister {
    #[id]
    principal: Principal,

    #[state]
    state: Rc<RefCell<MyCanisterState>>,
}

impl MyCanister {
    #[init]
    fn init(&self) {
        self.state.borrow_mut().network = Network::new(vec![784, 512, 128, 10], 0.005);
    }

    #[update]
    fn single_train(&self, input: Vec<f64>, labels: Vec<f64>) {
        self.state.borrow_mut().network.single_train(input, labels);
    }

    #[update]
    fn train(&self, input: Vec<Vec<f64>>, labels: Vec<Vec<f64>>, epochs: u16) {
        self.state.borrow_mut().network.train(input, labels, epochs);
    }

    #[query]
    fn predict(&self, input: Vec<f64>) -> Vec<f64> {
        self.state.borrow().network.predict(input)
    }

    pub fn idl() -> Idl {
        generate_idl!()
    }
}

impl PreUpdate for MyCanister {
    fn pre_update(&self, _method_name: &str, _method_type: MethodType) {
        
    }
}