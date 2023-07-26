use crate::msg_trait::ToThirdPartyMsg2;
use std::time::{Duration, Instant};
mod msg_trait;


fn run(n: u32) {
    println!("\nTrait Object 版本：");
    let wework_tpl = msg_trait::WeworkTpl::new();
    let dingtalk_tpl = msg_trait::DingtalkTpl::new();
    let feishu_tpl = msg_trait::FeishuTpl::new();
    let msgs: Vec<&dyn ToThirdPartyMsg2> = vec![&wework_tpl, &dingtalk_tpl, &feishu_tpl];
    for _ in 0..n {
        for msg in msgs.iter() {
            msg.to_third_party_msg().expect("to_third_party_msg error");
        }
    }
}


fn main() {
    let start = Instant::now();
    run(10000);
    let duration = start.elapsed();
    println!("Time elapsed in run() is: {:?}", duration);
}
