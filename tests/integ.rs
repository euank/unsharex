use std::collections::HashMap;
use std::process::Command as StdCommand;

use nix;
use assert_cmd::Command;

// compare impls compares 'unsharex' vs 'unshare'
fn compare_impls<F: Fn(bool, &mut Command)>(f: F) -> std::process::Output {
    let mut lhs = Command::cargo_bin("unsharex").unwrap();
    let mut rhs = Command::new("unshare");

    f(true, &mut lhs);
    f(false, &mut rhs);

    let lhs_out = lhs.output();
    let rhs_out = rhs.output();

    let (mut lhs, mut rhs) = match (lhs_out, rhs_out) {
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
            (l, r)
        },
    };
    // rewrite unsharex to unshare in stdout/stderr and such before asserting
    for out in vec![&mut lhs, &mut rhs] {
        let lhs_str = String::from_utf8(out.stdout.clone()).unwrap();
        out.stdout = lhs_str
            .replace(assert_cmd::cargo::cargo_bin("unsharex").to_str().unwrap(), "unshare")
            .replace("unsharex", "unshare").into_bytes();
        let lhs_str = String::from_utf8(out.stderr.clone()).unwrap();
        out.stderr = lhs_str
            .replace(assert_cmd::cargo::cargo_bin("unsharex").to_str().unwrap(), "unshare")
            .replace("unsharex", "unshare").into_bytes();
    }

    assert_eq!(lhs.status, rhs.status, "status: {} != {}", lhs.status, rhs.status);
    assert_eq!(lhs.stdout, rhs.stdout, "stdout\n{:?}\n{:?}", String::from_utf8(lhs.stdout.clone()), String::from_utf8(rhs.stdout.clone()));
    assert_eq!(lhs.stderr, rhs.stderr, "stderr\n{:?}\n{:?}", String::from_utf8(lhs.stderr.clone()), String::from_utf8(rhs.stderr.clone()));

    lhs
}

#[test]
fn test_simple() {
    let cmds = vec![
        vec!["--", "/bin/sh", "-c", "echo 1"],
        vec!["/bin/sh", "-c", "echo 1"],
        vec!["--", "--", "/bin/sh", "-c", "echo 1"],
        vec!["--", "--", "/some/path/that/does/not/exist"],
        vec!["/bin/true"],
        vec!["/bin/false"],
        vec!["--keep-caps", "sh", "-c", "echo hi"],
        vec!["--", "printenv"],
    ];

    for args in cmds {
        println!("testing: {:?}", args);
        compare_impls(|_, c| {
            c.args(args.clone());
        });
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

#[test]
fn test_make_each_ns() {
    if users::get_effective_uid() != 0 {
        eprintln!("skipping test that requires root");
        return;
    };

    // Join each namespace in the two different forms you can
    // the '-t$PID' format, and the '/file' format

    // first, create some namespace files and keep em open with an unshare
    let tmp_dir = tempfile::tempdir().unwrap();
    let nses = vec![
        // (long flag, proc name, short flag)
        // long flag is also the filename of /proc/self/ns/$file
        ("mount", "mnt", "m"),
        ("uts", "uts", "u"),
        ("ipc", "ipc", "i"),
        ("net", "net", "n"),
        ("pid", "pid", "p"),
        ("user", "user", "U"),
        ("cgroup", "cgroup", "C"),
        ("time", "time", "T"),
    ];

    for ns in &nses {
        std::fs::File::create(tmp_dir.path().join(ns.0)).unwrap();
    }

    let mut root_nses = HashMap::new();
    for ns in &nses {
        root_nses.insert(ns.0, std::fs::read_link(format!("/proc/self/ns/{}", ns.1)).unwrap().to_str().unwrap().to_string());
    }

    let parse_nses = |readlink_output: &str| {
        let mut out = HashMap::new();
        let lines = readlink_output.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), nses.len());
        for (i, ns) in nses.iter().enumerate() {
            out.insert(ns.0, lines[i].to_string());
        }
        out
    };

    let readlink_cmd = format!("readlink /proc/self/ns/{{{}}}", nses.iter().map(|n| n.1).collect::<Vec<_>>().join(","));

    for unsharex in vec![true, false] {
        for ns in &nses {
            match (unsharex, ns.0) {
                (false, "mount") | (false, "user") => {
                    // XXX: for some reason, unshare/unsharex work differently these and I don't
                    // get why yet. Come back to this.
                    continue
                }
                _ => {},
            }
            let nsf = tmp_dir.path().join(ns.0).to_str().unwrap().to_string();

            let mut cmd = if unsharex {
                Command::cargo_bin("unsharex").unwrap()
            } else {
                Command::new("unshare")
            };

            cmd.args(vec![&format!("--{}={}", ns.0, nsf)]);
            if ns.0 == "pid" {
                // special case, need to fork or pid namespaces don't work
                cmd.arg("--fork");
            }
            cmd.args(vec!["--", "sh", "-c", &readlink_cmd]);
            let out = cmd.unwrap();
            let outns = parse_nses(&String::from_utf8(out.stdout).unwrap());

            for ns2 in &nses {
                if ns.0 == ns2.0 {
                    assert_ne!(outns.get(ns2.0), root_nses.get(ns2.0), "unsharex={}, ns={}", unsharex, ns.0);
                } else {
                    assert_eq!(outns.get(ns2.0), root_nses.get(ns2.0), "unsharex={}, ns={}", unsharex, ns.0);
                }
            }

            // and cleanup
            let _ = nix::mount::umount(nsf.as_str());
        }
    }
}
