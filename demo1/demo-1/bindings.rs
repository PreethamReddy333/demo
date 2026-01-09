
use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, secured, smart_contract, WeilType};
use weil_rs::collections::{streaming::ByteStream, plottable::Plottable};
use weil_rs::config::Secrets;
use weil_rs::webserver::WebServer;


trait DemoOne {
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    async fn get_list(&self, id: String) -> Result<Vec<u8>, String>;
    async fn set_val(&mut self, id: String, val: u8) -> Result<(), String>;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct DemoOneContractState {
    // define your contract state here!
}

#[smart_contract]
impl DemoOne for DemoOneContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        unimplemented!();
    }


    #[query]
    async fn get_list(&self, id: String) -> Result<Vec<u8>, String> {
        unimplemented!();
    }

    #[mutate]
    async fn set_val(&mut self, id: String, val: u8) -> Result<(), String> {
        unimplemented!();
    }
}

