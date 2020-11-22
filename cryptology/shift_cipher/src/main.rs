const K: u8 = 3;
const N: u8 = 26;

use std::io;

// E_k(x) = (x + k) mod 26
// this k is 3
fn encode(statement_str: &str) -> String {

    let statement_vec : Vec<u8> = statement_str
        .chars()
        .map(|val| {
            let val = val as u8;
            let val = val - 97;
            val
        }).collect();
    
    let closure = |val: &u8| {
        let res = (val + K) % N; // k = 3
        res
    };

    let computer_after_statement_vec: Vec<u8> = statement_vec
        .iter()
        .map(
            closure
        ).collect();

    let result : String = computer_after_statement_vec
        .iter()
        .map(|val| {
            let val = val + 65;
            let val = val as char;
            val
        }).collect();
    result
}

// D_k(y) = (y - k) mod 26
fn decode(statement_str: &str) -> String {

    let statement_vec : Vec<u8> = statement_str
        .chars()
        .map(|val| {
            let val = val as u8;
            let val = val - 65;
            val
        }).collect();
    
    let closure = |val: &u8| {
        let res = (val - K) % N; // k = 3
        res
    };

    let computer_after_statement_vec: Vec<u8> = statement_vec
        .iter()
        .map(
            closure
        ).collect();

    let result : String = computer_after_statement_vec
        .iter()
        .map(|val| {
            let val = val + 97;
            let val = val as char;
            val
        }).collect();
    result
}

fn solution(statement_str: &str) -> String {
    let mut result = String::new();

    result
}

fn main() -> io::Result<()>{
    
    print!("请输入需要加密的明文 : ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let buffer = buffer.trim();
    let buffer = buffer.replace(" ", "");
    println!("{}", buffer);

    let result = encode(&buffer);
    print!("加密后的明文是: {}\n", result);
    

    let result = decode(&result);
    
    println!("解密后的密码是: {}", result);
    
    Ok(())
}
