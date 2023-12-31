mod product;
mod customer;

use crate::product::*;
use product::*;

fn main() {
    let manager= product::Product::new_product_iofo("dongsub".to_string()
        , "inje".to_string());
    let time= product::Product::new_date(24, 12, 2023);
    let poki= product::Product::new("10", "1500", manager, time);

    poki.accounting(5);
}
