mod fraction;

use fraction::Fraction;

fn main() {
    let frac = Fraction::new(1, 2);
    println!("DEBUG_PRINT: {:?}", frac);
    println!("STRING_METHOD: {}", frac.to_string());

    let mul_frac = Fraction::new(2, 3);
    let res_frac = frac.multiply(&mul_frac);

    println!("MUL_RESULT: {}", res_frac.to_string());

    let div_frac: Fraction<i32> = res_frac.divide(&mul_frac);
    println!("DIV_RESULT: {}", div_frac.to_string());
}
