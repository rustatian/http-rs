pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn it_works() {
            let mut apple_map = HashMap::from([
        ("Red Delicious", 2),
        ("Kawana Apple", 5),
    ]);
        let result = add(2, 2);
        let s: String = String::from("test");
        let p = Plugin {
            name: s,
            version: String::from("1.0"),
        };
        assert_eq!(result, 4);
    }
}

struct Plugin {
    name: String,
    version: String,
}

impl Plugin {
    #[unsafe(no_mangle)]
    pub extern "C" fn init() -> anyhow::Result<()> {
        Ok(())
    }
}