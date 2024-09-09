use transpiler::Transpiler;

mod lexer;
mod parser;
mod transpiler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("ts2 <test>");
        return;
    }
    let (test_name, output_name) = match &*args[1] {
        "log" => ("consolelog.idk", "consolelog.js"),
        _ => ("consolelog.idk", "consolelog.js"),
    };
    let file_content = std::fs::read_to_string(format!("./tests/{}", test_name)).unwrap();
    match Transpiler::from_source(file_content) {
        Ok(t) => {
            let _ = std::fs::write(format!("./tests/output/{}", output_name), t.transpile());
        }
        Err(e) => println!("Deu bom não ó {e:?}"),
    };
}
