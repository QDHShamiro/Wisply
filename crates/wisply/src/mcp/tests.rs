use super::*;
use std::cell::RefCell;

fn run(line: &str) -> (Value, Vec<String>, Option<String>) {
    let seen = RefCell::new((Vec::new(), None));
    let call = |argv: &[String], secret: Option<&str>| {
        *seen.borrow_mut() = (argv.to_vec(), secret.map(str::to_string));
        reply_text("ok", false)
    };
    let res = run_cmd(line, &call);
    let (argv, secret) = seen.into_inner();
    (res, argv, secret)
}

fn said(v: &Value) -> (&str, bool) {
    (v["content"][0]["text"].as_str().unwrap_or(""), v["isError"].as_bool().unwrap_or(false))
}

#[test]
fn a_missing_daemon_is_started_once_and_only_an_old_home_goes_to_the_old_client() {
    let (short, long) = (Duration::from_millis(60), Duration::from_secs(5));
    let (mut tries, mut starts) = (0, 0);
    let got = reach(
        || {
            tries += 1;
            if tries < 4 { Err(Conn::Missing) } else { Ok(b"0\0\0ok".to_vec()) }
        },
        true,
        || starts += 1,
        short,
        long,
    );
    assert!(matches!(got, Got::Bytes(ref b) if b == b"0\0\0ok"));
    assert_eq!((tries, starts), (4, 1));
    let mut starts = 0;
    assert!(matches!(reach(|| Err(Conn::Missing), false, || starts += 1, short, short), Got::Old));
    assert_eq!(starts, 0);
    let at = Instant::now();
    assert!(matches!(reach(|| Err(Conn::Missing), true, || starts += 1, long, short), Got::Down));
    assert!(starts == 1 && at.elapsed() >= short && at.elapsed() < long);
    assert!(matches!(reach(|| Err(Conn::Busy), true, || starts += 1, short, long), Got::Busy));
    assert_eq!(starts, 1);
}

#[test]
fn commands_go_to_the_daemon_as_words() {
    let (res, argv, secret) = run("snap -f price");
    assert_eq!(argv, ["snap", "-f", "price"]);
    assert_eq!(secret, None);
    assert_eq!(said(&res), ("ok", false));
    assert_eq!(run("wisply click \"Sign in\"").1, ["click", "Sign in"]);
    assert_eq!(run("lighting click \"Sign in\"").1, ["click", "Sign in"]);
}

#[test]
fn help_goes_to_the_daemon_help_command() {
    assert_eq!(run("-h").1, ["help"]);
    assert_eq!(run("--help").1, ["help"]);
    assert_eq!(run("help snap").1, ["help", "snap"]);
    assert_eq!(run("snap --help").1, ["help", "snap"]);
    assert_eq!(run("snap -h").1, ["help", "snap"]);
    assert_eq!(run("snap -h x").1, ["snap", "-h", "x"]);
    assert_eq!(run("do a -- --help").1, ["do", "a", "--", "--help"]);
}

#[test]
fn secrets_come_from_the_environment() {
    let path = env::var("PATH").unwrap();
    let (_, argv, secret) = run("type e3 --env PATH");
    assert_eq!(argv, ["type", "e3"]);
    assert_eq!(secret.as_deref(), Some(path.as_str()));
    let (res, argv, _) = run("type e3 --env LT_TEST_SURELY_MISSING_9F2");
    assert!(argv.is_empty());
    assert_eq!(said(&res), ("err: environment variable LT_TEST_SURELY_MISSING_9F2 is not set", true));
    assert_eq!(run("type --env").1, ["type", "--env"]);
}

#[test]
fn blocked_empty_and_broken_lines_answer_locally() {
    let (res, argv, _) = run("daemon");
    assert!(argv.is_empty());
    assert_eq!(said(&res), ("err: daemon is not available through MCP", true));
    assert_eq!(said(&run("wisply mcp").0), ("err: mcp is not available through MCP", true));
    assert_eq!(said(&run("   ").0), ("err: empty command -> try: help", true));
    assert_eq!(said(&run("wisply").0), ("err: empty command -> try: help", true));
    assert_eq!(said(&run("click \"Sign in").0), ("err: No closing quotation", true));
}

