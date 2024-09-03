use alloy::primitives::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

pub mod hash;
pub mod user_operation_hash;

use crate::user_operation::{
    hash::get_user_operation_hash_v07, user_operation_hash::UserOperationHash,
};

pub fn as_checksum_addr<S>(val: &Address, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let address_checksum: String = val.to_checksum(None);
    serde::Serialize::serialize(&address_checksum, s)
}

#[derive(
    Default,
    Clone,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct UserOperationV07 {
    #[serde(serialize_with = "as_checksum_addr")]
    pub sender: Address,
    pub nonce: U256,
    pub factory: Option<Address>,
    pub factory_data: Option<Bytes>,
    pub call_data: Bytes,
    pub call_gas_limit: U256,
    pub verification_gas_limit: U256,
    pub pre_verification_gas: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub paymaster: Option<Address>,
    pub paymaster_verification_gas_limit: Option<U256>,
    pub paymaster_post_op_gas_limit: Option<U256>,
    pub paymaster_data: Option<Bytes>,
    pub signature: Bytes,
}

impl UserOperationV07 {
    /// Calculates the hash of the user operation
    pub fn hash(
        &self,
        entry_point: &Address,
        chain_id: u64,
    ) -> eyre::Result<UserOperationHash> {
        get_user_operation_hash_v07(self, entry_point, chain_id)
    }
}

impl UserOperationV07 {
    pub fn mock() -> Self {
        use std::str::FromStr;

        let sender = "0xa3aBDC7f6334CD3EE466A115f30522377787c024"
            .parse::<Address>()
            .unwrap();
        let nonce = U256::from(16);
        let factory: Option<Address> = None;
        let factory_data: Option<Bytes> = None;
        let call_date = Bytes::from_str("b61d27f6000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa9604500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000568656c6c6f000000000000000000000000000000000000000000000000000000").unwrap();

        let max_fee_per_gas = U256::from(17578054897u64);
        let max_priority_fee_per_gas = U256::from(1138018869u64);

        let signature = Bytes::from_str("a15569dd8f8324dbeabf8073fdec36d4b754f53ce5901e283c6de79af177dc94557fa3c9922cd7af2a96ca94402d35c39f266925ee6407aeb32b31d76978d4ba1c").unwrap();
        let call_gas_limit = U256::from(80000);
        let verification_gas_limit = U256::from(68389);
        let pre_verification_gas = U256::from(55721);
        let paymaster = Some(
            "0x0000000000000039cd5e8aE05257CE51C473ddd1"
                .parse::<Address>()
                .unwrap(),
        );
        let paymaster_verification_gas_limit = Some(U256::from(27776));
        let paymaster_post_op_gas_limit = Some(U256::from(1));
        let paymaster_data = Some(Bytes::from_str("00000066cc6b8b000000000000bce787423a07dde9c43cdf50ff33bf35b18babd336cc9739fd9f6dca86e200934505c311454b60c3aa1d206e6bb893f3489e77ace4c58f30d47cebd368a1422a1c").unwrap());

        let user_operation = UserOperationV07 {
            sender: sender,
            nonce: nonce,
            factory: factory,
            factory_data: factory_data,
            call_data: call_date,
            call_gas_limit: call_gas_limit,
            verification_gas_limit: verification_gas_limit,
            pre_verification_gas: pre_verification_gas,
            max_fee_per_gas: max_fee_per_gas,
            max_priority_fee_per_gas: max_priority_fee_per_gas,
            paymaster: paymaster,
            paymaster_verification_gas_limit: paymaster_verification_gas_limit,
            paymaster_post_op_gas_limit: paymaster_post_op_gas_limit,
            paymaster_data: paymaster_data,
            signature: signature,
        };

        user_operation
    }
}
