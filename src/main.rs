use std::thread;
use std::time::Duration;

use subprocess::Exec;

static mut OLD_LEN: i32 = 0;

fn main() {
    let wait_time = Duration::from_millis(2500);
    loop {
        thread::spawn(|| {
            let res = {
                Exec::shell("ps aux") | Exec::cmd("grep").arg("firefox")
            }.capture();

            match res {
                Ok(values) => {
                    let str_res = values.stdout_str();
                    let new_len = str_res.split("\n").collect::<Vec<&str>>().len() as i32;
                    unsafe {
                        if OLD_LEN != new_len {
                            println!("{}", new_len);
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
