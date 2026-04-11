pub struct ContractConfig {
    pub exchange: String,
    pub neg_risk_exchange: String,
    pub collateral: String,
    pub conditional_tokens: String,
}

/// Returns the contract configuration for the given chain.
///
/// V2 migration: `exchange` and `neg_risk_exchange` now point to the CTF
/// Exchange V2 contracts. The caller picks the right address based on the
/// market's `neg_risk` flag at order time. Collateral is the new Polymarket
/// USD token, replacing the old USDC.e address.
pub fn get_contract_config(chain_id: u64) -> Option<ContractConfig> {
    match chain_id {
        137 => Some(ContractConfig {
            exchange: "0xE111180000d2663C0091e4f400237545B87B996B".to_owned(),
            neg_risk_exchange: "0xe2222d279d744050d28e00520010520000310F59".to_owned(),
            collateral: "0xC011a7E12a19f7B1f670d46F03B03f3342E82DFB".to_owned(),
            conditional_tokens: "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045".to_owned(),
        }),
        80002 => Some(ContractConfig {
            exchange: "0xE111180000d2663C0091e4f400237545B87B996B".to_owned(),
            neg_risk_exchange: "0xe2222d279d744050d28e00520010520000310F59".to_owned(),
            collateral: "0xC011a7E12a19f7B1f670d46F03B03f3342E82DFB".to_owned(),
            conditional_tokens: "0x69308FB512518e39F9b16112fA8d994F4e2Bf8bB".to_owned(),
        }),
        _ => None,
    }
}
