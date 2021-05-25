fn main() {
    let function_name = match std::env::args().nth(1) {
        Some(function_name) if function_name != "--help" => function_name,
        _ => print_help(),
    };

    let input = {
        use std::io::Read;
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("failed to read stdin");
        input
    };

    let func = extract(&input, &function_name).expect("failed to find function");

    println!("{}", func);
}

fn print_help() -> ! {
    let text = "ir-extract
        usage: ir-extract <function-name>

        read  llvm-ir from stdin.
        print only the function <function-name>. Ignore surroundings";

    println!("{}", text);
    std::process::exit(0);
}

fn extract<'a>(input: &'a str, name: &str) -> Option<&'a str> {
    let fn_name = format!("@{}", name);
    let res = input.find(&fn_name)?;
    let start = ( &input[..res] ).rfind("define")?;
    let input = &input[start..];
    let end = input.find("}")?;

    Some(&input[..end + 1])
}
