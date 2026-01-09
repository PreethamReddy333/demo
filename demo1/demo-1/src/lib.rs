use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};
use weil_rs::collections::{map::WeilMap, WeilId};

trait DemoOne {
    fn new() -> Result<Self, String>
    where
        Self: Sized;

    async fn get_list(&self, id: String) -> Result<Vec<u8>, String>;
    async fn set_val(&mut self, id: String, val: u8) -> Result<(), String>;
}

#[derive(Serialize, Deserialize, WeilType)]
pub struct DemoOneContractState {
    map: WeilMap<String, Vec<u8>>,
}

#[smart_contract]
impl DemoOne for DemoOneContractState {
    #[constructor]
    fn new() -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(Self {
            map: WeilMap::new(WeilId(0)),
        })
    }

    #[query]
    async fn get_list(&self, id: String) -> Result<Vec<u8>, String> {
        self.map
            .get(&id)
            .ok_or_else(|| format!("id {} not present in map", id))
    }

    #[mutate]
    async fn set_val(&mut self, id: String, val: u8) -> Result<(), String> {
        let mut list = self.map.get(&id).unwrap_or_else(|| Vec::new());

        list.push(val);

        self.map.insert(id, list);

        Ok(())
    }
}
