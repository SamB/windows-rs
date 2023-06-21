use std::io::Write;

pub fn try_format(gen: &super::Gen, tokens: &str) -> String {
    let version = std::env!("CARGO_PKG_VERSION");

    // Packaging - e.g. windows/windows-sys crates - assumes the crate will allow whatever warnings it deems fit.
    let allow = if gen.package {
        ""
    } else {
        "#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]\n"
    };

    let tokens = format!(
        r#"// Bindings generated by `riddle` {version}

{allow}{tokens}"#
    );

    let Ok(mut child) = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn() else {
        return tokens;
    };

    let Some(mut stdin) = child.stdin.take() else {
        return tokens;
    };

    if stdin.write_all(tokens.as_bytes()).is_err() {
        return tokens;
    }

    drop(stdin);

    let Ok(output) = child.wait_with_output() else {
        return tokens;
    };

    if !output.status.success() {
        return tokens;
    }

    if let Ok(result) = String::from_utf8(output.stdout) {
        result
    } else {
        tokens
    }
}
