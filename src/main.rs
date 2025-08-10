use instruction_decoder::Decoder;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut decoder = Decoder::new(&vec![include_str!("RV32I.toml").to_string()]).unwrap();


    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        line = line.trim_end().to_string();
        if (line.len()) > 0 {
            // println!("Read line: {}", line);

            let instr : u32 = u32::from_str_radix(&line, 16)?;
            // println!("Decoded: {}", instr);

            if let Ok(res) = decoder.decode_from_u32(instr, 32) {
                print!("{:?}", res);
            } else {
                // eprintln!("Error: {}", instr);
            }
        }
        println!("");
    }
    return Ok(());
}
