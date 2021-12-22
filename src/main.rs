use std::thread;
use std::time::Duration;

use subprocess::Exec;
use show_image::{ImageView, ImageInfo, create_window};

static mut OLD_LEN: i32 = 0;
static BYTES:&[u8; 72178] = include_bytes!("../img/cervo1.jpg");

fn main() {
    println!("{:?}", BYTES);
    let wait_time = Duration::from_secs(2);
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
                            let image = ImageView::new(ImageInfo::rgb8(1000, 749), BYTES);
                            match create_window("image", Default::default()) {
                                Ok(window) => {
                                    match window.set_image("image-001", image) {
                                        Ok(_) => (),
                                        Err(_) => ()
                                    }
                                },
                                Err(_) => ()
                            }
                            match Exec::shell("eog ./img/cervo1.jpg").capture() {
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
