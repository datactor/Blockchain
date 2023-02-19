use super::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_to_bytes(&self.value));

        bytes
    }
}

#[derive(Clone)]
pub struct Transaction {
    pub inputs: Vec<(Output, String)>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn input_value(&self) -> u64 {
        self.inputs
            .iter()
            .map(|input| input.0.value)
            .sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs
            .iter()
            .map(|output| output.value)
            .sum()
    }

    // pub fn input_hashes(&self) -> HashSet<Hash> {
    //     self.inputs
    //         .iter()
    //         .map(|input| input.0.hash())
    //         .collect::<HashSet<Hash>>()
    // }
    //
    // pub fn output_hashes(&self) -> HashSet<Hash> {
    //     self.outputs
    //         .iter()
    //         .map(|output| output.hash())
    //         .collect::<HashSet<Hash>>()
    // }

    // genesis
    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }
}

// transaction 직렬화
impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.0.bytes())
                .collect::<Vec<u8>>()
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>()
        );

        bytes
    }
}