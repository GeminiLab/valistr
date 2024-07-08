use valistr::valistr;

#[valistr("\\w+([-+.']\\w+)*@\\w+([-.]\\w+)*\\.\\w+([-.]\\w+)*")]
pub struct EMailAddress();

#[valistr("(?<region>0\\d{2,3})-(?<number>\\d{7,8})")]
pub struct PhoneNumber;

fn main() {
    println!("Hello, world!");

    let a = PhoneNumber::try_from("0811-81675528").unwrap();
    PhoneNumber::try_from("9999-123456789").unwrap_err();

    let email = EMailAddress::try_from("aarkegz@gmail.com").unwrap();
    EMailAddress::try_from("aarkegz@gmail").unwrap_err();

    let region = a.get_region().unwrap();
    let number = a.get_number().unwrap();

    println!("{}", *a);
    println!("Region: {}", region);
    println!("Number: {}", number);
    println!("{:?}", email);
}
