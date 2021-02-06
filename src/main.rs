mod population;
mod resources;

fn main() {
    let mut paris = population::Settlement::new();
    println!("{:#?}", paris);
    paris.add_community();
    println!("{:#?}", paris);
}
