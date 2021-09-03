use ic_cdk_macros::*;
use ic_cdk::export::candid;
use ic_cdk::trap;
use ic_cdk::api::call::call_with_payment;
use ic_cdk::export::candid::{CandidType, Principal};


static mut COUNTER: u64 = 99;


#[update]
async fn increment() -> candid::Nat {
    let principal = Principal::from_text("ddzvf-qyaaa-aaaah-aaqxa-cai").unwrap();
    call_with_payment::<(),()>(principal, "trapme", (), 100000000).await;

    let mut sum = 0;
    for i in 1 .. 1000000
    {
        if i%2 == 1
        {
            sum += i;
        }
    }
    candid::Nat::from(sum)
}
