fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let name: String = cat.0.to_string();
    // let age: f64 = cat.1;

    let (name, age) = cat;
    println!("{name} is {age} years old");
}
