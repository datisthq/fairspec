//! Runs without the test harness, which would capture a stray `println!` before it
//! reached fd 1, and alone, so nothing else writes to fd 1 meanwhile.

#[cfg(unix)]
fn main() {
    use std::io::Read;
    use std::os::fd::FromRawFd;

    let path = std::env::temp_dir().join(format!("fairspec-mcp-stdout-{}.csv", std::process::id()));
    std::fs::write(&path, "id,name\n1,alice\n2,bob\n").unwrap();
    let arguments = serde_json::json!({"resource": {"data": path.to_str().unwrap()}});

    let mut fds = [0; 2];
    let result = unsafe {
        assert_eq!(libc::pipe(fds.as_mut_ptr()), 0);
        let saved = libc::dup(1);
        libc::dup2(fds[1], 1);
        let result = fairspec_mcp_server::call_tool("inferTableSchemaTool", Some(arguments));
        libc::dup2(saved, 1);
        libc::close(fds[1]);
        libc::close(saved);
        result
    };
    std::fs::remove_file(&path).unwrap();

    let mut output = String::new();
    unsafe { std::fs::File::from_raw_fd(fds[0]) }
        .read_to_string(&mut output)
        .unwrap();
    assert_eq!(result.is_error, Some(false));
    assert_eq!(output, "", "a tool call wrote to stdout");
    println!("test test_should_write_nothing_to_stdout ... ok");
}

#[cfg(not(unix))]
fn main() {}
