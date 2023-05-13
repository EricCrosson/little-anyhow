#[derive(Debug)]
struct MyError(std::io::Error);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unable to charge flux capacitor")
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

fn main() -> Result<(), little_anyhow::Error> {
    let error = std::io::Error::new(std::io::ErrorKind::Other, "not enough gigawatts");
    let error = MyError(error);
    Err(error.into())
}
