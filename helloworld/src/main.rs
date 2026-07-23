fn test(message : String)
{
    println!("Hello, world! Message : {}", message);
}

fn main() {
    test("salut".to_string());
    test("aller l'OM".to_lowercase().to_string());
}
