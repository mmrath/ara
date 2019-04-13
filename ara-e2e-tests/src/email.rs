use serde_json::{json, Map, Value};
use std::io::{BufRead, BufReader};

fn clean_mailbox(from: &str) {
    let client = reqwest::Client::new();
    let base_url = "http://127.0.0.1:8085/mail";
    client
        .delete(base_url)
        .json(&json!({
            "pruneCode": "all"
        }))
        .send()
        .unwrap();
}

pub fn fetch_mails(from: &str) -> Map<String, Value> {
    let base_url = "http://127.0.0.1:8085/mail";
    let body: Map<String, Value> = reqwest::get(base_url).unwrap().json().unwrap();
    body
}

pub fn download_and_run_mail_server() -> String {
    use std::io;
    use std::path::PathBuf;
    use std::{fs, fs::File};

    let out_dir = "../target/mailslurper/";

    let exe_path = PathBuf::from(out_dir).join("mailslurper");

    if exe_path.exists() {
        println!("mailslurper already downloaded");
        return run_mail_server();
    }

    let url = "https://github.com/mailslurper/mailslurper/releases/download/1.14.1/mailslurper-1.14.1-osx.zip";

    let mut resp = reqwest::get(url).expect("request failed");
    fs::create_dir_all(out_dir).expect("failed to create directory");

    let filename = "../target/mailslurper/mailslurper.zip";
    let mut out = File::create(filename).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");

    let file = File::open(filename).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let zip_fil_path = file.sanitized_name();
        let mut outpath = PathBuf::from(out_dir);
        outpath.push(zip_fil_path);
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    run_mail_server()
}

pub fn run_mail_server() -> String {
    use once_cell::sync::OnceCell;
    use std::process::{Child, Command, Stdio};

    let mail_bin = "./mailslurper";
    let mut cmd = Command::new(mail_bin);

    //    let mut kill_cmd = Command::new("pkill")
    //        .arg("-f")
    //        .arg(mail_bin)
    //        .spawn()
    //        .expect("Failed to kill existing mail process")
    //        .wait();

    println!("Command is {:?}", cmd);

    let mut process = cmd
        .current_dir("../target/mailslurper/")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Mail Slurper failed to start");

    println!("Mail Slurper process {}", process.id());
    let stdout = process.stderr.as_mut().unwrap();

    let stdout_reader = BufReader::new(stdout);

    let stdout_lines = stdout_reader.lines();
    let mut url = None;
    // ensures we only have regex init once
    // static cell: OnceCell<regex::Regex> = OnceCell::INIT;
    let re = regex::Regex::new(r"HTTP admin listener running on (localhost:\d*).*").unwrap();

    for line in stdout_lines {
        match line {
            Ok(text) => {
                if let Some(caps) = re.captures(&text) {
                    url = caps.get(1).map(|m| m.as_str().to_owned());
                    break;
                }
            }
            Err(error) => {
                dbg!(error);
            }
        }
    }
    println!("Mail slurper url: {:?}", url);
    url.unwrap()
}
