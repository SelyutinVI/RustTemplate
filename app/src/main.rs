fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod resolve_modules {
    #[test]
    fn resolve_api_module() {
        let result = api::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn resolve_domain_module() {
        let result = domain::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn resolve_storage_module() {
        let result = storage::add(2, 2);
        assert_eq!(result, 4);
    }
}
