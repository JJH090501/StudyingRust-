struct Logger;

impl Logger {
    fn log(message: &str) {
        println!("{message}");
    }
}

fn main() {
    Logger::log("Logger is logging!")
}