fn main() {
    println!("Hello, world!");
    // let mode = "ctof";
    println!("{}",convert_temps("ftoc",40.0))
}

fn convert_temps(mode:&str,input_temp:f64)->f64 {
    if mode=="ftoc" {
        return (input_temp-32.0)*(5.0/9.0)
    }else if mode=="ctof" {
        return (input_temp*1.8)+32.0
    }else {
        println!("Please enter a valid mode!");
        return 1.0;
    }
}
