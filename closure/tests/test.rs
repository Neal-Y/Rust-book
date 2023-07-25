#[cfg(test)]
mod tests {

    #[test]
    fn test_expensive_calculation() {
        let mut c = closure::Cacher::new(|a| a);
        let b = c.value(9);
        let a = c.value(10);

        assert_eq!(9, b);
        assert_eq!(9, a);
    }

    #[test]
    fn test_next_method() {
        let mut c = closure::Counter::constructor();
        assert_eq!(c.next(), Some(1));
        assert_eq!(c.next(), Some(2));
        assert_eq!(c.next(), Some(3));
        assert_eq!(c.next(), Some(4));
        assert_eq!(c.next(), Some(5));
        assert_eq!(c.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = closure::Counter::constructor()
            .zip(closure::Counter::constructor().skip(1))
            .map(|(iter1, iter2)| iter1 * iter2)
            .sum();
        assert_eq!(40, sum);
    }
}
