# valistr

> Creating immutable string wrapper types with values validated with regexes.

## TL;DR

```rust
use valistr::valistr;

/// A valid identifier in PascalCase. The regex used here is not perfect, but it's good enough for demonstration.
#[valistr(r"([A-Z][a-z0-9]*)+")]
struct PascalCaseId;

#[valistr(r"(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})")]
struct Date;

fn main() {
    // Create it with `new` method.
    let id = PascalCaseId::new("HelloWorld").unwrap();

    // `Debug` and `Display` are implemented.
    assert_eq!(format!("{}", id), "HelloWorld");
    assert_eq!(format!("{:?}", id), "\"HelloWorld\"");

    // `Deref<Target = String>` is implemented, so methods of `String` can be called directly.
    assert_eq!(id.as_str(), "HelloWorld");

    // `new` returns `None` if the input is not valid.
    assert!(PascalCaseId::new("helloWorld").is_none());

    // `TryFrom<&str>` and `TryFrom<String>` is also implemented.
    assert!(PascalCaseId::try_from("HelloWorld").is_ok());
    assert!(PascalCaseId::try_from("hello_world".to_string()).is_err());

    // For each named capture group `x`, a method `fn get_x(&self) -> Option<&str>` is provided.
    //
    // Note that the method will be generated only if the name matches `[a-z][a-z0-9_]*`. This 
    // constraint guarantees the generated method name is a valid and clean Rust identifier.
    let date = Date::new("2023-08-18").unwrap();
    assert_eq!(date.get_year().unwrap(), "2023");
    assert_eq!(date.get_month().unwrap(), "08");
    assert_eq!(date.get_day().unwrap(), "18");
}
```
