mod product;
// product
fn main() {
    let managers= product::Product_Info::new("dongsub", "Inje");
    let time= product::Date::new(24, 12, 2023);
    let poki= product::Product::new("10", "1500", managers, time);

    poki.accounting(5);
}
