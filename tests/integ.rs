use assert_cmd::Command;

// compare impls compares 'unsharex' vs 'unshare'
fn compare_impls<F: Fn(bool, &mut Command)>(f: F) -> std::process::Output {
    let mut lhs = Command::cargo_bin("unsharex").unwrap();
    let mut rhs = Command::new("unshare");

    f(true, &mut lhs);
    f(false, &mut rhs);

    let lhs_out = lhs.output();
    let rhs_out = rhs.output();

    match (lhs_out, rhs_out) {
        (Err(le), Err(re)) => {
            panic!("error equality: {:?}, {:?}", le, re);
        },
        (Err(le), _) => {
            panic!("unsharex error: {}", le);
        }
        (_, Err(re)) => {
            panic!("unshare error, but unsharex did not: {}", re);
        },
        (Ok(l), Ok(r)) => {
            assert_eq!(l, r);
            l
        },
    }
}

#[test]
fn test_shell() {
    let shell = compare_impls(|_, c| {
        c.args(vec!["--", "printenv", "SHELL"]);
    }).stdout;
    assert_eq!(std::env::var("SHELL").unwrap(), String::from_utf8(shell).unwrap().trim());

    let shell = compare_impls(|_, c| {
        c
            .env_remove("SHELL")
            .args(vec!["printenv", "SHELL"]);
    });
    assert_eq!(
        "".to_string(),
        String::from_utf8(shell.stdout).unwrap().trim(),
    );
}

#[test]
fn test_netns() {
    if users::get_effective_uid() != 0 {
        eprintln!("skipping test that requires root");
        return;
    };
    assert_eq!(
        compare_impls(|_, c| {
            c.args(vec!["-n", "--", "ip", "a"]);
        }).stdout,
        r#"1: lo: <LOOPBACK> mtu 65536 qdisc noop state DOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
"#.as_bytes(),
    );
}
