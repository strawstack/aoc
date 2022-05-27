#[allow(dead_code)]
pub fn main(input: &str) -> i32 {

    let mut nonce: i32 = 1;
        
    loop {
        let digest  = md5::compute(format!("{}{}", input.trim(), nonce.to_string()));
        let ans: String = format!("{:?}", digest);
        
        let slice: &str = &ans[0..6];         

        if slice == "000000" {
            println!("{}", nonce);
            return nonce; 
        }

        nonce += 1;
    }
    
}
