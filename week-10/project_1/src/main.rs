struct Brand{
    hp:i32,
    ibm:i32,
    toshiba:i32,
    dell:i32
}

impl Brand {
    fn total_price(&self)->i32{
        self.hp*3 + self.ibm*3 + self.toshiba*3 + self.dell*3
    }
}
fn main() {
    let unit_price = Brand{
        hp:650_000,
        ibm:755_000,
        toshiba:550_000,
        dell:850_000
    };
    print!("Hp price is {}\n IBM price is {}\n Toshiba price is {}\n Dell price is {}\n Total price is {}",unit_price.hp,unit_price.ibm,unit_price.toshiba,
            unit_price.dell,unit_price.total_price());
}
