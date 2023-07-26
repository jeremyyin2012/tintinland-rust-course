use crate::msg_enum::ToThirdPartyMsg1;
use std::time::{Duration, Instant};
mod msg_enum;

fn run(n: u32) {
    println!("枚举 Enum 版本：");
    let wework_tpl = msg_enum::Template::new_wework_tpl();
    let dingtalk_tpl = msg_enum::Template::new_dingtalk_tpl();
    let feishu_tpl = msg_enum::Template::new_feishu_tpl();
    let msgs = vec![wework_tpl, dingtalk_tpl, feishu_tpl];
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
