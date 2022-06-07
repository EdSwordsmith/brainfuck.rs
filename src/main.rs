mod parser;

fn main() {
    let input = "+-[><].,";
    let ast = parser::parse(input);
    println!("{:?}", ast);
}
