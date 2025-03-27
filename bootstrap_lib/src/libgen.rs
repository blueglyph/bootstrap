use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, BufWriter, Read, Seek};
use bootstrap::CollectJoin;

/// Collects and indents the source given in `parts`.
pub fn indent_source(parts: Vec<Vec<String>>, indent: usize) -> String {
    let s = String::from_utf8(vec![32; indent]).unwrap();
    let mut source = String::new();
    let mut first = true;
    for part in parts {
        if !first {
            source.push('\n');
        }
        first = false;
        for string in part {
            for line in string.split("\n") {
                let cured_line = line.trim_end();
                if cured_line.len() > 0 {
                    source.push_str(&s);
                }
                source.push_str(cured_line);
                source.push('\n');
            }
        }
    }
    source
}

/// Reads the file `filename` and finds the part included between two tags `tag`.
/// Returns the text between the tags or `None` if they couldn't be found.
///
/// Each line is trimmed from any ending space characters and ends with `\n`.
pub fn get_tagged_source(filename: &str, tag: &str) -> Option<String> {
    let file_tag = format!("[{tag}]");
    let file = File::open(filename).ok()?;
    let result = BufReader::new(file).lines()
        .filter_map(|l| l.ok())
        .skip_while(|l| !l.contains(&file_tag))
        .skip(2)
        .take_while(|l| !l.contains(&file_tag))
        .map(|mut s| {
            s.truncate(s.trim_end().len());
            s
        })
        .join("\n"); // the last line won't end by `\n`, which removes the last empty line
    Some(result)
}

/// Replaces the text between two tags `tag` by `new_src` in the file `filename`. Returns `Ok` on
/// success, or `Err` on failure, either I/O or if the tags couldn't be found.
pub fn replace_tagged_source(filename: &str, tag: &str, new_src: &str) -> std::io::Result<()> {
    let file_tag = format!("[{tag}]");
    let file = File::open(filename)?;
    let mut buf = BufReader::new(file);
    let mut count = 0;
    let mut line = String::new();
    let mut after = String::new();
    let mut position = 0;
    loop {
        line.clear();
        match buf.read_line(&mut line) {
            Ok(n) => if n == 0 { return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("tag {file_tag} not found"))); }
            Err(e) => return Err(e),
        }
        if line.contains(&file_tag) {
            count += 1;
            match count {
                1 => {
                    position = buf.stream_position()?;
                }
                2 => {
                    after.push_str(&line);
                    buf.read_to_string(&mut after)?;
                    break;
                }
                _ => panic!()
            }
        }
    }
    let file = OpenOptions::new().write(true).open(filename)?;
    file.set_len(position)?;
    let mut buf = BufWriter::new(file);
    buf.seek(std::io::SeekFrom::End(0))?;
    write!(&mut buf, "\n{new_src}\n{after}")?;
    Ok(())
}
