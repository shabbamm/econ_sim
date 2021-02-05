mod population;

fn main() {
    let mut paris = population::Settlement::new();
    println!("{:#?}", paris);
    paris.add_community();
    println!("{:#?}", paris);
    paris.society[0].add_size(23);
    println!("{:#?}", paris);
}
