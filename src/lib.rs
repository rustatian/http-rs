pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
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