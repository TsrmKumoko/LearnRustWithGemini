mod basics;
mod advanced;

fn main() {
    println!("--- Rust Basics ---");
    basics::variables::variables_example();
    basics::functions::functions_example();
    basics::control_flow::control_flow_example();
    basics::ownership::ownership_example();
    basics::structs::structs_example();
    basics::enums::enums_example();

    println!("\n--- Rust Advanced ---");
    advanced::lifetimes::lifetimes_example();
    advanced::traits::traits_example();
    advanced::generics::generics_example();
    advanced::closures::closures_example();
    advanced::threads::threads_example();
    advanced::macros::macros_example();
}
