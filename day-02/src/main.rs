mod libs;

use crate::libs::is_valid_id;

use std::fs;

fn main() {
    let product_ids = fs::read_to_string("src/input.txt")
        .expect("input txt file should be in the same directory as main src file");

    let product_ids = product_ids.trim().split(",");

    let mut result: i128 = 0;

    // let mut invalid_ids: Vec<String> = Vec::new();

    for product in product_ids {
        let mut product = product.trim().split("-");
        let first_part: i128 = product.next().unwrap().parse().unwrap();
        let last_part: i128 = product.next().unwrap().parse().unwrap();

        let range = std::ops::Range {
            start: first_part,
            end: last_part + 1,
        };

        for id in range {
            let id_str = id.to_string();
            if !is_valid_id(&id_str) {
                result += id;
                // invalid_ids.push(id_str);
            }
        }
    }

    println!("Result is: {}", result);
    // println!("Invalid IDs are: {:?}", invalid_ids);
}
