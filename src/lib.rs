use uuid::Uuid
;

pub mod core;
pub mod infrastructure;

pub mod utils {
    use uuid::Uuid;

    pub fn generate_uuid() -> Uuid {
        Uuid::new_v4()
    }

    pub fn cmd_prompt(x: &str) -> String {
        println!("{}", x);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn get_current_unix_timestamp() -> i64 {
        let now = std::time::SystemTime::now();
        let unix_timestamp = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        unix_timestamp.as_secs() as i64
    }
}
