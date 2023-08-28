use std::collections::HashMap;

use assert_cmd::Command;
use nix;

#[derive(Debug, PartialEq, Eq)]
struct Ns<'a> {
    proc_name: &'a str,
    name: &'a str,
    short_flag: char,
}

const MNT: Ns<'static> = Ns {
    proc_name: "mnt",
    name: "mount",
    short_flag: 'm',
};
const UTS: Ns<'static> = Ns {
    proc_name: "uts",
    name: "uts",
    short_flag: 'u',
};
const IPC: Ns<'static> = Ns {
    proc_name: "ipc",
    name: "ipc",
    short_flag: 'i',
};
const NET: Ns<'static> = Ns {
    proc_name: "net",
    name: "net",
    short_flag: 'n',
};
const PID: Ns<'static> = Ns {
    proc_name: "pid",
    name: "pid",
    short_flag: 'p',
};
const USER: Ns<'static> = Ns {
    proc_name: "user",
    name: "user",
    short_flag: 'U',
};
const CGROUP: Ns<'static> = Ns {
    proc_name: "cgroup",
    name: "cgroup",
    short_flag: 'C',
};
const TIME: Ns<'static> = Ns {
    proc_name: "time",
    name: "time",
    short_flag: 'T',
};

const NSES: [Ns<'static>; 8] = [MNT, UTS, IPC, NET, PID, USER, CGROUP, TIME];

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
        }
        (Err(le), _) => {
            panic!("unsharex error: {}", le);
        }
        (_, Err(re)) => {
            panic!("unshare error, but unsharex did not: {}", re);
        }
        (Ok(l), Ok(r)) => (l, r),
    };
    // rewrite unsharex to unshare in stdout/stderr and such before asserting
    for out in vec![&mut lhs, &mut rhs] {
        let lhs_str = String::from_utf8(out.stdout.clone()).unwrap();
        out.stdout = lhs_str
            .replace(
                assert_cmd::cargo::cargo_bin("unsharex").to_str().unwrap(),
                "unshare",
            )
            .replace("unsharex", "unshare")
            .into_bytes();
        let lhs_str = String::from_utf8(out.stderr.clone()).unwrap();
        out.stderr = lhs_str
            .replace(
                assert_cmd::cargo::cargo_bin("unsharex").to_str().unwrap(),
                "unshare",
            )
            .replace("unsharex", "unshare")
            .into_bytes();
    }

    assert_eq!(
        lhs.status, rhs.status,
        "status: {} != {}",
        lhs.status, rhs.status
    );
    assert_eq!(
        lhs.stdout,
        rhs.stdout,
        "stdout\n{:?}\n{:?}",
        String::from_utf8(lhs.stdout.clone()),
        String::from_utf8(rhs.stdout.clone())
    );
    assert_eq!(
        lhs.stderr,
        rhs.stderr,
        "stderr\n{:?}\n{:?}",
        String::from_utf8(lhs.stderr.clone()),
        String::from_utf8(rhs.stderr.clone())
    );

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
    })
    .stdout;
    assert_eq!(
        std::env::var("SHELL").unwrap(),
        String::from_utf8(shell).unwrap().trim()
    );

    let shell = compare_impls(|_, c| {
        c.env_remove("SHELL").args(vec!["printenv", "SHELL"]);
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
        })
        .stdout,
        r#"1: lo: <LOOPBACK> mtu 65536 qdisc noop state DOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
"#
        .as_bytes(),
    );
}

