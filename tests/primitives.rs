#[cfg(test)]
mod test {
    use rust_apl::interpreter::Interpreter;
    use rust_apl::run::run;

    fn apl_assert(left: &str, right: &str) {
        let l = format!("{}\n", left.to_string());
        let r = format!("{}\n", right.to_string());

        assert_eq!(
            run(l, &mut Interpreter::new(), false).unwrap(),
            run(r, &mut Interpreter::new(), false).unwrap()
        );
    }

    #[test]
    fn add() {
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
    }
}
