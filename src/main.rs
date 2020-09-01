use std::time::{Duration};
use std::thread::sleep;

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..20 {
        sleep(Duration::new(1, 0));
        pb.println(format!("[+] finished #{}", i));
        pb.inc(5);
    }
    pb.finish_with_message("done");
}