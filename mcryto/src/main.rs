use pretty_hex::PrettyHex;
use mcryto::{cmd_sha256, cmd_aes128_gcm_tag};
fn main() {

    // let argtest = "heheheheh";
    // println!("input:");
    // println!("{:?}", argtest.hex_dump());
    // let sha_res = cmd_sha256(&argtest);

    // println!("result:");
    // println!("{:?}", sha_res.hex_dump());

    // let arg2:Vec<u8> = vec![1,2,3];
    // let res = cmd_sha256(&arg2);

    // println!("input:");
    // println!("{:?}", arg2.hex_dump());
    // println!("result2:");
    // println!("{:?}", res.hex_dump());

    let result = cmd_aes128_gcm_tag("hello", "toot", "", "unix is king");

    println!("aes:");
    println!("{:?}", result.out.hex_dump());



}


