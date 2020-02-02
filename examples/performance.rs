use std::fmt;
use std::time::Instant;

struct Struct {
    data: [u8; 32],
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

fn main() {
    // --------------------------------
    println!("loge");
    ::std::env::set_var("RUST_LOG", "trace");
    ::std::env::set_var("LOGE_FORMAT", "target");
    loge::init();
    // --------------------------------
    // $> Set-Item -Path Env:RUST_LOG -Value "trace"
    // println!("env_logger");
    // env_logger::init();
    // $> Set-Item -Path Env:RUST_LOG
    // --------------------------------
    let mut structs = Vec::new();
    for i in 0..100 {
        structs.push(Struct { data: [i as u8; 32] });
    }

    {
        // With format
        let start = Instant::now();
        for s in &structs {
            log::info!("{}", format!("{}", s));
        }
        eprintln!("with format: {:?}", start.elapsed());
    }

    {
        // Plain logger
        let start = Instant::now();
        for s in &structs {
            log::info!("{}", s);
        }
        eprintln!("plain: {:?}", start.elapsed());
    }
}
