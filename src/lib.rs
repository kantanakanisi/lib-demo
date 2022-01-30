mod generator;

pub fn print_random_number() {
    let n = generator::gen_run();
    print!("{}", n);
}
