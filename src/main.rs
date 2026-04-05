
use progranome::app;

#[cfg(target_os = "linux")]
pub fn main() -> Result<(),  Box<dyn std::error::Error>> {
    app::start_app()
}