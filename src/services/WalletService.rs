use wallet_gen::*;

pub struct WalletService;

impl WalletService {
    pub fn createEthWallet() -> Vec<String> {
        let x = ethereum::new_wallet(prelude::Coin::Ethereum).unwrap();
        return vec![x.public_key, x.private_key];
    }
    pub fn createBitcoinWallet() -> Vec<String> {
        let x = bitcoin::new_wallet(prelude::Coin::Bitcoin).unwrap();
        return vec![x.public_key, x.private_key];
    }
}