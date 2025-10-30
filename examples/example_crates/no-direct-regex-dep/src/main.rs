use valistr_renamed::valistr;

#[valistr(r"^(?<major>0|[1-9]\d*)\.(?<minor>0|[1-9]\d*)\.(?<patch>0|[1-9]\d*)$")]
struct SemVer3;

fn main() {
    let ver = SemVer3::try_from("1.2.3").unwrap();
    
    println!("Version: {}", ver);
    println!("Major: {}", ver.get_major().unwrap());
    println!("Minor: {}", ver.get_minor().unwrap());
    println!("Patch: {}", ver.get_patch().unwrap());
}
