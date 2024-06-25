#[ic_cdk::query]
fn greet(name: String, surname: String) -> String {
    format!("Hello, {} {}!", name, surname)
}
