use clap::Parser;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Main file
    #[arg(short, long)]
    main: String,

    /// Input file
    #[arg(short, long)]
    input: Vec<String>,

    /// Output file
    #[arg(short, long)]
    output: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Userdb {
    key: String,
    ci: i32,
    c: String,
    d: String,
    t: String,
}

fn parse_userdb(line: &str) -> Option<Userdb> {
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() != 3 {
        eprintln!("Invalid line: {}", line);
        return None;
    }

    let opt: Vec<&str> = parts[2].split_whitespace().collect();
    if opt.len() != 3 {
        eprintln!("Invalid line: {}", line);
        return None;
    }

    let vec_c: Vec<&str> = opt[0].split('=').collect();
    if vec_c.len() != 2 {
        eprintln!("Invalid line: {}", line);
        return None;
    }

    let ci: i32 = vec_c[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid line: {}", line);
        0
    });

    Some(Userdb {
        key: format!("{}\t{}", parts[0], parts[1]),
        ci,
        c: opt[0].to_string(),
        d: opt[1].to_string(),
        t: opt[2].to_string(),
    })
}

fn parser_line(h: &FileHeader, line: &str, m: &mut BTreeMap<String, Userdb>) {
    if let Some(mut userdb) = parse_userdb(line) {
        if let Some(v) = m.get_mut(&userdb.key) {
            if v.ci < userdb.ci {
                v.ci = userdb.ci;
                v.c = userdb.c;
            }
        } else {
            userdb.t = h.tick.clone();
            m.insert(userdb.key.clone(), userdb);
        }
    }
}

fn parser_file(h: &FileHeader, file: &str, m: &mut BTreeMap<String, Userdb>) -> io::Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        parser_line(h, &line?, m);
    }

    Ok(())
}

fn write_to_file(h: &FileHeader, m: BTreeMap<String, Userdb>, f: &str) -> io::Result<()> {
    let file = File::create(f)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(h.header.as_bytes())?;

    for (_, value) in m.iter() {
        writer.write_all(&convert_to_str(value).as_bytes())?;
    }

    Ok(())
}

fn convert_to_str(userdb: &Userdb) -> String {
    format!("{}\t{} {} {}\n", userdb.key, userdb.c, userdb.d, userdb.t)
}

#[derive(Debug)]
struct FileHeader {
    header: String,
    tick: String,
}

fn parser_main_file(file: &str, m: &mut BTreeMap<String, Userdb>) -> io::Result<FileHeader> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut file_header = FileHeader {
        header: String::new(),
        tick: String::new(),
    };

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            file_header.header.push_str(&line);
            file_header.header.push('\n');

            if line.starts_with("#@/tick") {
                let mut parts = line.split_whitespace();
                if let Some(t) = parts.nth(1) {
                    file_header.tick = format!("t={}", t);
                }
            }
        } else {
            parser_line(&file_header, &line, m);
        }
    }

    Ok(file_header)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut mapdb: BTreeMap<String, Userdb> = BTreeMap::new();
    let file_header = parser_main_file(&args.main, &mut mapdb)?;

    for f in &args.input {
        parser_file(&file_header, f, &mut mapdb)?;
    }

    write_to_file(&file_header, mapdb, &args.output)?;

    println!("Successfully merged files to {}", args.output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_str() {
        let u = Userdb {
            key: String::from("a"),
            ci: 0,
            c: String::from("b"),
            d: String::from("c"),
            t: String::from("d"),
        };
        assert_eq!("a\tb c d\n", convert_to_str(&u));
    }
    #[test]
    fn test_parse_userdb() {
        assert_eq!(
            parse_userdb("ce shi\t测试\tc=10 d=0.1111 t=12345"),
            Some(Userdb {
                key: String::from("ce shi\t测试"),
                ci: 10,
                c: String::from("c=10"),
                d: String::from("d=0.1111"),
                t: String::from("t=12345"),
            })
        );
        assert_eq!(parse_userdb(""), None);
        assert_eq!(parse_userdb("ce shi 测试\tc=10 d=0.1111 t=12345"), None);
        assert_eq!(parse_userdb("ce shi\t测试\tc10 d=0.1111 t=12345"), None);
        assert_eq!(parse_userdb("ce shi 测试\tc=a d=0.1111 t=12345"), None);
    }
    #[test]
    fn test_parser_line() {
        let h = FileHeader {
            header: String::from("aa"),
            tick: String::from("t=12345"),
        };
        let mut m: BTreeMap<String, Userdb> = BTreeMap::new();
        parser_line(&h, "ce shi\t测试\tc=10 d=0.1111 t=11111", &mut m);
        assert!(m.contains_key("ce shi\t测试"));
        let val = m.get("ce shi\t测试");
        assert!(!val.is_none());
        if let Some(v) = val {
            assert_eq!("ce shi\t测试", v.key);
            assert_eq!(10, v.ci);
            assert_eq!("c=10", v.c);
            assert_eq!("d=0.1111", v.d);
            assert_eq!("t=12345", v.t);
        }
    }
}
