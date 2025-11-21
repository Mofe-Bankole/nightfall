use std::str::FromStr;

pub async fn transfer(amount : u64 , recipient: &str){
    let transparent_address_recipient = zcash_address::ZcashAddress::try_from_encoded(recipient)
        .unwrap()
        .convert_to_transparent()
        .unwrap();
}
