mod util;
mod win;
extern crate time;

fn main() -> Result<(), String> {
    for _ in 1..1000 {
        let time_point = time::get_time();
        println!("Time = {:?}", time_point);
        println!("Result={}", win::get_active_title());
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(())
}
