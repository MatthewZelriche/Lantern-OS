use kernel::kprintln;

pub fn test_runner(tests: &[&dyn Fn()]) {
    kprintln!("TESTS BEGIN: RUNNING {} TESTS", tests.len());

    for test in tests {
        test();
    }

    kprintln!("TESTS END\n");
}
