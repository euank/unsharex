use std::os::unix::process::CommandExt;

pub fn exec_shell() -> ! {
    let shell_str = std::env::var("SHELL").unwrap_or("/bin/sh".to_string());
    let shell = std::path::Path::new(&shell_str);
    let shell_basename = shell.file_name().unwrap_or_default();

    // see upstream exec_shell; the arg0 gets a '-' prefixed on
    let mut arg0 = std::ffi::OsString::with_capacity(shell_basename.len() + 2);
    arg0.push("-");
    arg0.push(shell_basename);

    std::process::Command::new(shell)
        .arg0(arg0)
        .exec();
    panic!("failed to execute {}", shell_str);
}
