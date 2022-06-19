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
        apl_assert("1+1", "2");
        apl_assert("5+1 2 3", "6 7 8");
        apl_assert("1 2 3+4 5 6", "5 7 9");
        apl_assert("(1 (2 3) (4 5))+(1 2 (3 4))", "2 (4 5) (7 9)");
    }

  
}