#[test]
fn protocol_answers_like_the_2x_server() {
    let call = |_: &[String], _: Option<&str>| reply_text("ok", false);
    let one = |s: &str| answer(s, &call);
    let init = one(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26"}}"#);
    assert_eq!(init[0]["result"]["protocolVersion"], "2025-03-26");
    assert_eq!(init[0]["result"]["serverInfo"]["name"], "wisply");
    let init = one(r#"{"jsonrpc":"2.0","id":2,"method":"initialize","params":{"protocolVersion":"1999-01-01"}}"#);
    assert_eq!(init[0]["result"]["protocolVersion"], "2025-06-18");
    assert!(one(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#).is_empty());
    assert!(one(r#"{"jsonrpc":"2.0","id":null,"method":"ping"}"#).is_empty());
    assert_eq!(one(r#"{"jsonrpc":"2.0","id":"a","method":"ping"}"#)[0]["result"], json!({}));
    let tools = one(r#"{"jsonrpc":"2.0","id":3,"method":"tools/list"}"#);
    assert_eq!(tools[0]["result"]["tools"][0]["name"], "wisply");
    assert_eq!(tools[0]["result"]["tools"][0]["inputSchema"]["required"][0], "cmd");
    let ok =
        one(r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"wisply","arguments":{"cmd":"snap"}}}"#);
    assert_eq!(ok[0]["result"]["content"][0]["text"], "ok");
    assert_eq!(ok[0]["id"], 4);
    let wrong = one(r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"other"}}"#);
    assert_eq!(wrong[0]["error"]["code"], -32602);
    assert_eq!(one(r#"{"jsonrpc":"2.0","id":6,"method":"nope"}"#)[0]["error"]["code"], -32601);
    assert_eq!(one(r#"{"jsonrpc":"2.0","id":7,"method":"resources/list"}"#)[0]["result"]["resources"], json!([]));
    assert_eq!(one("{not json")[0]["error"]["code"], -32700);
    assert_eq!(one(r#"[{"jsonrpc":"2.0","id":8,"method":"ping"},{"jsonrpc":"2.0","method":"x"}]"#).len(), 1);
}

#[test]
fn long_waits_stretch_the_timeout() {
    let secs = |s: &str| wait_seconds(&wisply_proto::words(s).unwrap());
    assert_eq!(secs("listen"), 900.0);
    assert_eq!(secs("inbox @x --timeout 10m"), 600.0);
    assert_eq!(secs("wait e3 --timeout 1.5s"), 1.5);
    assert_eq!(secs("wait --timeout soon"), 0.0);
    assert_eq!(secs("snap --timeout 10m"), 0.0);
    assert_eq!(parse_ms(" 2H "), Some(7_200_000.0));
    assert_eq!(parse_ms("250"), Some(250.0));
    assert_eq!(parse_ms(".5s"), None);
    assert_eq!(parse_ms("5.s"), None);
    assert_eq!(parse_ms("1.2.3s"), None);
}

#[test]
fn base64_matches_rfc_4648() {
    let cases =
        [("", ""), ("f", "Zg=="), ("fo", "Zm8="), ("foo", "Zm9v"), ("foob", "Zm9vYg=="), ("foobar", "Zm9vYmFy")];
    for (raw, want) in cases {
        assert_eq!(base64(raw.as_bytes()), want);
    }
    assert_eq!(base64(&[255, 254, 253]), "//79");
}

#[test]
fn replies_carry_text_image_and_error_flag() {
    let dir = env::temp_dir().join(format!("lt-mcp-img-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let img = dir.join("a.jpg");
    fs::write(&img, b"foo").unwrap();
    let res = reply_content(Reply { code: 0, image: img.display().to_string(), out: "hi".into() });
    assert_eq!(res["content"][0]["text"], "hi");
    assert_eq!(res["content"][1]["data"], "Zm9v");
    assert_eq!(res["content"][1]["mimeType"], "image/jpeg");
    assert_eq!(res["isError"], false);
    assert_eq!(mime("C:/out/x.PNG"), "image/png");
    assert_eq!(mime("C:/out/x.webp"), "image/webp");
    let gone = dir.join("gone.jpg").display().to_string();
    let res = reply_content(Reply { code: 3, image: gone, out: String::new() });
    assert_eq!(res["content"].as_array().map(Vec::len), Some(1));
    assert_eq!(res["isError"], true);
    fs::remove_dir_all(&dir).unwrap();
}
