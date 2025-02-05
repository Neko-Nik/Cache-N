#[cfg(test)]
mod tests {
    use cache_n::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 5);
    }
}
