use dotenv::dotenv;

pub fn init() {
    dotenv().ok();
}
