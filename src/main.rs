use std::env;
use std::thread;
use std::path::Path;
use std::time::Duration;

use subprocess::Exec;
use reqwest;

static mut OLD_LEN: i32 = 0;

//CERVO: https://www.bresciaoggi.it/image/policy:1.3613028:1446548541/cervi.jpg?f=default\&w=1024\&\$p\$f\$w=3a7dd60

fn main(){
    let wait_time = Duration::from_secs(2);
    loop {
        thread::spawn(|| {
            let res = {
                Exec::shell("ps aux") | Exec::cmd("grep").arg("firefox")
            }.capture();

            match res {
                Ok(values) => {
                    let path: String = format!("{}{}", std::env::var("HOME").unwrap(), "/.cache/.crv.jpeg");
                    let new_len: i32 = values.stdout_str().split("\n").collect::<Vec<&str>>().len() as i32;
                    unsafe {
                        if OLD_LEN != new_len {
                            if !Path::new(&path).exists() {
                                let url: Vec<String> = env::args().collect();
                                let url: String = url[1].clone();
                                match reqwest::blocking::get(url.clone()) {
                                    Ok(temp) => {
                                        match temp.bytes() {
                                            Ok(temp) => {
                                                match image::load_from_memory(&temp) {
                                                    Ok(image) => {
                                                        match image.save(path.clone()) {
                                                            Ok(_) => (),
                                                            Err(e) => println!("{}", e)
                                                        }
                                                    },
                                                    Err(_) => (),
                                                }
                                            },
                                            Err(_) => (),
                                        }
                                    },
                                    Err(_) => (),
                                }
                            }
                            match Exec::shell(format!("eog {}", &path)).capture() {
                                Ok(_) => (),
                                Err(_) => ()
                            }
                        }
                    
                        OLD_LEN = new_len;
                    };

                },
                Err(_) => ()
            }
            
        });
        thread::sleep(wait_time);
    }
}
