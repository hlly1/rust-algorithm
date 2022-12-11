pub mod majority_element;

include!("single_number.rs");
include!("majority_element.rs");
include!("search_matrix.rs");

fn main() {
    //只出现了一次的数字
    //single_number();
    // majority_element();

    match search_matrix(vec![vec![1,1]], 0) {
        true => println!("true!"),
        false => println!("false!"),
    }

}