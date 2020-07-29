use log::info;

#[derive(Default)]
pub struct Adder {
    count: i64,
}

impl Adder {
    pub fn add(&mut self, value: i64) {
        info!("Adder::add()");
        self.count += value;
    }

    pub fn tell(&self) -> i64 {
        info!("Adder::tell()");
        self.count
    }
}
