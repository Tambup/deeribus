
use subprocess::Exec;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    //let dir_checksum = {
    //Exec::shell("tasklist")}.capture().unwrap().stdout_str();
    //println!("{}", dir_checksum);
    
    let res = {
        Exec::shell("tasklist") | Exec::cmd("findstr").arg("opera")
    }.capture().unwrap().stdout_str();
    println!("{}", res);
    
    Ok(())
}
