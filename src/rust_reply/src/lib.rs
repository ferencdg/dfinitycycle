use ic_cdk_macros::*;
use ic_cdk::export::candid;
use ic_cdk::*;


#[update]
fn trapme() -> () {
    let available = api::call::msg_cycles_available();
    let accepted = api::call::msg_cycles_accept(available);
}
