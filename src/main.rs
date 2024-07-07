pub use valistr_proc_macro::valistr;

// #[valistr("\\w+([-+.']\\w+)*@\\w+([-.]\\w+)*\\.\\w+([-.]\\w+)*")]
// pub struct EMailAddress(String);

#[valistr("(?<region>0\\d{2,3})-(?<number>\\d{7,8})")]
pub struct PhoneNumber;

fn main() {
    println!("Hello, world!");

    let a = PhoneNumber::try_from("028-23478953").unwrap();
    let b = PhoneNumber::try_from("09999-1237892").unwrap();

    let region = a.get_region().unwrap();
    let number = a.get_number().unwrap();

    println!("{}", *a);
    println!("Region: {}", region);
    println!("Number: {}", number);
}