#[test]
// This test creates each easy-to-create namespace (i.e. not mount or user), and verifies that
// /proc/self/ns/{ns}, but other namespaces don't.
fn test_make_each_simple_ns() {
    if users::get_effective_uid() != 0 {
        eprintln!("skipping test that requires root");
        return;
    };

    println!(
        "debug: {:?}",
        std::fs::read_dir("/proc/self/ns")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>()
    );

    // first, create some namespace files and keep em open with an unshare
    let tmp_dir = tempfile::tempdir().unwrap();

    for ns in NSES {
        std::fs::File::create(tmp_dir.path().join(ns.name)).unwrap();
    }

    let mut root_nses = HashMap::new();
    for ns in NSES {
        root_nses.insert(
            ns.name,
            std::fs::read_link(format!("/proc/self/ns/{}", ns.proc_name))
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }

    let parse_nses = |readlink_output: &str| {
        let mut out = HashMap::new();
        let lines = readlink_output.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), NSES.len());
        for (i, ns) in NSES.iter().enumerate() {
            out.insert(ns.name, lines[i].to_string());
        }
        out
    };

    let readlink_cmd = format!(
        "readlink /proc/self/ns/{{{}}}",
        NSES.iter()
            .map(|n| n.proc_name)
            .collect::<Vec<_>>()
            .join(",")
    );

    for unsharex in vec![true, false] {
        for ns in NSES {
            match ns {
                ns if ns == MNT || ns == USER || ns == TIME => {
                    // finicky namespaces, handle em separately in other tests
                    continue;
                }
                _ => {}
            }
            let nsf = tmp_dir.path().join(ns.name).to_str().unwrap().to_string();

            let mut cmd = if unsharex {
                Command::cargo_bin("unsharex").unwrap()
            } else {
                Command::new("unshare")
            };

            cmd.args(vec![&format!("--{}={}", ns.name, nsf)]);
            if ns == PID {
                // special case, need to fork or pid namespaces don't work
                cmd.arg("--fork");
            }
            cmd.args(vec!["--", "bash", "-c", &readlink_cmd]);
            let out = cmd.unwrap();
            let outns = parse_nses(&String::from_utf8(out.stdout).unwrap());

            for ns2 in NSES {
                if ns == ns2 {
                    assert_ne!(
                        outns.get(ns2.name),
                        root_nses.get(ns2.name),
                        "unsharex={}, ns={}",
                        unsharex,
                        ns.name
                    );
                } else {
                    assert_eq!(
                        outns.get(ns2.name),
                        root_nses.get(ns2.name),
                        "unsharex={}, ns={}",
                        unsharex,
                        ns.name
                    );
                }
            }

            // and cleanup
            let _ = nix::mount::umount(nsf.as_str());
        }
    }
}

#[test]
fn test_make_mount_ns() {
    if users::get_effective_uid() != 0 {
        eprintln!("skipping test that requires root");
        return;
    };

    let tmp_dir = tempfile::tempdir().unwrap();
    let path = tmp_dir.path();

    std::fs::create_dir(path.join("proc")).unwrap();

    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut cmd = std::process::Command::new("cargo");
    cmd.current_dir(format!("{}/testbins/printnamespaces", &manifest_path));
    cmd.args(vec!["build", "--target", "x86_64-unknown-linux-musl"]);
    println!("Running {:?}", cmd);
    cmd.status().unwrap();

    let test_bin_path = format!(
        "{}/testbins/printnamespaces/target/x86_64-unknown-linux-musl/debug/printnamespaces",
        &manifest_path
    );
    let test_bin_path = std::path::Path::new(&test_bin_path);

    let out = std::process::Command::new(&test_bin_path).output().unwrap();
    let outstr = String::from_utf8(out.stdout).unwrap();
    let host_mount_ns = outstr.lines().find(|line| line.starts_with("mnt")).unwrap();

    std::fs::copy(test_bin_path, path.join("printnamespaces")).unwrap();
    for unsharex in vec![false, true] {
        let mut cmd = if unsharex {
            Command::cargo_bin("unsharex").unwrap()
        } else {
            Command::new("unshare")
        };

        cmd.args(vec![
            "--mount-proc",
            "--mount",
            &format!("--root={}", path.to_string_lossy()),
            "--",
            "/printnamespaces",
        ]);
        let out = cmd.unwrap();
        let outstr = String::from_utf8(out.stdout).unwrap();
        let outns = outstr.lines().find(|line| line.starts_with("mnt")).unwrap();

        assert_ne!(host_mount_ns, outns);
    }
}
