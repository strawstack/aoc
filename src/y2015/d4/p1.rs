#[allow(dead_code)]
pub fn main(input: &str) {

    let mut nonce: i32 = 1;
        
    loop {
        let digest  = md5::compute(format!("{}{}", input.trim(), nonce.to_string()));
        let ans: String = format!("{:?}", digest);
        
        let slice: &str = &ans[0..5];         

        if slice == "00000" {
            println!("{}", nonce);
            break;
        }

        nonce += 1;
    }
    
}
