#[cfg(test)]
mod test {
    use rust_apl::interpreter::Interpreter;
    use rust_apl::run::run;

    fn apl_assert(left: &str, right: &str) {
        let l = format!("{}\n", left);
        let r = format!("{}\n", right);

        assert_eq!(
            run(l, &mut Interpreter::new(), false).unwrap(),
            run(r, &mut Interpreter::new(), false).unwrap()
        );
    }

    fn assert_err(expr: &str) {
        let e = format!("{}\n", expr);
        let value = run(e, &mut Interpreter::new(), false);
        assert!(value.is_err())
    }

    fn assert_sequence(left: Vec<&str>, right: &str) {
        let mut interpreter = Interpreter::new();
        let mut results = Vec::new();

        for l in left {
            let s = format!("{}\n", l);
            results.push(run(s, &mut interpreter, false).unwrap())
        }

        let r = format!("{}\n", right);
        let r_val = run(r, &mut Interpreter::new(), false).unwrap();

        assert_eq!(*results.last().unwrap(), r_val);
    }

    #[test]
    fn plus() {
        // nine combos of scalar, vector (soon array!), enclose
        apl_assert("1+1", "2");
        apl_assert("1+1 2 3", "2 3 4");
        apl_assert("1+(1 2)(3 4)", "(2 3)(4 5)");

        apl_assert("1 2 3+1", "2 3 4");
        apl_assert("1 2 3+4 5 6", "5 7 9");
        apl_assert("(1 2 3)+(1 2)(3 4)(5 6)", "(2 3)(5 6)(8 9)");

        apl_assert("(1 2)(3 4)+1", "(2 3)(4 5)");
        apl_assert("(1 2)(3 4)(5 6)+1 2 3", "(2 3)(5 6)(8 9)");
        apl_assert("(1 2)(3 4)+(5 6)(7 8)", "(6 8)(10 12)");

        // should fail for incompatible sizes
        assert_err("1 2+1 2 3");
    }

    #[test]
    fn minus() {
        // nine combos of scalar, vector (soon array!), enclose
        apl_assert("1-2", "¯1");
        apl_assert("1-1 2 3", "0 ¯1 ¯2");
        apl_assert("1-(1 2)(3 4)", "(0 ¯1)(¯2 ¯3)");

        apl_assert("1 2 3-1", "0 1 2");
        apl_assert("1 2 3-4 5 6", "¯3 ¯3 ¯3");
        apl_assert("(1 2 3)-(1 2)(3 4)(5 6)", "(0 ¯1)(¯1 ¯2)(¯2 ¯3)");

        apl_assert("(1 2)(3 4)-1", "(0 1)(2 3)");
        apl_assert("(1 2)(3 4)(5 6)-1 2 3", "(0 1)(1 2)(2 3)");
        apl_assert("(1 2)(3 4)-(5 6)(7 8)", "(¯4 ¯4)(¯4 ¯4)");

        // should fail for incompatible sizes
        assert_err("1 2-1 2 3");
    }

    #[test]
    fn enclose() {
        // a bit hard to test, because no parsing way to enclose a single array...

        // no change for these scalars
        apl_assert("⊂1", "1");
        apl_assert("⊂'t'", "'t'");
    }

    // check enclose broadcating
    #[test]
    fn enclose_broadcast() {
        apl_assert("1 2 3+⊂4 5 6", "(5 6 7)(6 7 8)(7 8 9)");
    }

    #[test]
    fn assignment() {
        assert_sequence(vec!["a←1", "a"], "1");
        assert_sequence(vec!["a←1", "a+1"], "1+1");
    }
}
