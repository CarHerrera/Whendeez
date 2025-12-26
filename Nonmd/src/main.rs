use regex::Regex;
use sqlite::State;
use sqlite3 as sqlite;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct FileInfo {
    title: String,
    link: String,
    tags: String,
    map: String,
    nade_path: String,
    start: String,
    end: String,
    note_type: String,
    embed: String,
}
fn i_frame(x: String, st: String, e: String) -> String {
    let first_half = String::from("<iframe allowFullScreen=True class=\"grenLineUp\"");
    let src = String::from(" src=\"https://www.youtube.com/embed/");
    let clip_times: String;
    let second_half = String::from("></iframe>");
    if st == "X" {
        return first_half + &src + &x + "\"" + &second_half;
    } else {
        let start_time: i32 = st.parse().unwrap();
        let end_time: i32 = e.parse().unwrap();
        let dur = start_time + end_time;
        clip_times = format!("?start={}&end={}", st, dur);
        return first_half + &src + &x + &clip_times + "\"" + &second_half;
    }
}

fn tweet(x: String) -> String {
    let url = String::from("https://publish.twitter.com/oembed?url=");
    let request = url + &x;
    let bodt = match reqwest::blocking::get(request) {
        Err(e) => panic!("{}", e),
        Ok(y) => y,
    };

    let text = match bodt.text() {
        Err(e) => panic!("{}", e),
        Ok(y) => y,
    };
    let json: Vec<&str> = text.split(",").collect();
    let html = &json[3][8..];
    let mut obj: String =
        serde_json::from_str(&format!("\"{}\"", html)).expect("Failed Parsing Json");

    let end_handle = "</blockquote>\n<script async src=\"https://platform.twitter.com/widgets.js\" charset=\"utf-8\"></script>";
    obj.push_str(end_handle);
    // return html[7..].to_string();
    return obj;
}

fn nade_file(f: FileInfo) -> String {
    let tag_list: Vec<&str> = f.tags.split(',').collect();
    let nadelen = tag_list[1].len() - 1;
    let start_end: String;
    if f.start != "X" {
        let start_time: i32 = f.start.parse().unwrap();
        let end_time: i32 = f.end.parse().unwrap();
        let dur = start_time + end_time;
        start_end = format!("Start: {}\nEnd: {}", f.start, dur);
    } else {
        start_end = String::from("");
    }
    let tags = format!(
        "Side: {}
Nade: {}",
        &tag_list[0][1..],
        &tag_list[1][..nadelen]
    );
    let texrt = format!(
        "---
{}
Map: {}
Link: {}
{}
---

{}
",
        tags, f.map, f.link, start_end, f.embed
    );

    return texrt.to_string();
}

fn tip_file(f: FileInfo) -> String {
    let text = format!(
        "---
Map: {}
Side: {}
Link: {}
---

{}
",
        f.map, f.tags, f.link, f.embed
    );
    return text.to_string();
}

fn exec_file(f: FileInfo) -> String {
    let text = format!(
        "---
Map: {}
Side: {}
Link: {}
Status: TODO
---
{}
",
        f.map, f.tags, f.link, f.embed
    );
    return text.to_string();
}
fn addPropertiesToDb(yamlContent:String, path:String) -> String{
    let yamlParse = Regex::new(r##"---([\s\w:\-\[\]\/\.\?\='"]+)---"##).unwrap();
    let out = yamlParse.find(yamlContent.as_str());
    let mut parse;
    match out {
        Some(val) => match val {
            val => parse = val.as_str(),
        },
        None => return String::new(),
    }
    // println!("Test String: {}", parse);
    // Multiline property search
    let re = Regex::new(r"(?m)(?<property>[\w]+): (?<value>.+)").unwrap();
    let yaml: Vec<(&str, &str)> = re.captures_iter(parse).map(|caps| {
        let prop = caps.name("property").unwrap().as_str();
        let value = caps.name("value").unwrap().as_str();
        (prop, value)
    }).collect();
    let title = path;
    println!("Adding File {}", title);
    // Open Notes and Grab them all, grab all the columns and their names.
    // Further down the line we will use the columns to check if a property needs to be added.
    let query = "SELECT * FROM NOTES";
    let connection = sqlite::open("./notes.db").unwrap();
    let mut statement = connection.prepare(query).unwrap();
    // Get the title added 
    let mut keys = String::from("(title,");
    let mut vals = String::from(format!("('{}',", title));
    let cols = statement.column_names().unwrap();
    for pair in yaml {
        // Push new columns onto key side and their corresponding value on the other
        keys.push_str(format!("{},", pair.0).as_str());
        vals.push_str(format!("'{}',", pair.1).as_str());
        // Check if columns have the "key" if not add it and set to text default
        if !cols.contains(&pair.0.to_string()) {
            let add_column = format!("ALTER TABLE notes ADD COLUMN {} TEXT", pair.0);
            connection.execute(add_column).unwrap();
        }
    }
    // println
    // This is a check to see if there were any properties in the file in the first place.
    // If not just pop the comma off and replace it with a paranthesis and then add it to DB.
    if (keys.len() != 7) {
        keys.pop();
        vals.pop();
        keys.push(')');
        vals.push(')');
        // println!("Keys!=7: {:?}\n Vals:{:?}", keys, vals);
        let q = format!("INSERT OR IGNORE INTO notes {} VALUES {}", keys, vals);
        connection.execute(q).unwrap();
    } else {
        keys.pop();
        vals.pop();
        keys.push(')');
        vals.push(')');
        let q = format!("INSERT OR IGNORE INTO notes {} VALUES {}", keys, vals);
        connection.execute(q).unwrap();
    }
    return title;
}
fn addFolder(file: PathBuf) -> String {
    for entry in fs::read_dir(file.as_path()).unwrap() {
        let fname = file.as_path().to_str().unwrap();
        let t = entry.unwrap();
        let ftype = t.file_type().unwrap();
        // println!("Folder: {}", fname);
        if ftype.is_dir() == true {
            addFolder(t.path());
        } else {
            match t.path().extension() {
                Some(ext) => match ext.to_str().unwrap() {
                    "md" => {
                        let content = fs::read_to_string(t.path()).unwrap();
                        addPropertiesToDb(content, t.path().display().to_string());
                        // This is regex to try and only get the things inside of the --- ---

                    }
                    _ => continue,
                },
                None => continue,
            }
        }
    }
    return file.to_str().unwrap().to_string();
}
fn main() {
    // const TIPS:String = "Tips & Tricks".to_string();
    // const EXEC:String = "Exec".to_string();
    // let cwd = std::env::current_dir();
    // let res = cwd.expect("").display().to_string();
    // let l = res.len() - String::from("Nonmd").len();
    // let whendeez_path = &res[..l];
    // let path = Path::new("./Quick Links.csv");
    // let contents = path.display();
    // let mut files: Vec<FileInfo> = Vec::new();
    // let file = match fs::File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", contents, why),
    //     Ok(file) => file,
    // };
    // let mut rdr = csv::Reader::from_reader(file);
    // for result in rdr.records() {
    //     // An error may occur, so abort the program in an unfriendly way.
    //     // We will make this more friendly later!
    //     let record = result.expect("a CSV record");
    //     let title: String;
    //     let map = &record[1];
    //     let note_type = &record[2];
    //     let tags = &record[3];
    //     let link = &record[4];
    //     let start = &record[5];
    //     let end = &record[6];
    //     let embed: String;
    //     let nade_path: String;
    //     match note_type {
    //         "Nade" => {
    //             nade_path = whendeez_path.to_owned() + map + "/" + map + " Nades/";
    //             title = record[0].to_string();
    //         }
    //         "Exec" => {
    //             nade_path = whendeez_path.to_owned() + map + "/Execs/";
    //             title = record[0].to_string();
    //         }
    //         "Tip" => {
    //             nade_path = whendeez_path.to_owned() + map + "/Tips & Tricks/";
    //             title = record[0].to_string();
    //         }
    //         _ => {
    //             nade_path = String::from("Not yet implemented");
    //             title = String::from("Not yet implemented");
    //         }
    //     }
    //     // let split_links: Vec<&str> = link.split('/').collect();
    //     let mut reg = Regex::new(r".+(?<domain>youtu.be|youtube.com|x.com)").unwrap();
    //     let code = reg.captures(link).unwrap();
    //     // println!("{:?}", &code["domain"]);
    //     match &code["domain"] {
    //         "youtu.be" => {
    //             reg = Regex::new(r"youtu.be/(?<embed>.+)\?").unwrap();
    //             let code = reg.captures(link).unwrap();
    //             // println!("{:?}", &code["embed"]);
    //             embed = i_frame(
    //                 code["embed"].to_string(),
    //                 start.to_string(),
    //                 end.to_string(),
    //             )
    //         }
    //         "youtube.com" => {
    //             reg = Regex::new(r"youtube.com/(?<option>watch\?v=|shorts/).+").unwrap();
    //             let code = reg.captures(link).unwrap();
    //             let funcString: &str;
    //             let out;
    //             // println!("{:?}", &code["option"]);
    //             match &code["option"] {
    //                 "watch?v=" => {
    //                     reg = Regex::new(r"youtube.com/watch\?v=(?<embed>.+)").unwrap();
    //                     out = reg.captures(link).unwrap();
    //                     funcString = &out["embed"];
    //                 }
    //                 "shorts/" => {
    //                     reg = Regex::new(r"youtube.com/shorts/(?<embed>.+)\?").unwrap();
    //                     out = reg.captures(link).unwrap();
    //                     funcString = &out["embed"];
    //                 }
    //                 _ => {
    //                     funcString = "";
    //                 }
    //             }
    //             // println!("{:?}", funcString);
    //             // embed = String::from("");
    //             embed = i_frame(funcString.to_string(), start.to_string(), end.to_string())
    //         }
    //         "x.com" => embed = tweet(link.to_string()),
    //         _ => embed = "Not Implemented Yet".to_string(),
    //     }
    //     // // Print a debug version of the record.
    //     let map = FileInfo {
    //         title: title.to_string(),
    //         link: link.to_string(),
    //         tags: tags.to_string(),
    //         map: map.to_string(),
    //         nade_path: nade_path.to_string(),
    //         start: start.to_string(),
    //         end: end.to_string(),
    //         note_type: note_type.to_string(),
    //         embed: embed,
    //     };
    //     files.push(map);
    // }
    // for file in files {
    //     let f = file.clone();
    //     let result = fs::File::create(f.nade_path + &f.title + ".md");
    //     let text: String;

    //     match f.note_type.as_str() {
    //         "Nade" => text = nade_file(file),
    //         "Exec" => text = exec_file(file),
    //         "Tip" => text = tip_file(file),
    //         _ => text = String::from("Not yet bro"),
    //     }
    //     println!("{}", text);
    //     result
    //         .expect("Should be able to write to file")
    //         .write_all(text.as_bytes());
    // }
    // let mut t = fs::File::create(&path).expect("Couldn't open file");
    // t.write_all(String::from("Title,Map,Type,Tags,Link,Start,End").as_bytes())
    //     .expect("");
    let root = Path::new("../");
    let _ = std::env::set_current_dir(&root);
    let notesFile = "./notes.db";
    match fs::File::open(notesFile) {
        Ok(file) => {
            println!("File found and opened successfully! Clearing table to make sure it is empty.");
            let connection = sqlite::open(notesFile).unwrap();
            connection.execute("DROP TABLE notes").unwrap();
            connection.execute("CREATE TABLE notes (title text)").unwrap();
        },
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                println!("File not found, creating a new one...");
                let mut file = fs::File::create(notesFile);
                let connection = sqlite::open(notesFile).unwrap();
                connection.execute("CREATE TABLE notes (title text)").unwrap();
                // Code to create file goes here
            } else {
                eprintln!("Problem opening the file: {:?}", error);
            }
        }
    }
    for entry in fs::read_dir(".").unwrap(){
        let t = entry.unwrap();
        let p = t.path();
        let fname = p.to_str().unwrap();
        let ftype = t.file_type().unwrap();
        match fname {
            "./Nonmd" | "./.git" | "./.trash" | "./.obsidian" | "./Maps" | "./Templates"  => continue,
            _ => {
                if ftype.is_dir() == true{
                    // println!("{}",fname);
                    println!("Checking Folder: {}",addFolder(p));
                } else {
                    match p.extension() {
                        Some(ext) =>  match ext.to_str().unwrap() {
                                "md" =>  {
                                    let content = fs::read_to_string(&p).unwrap();
                                    addPropertiesToDb(content, p.display().to_string());
                                },
                                _ => continue,
                            },
                        None => continue,
                    }
                }
            }
        }

        // println!("{:?} {:?}",t.path(), t.file_type());
    }
    // let query =  "SELECT * FROM NOTES";
    // let connection = sqlite::open("./notes.db").unwrap();
    // let mut statement = connection.prepare(query).unwrap();
    // let cols = statement.column_names().unwrap();
    // for x in cols {
    //     println!("{}",x);
    // }
    // while let State::Row = statement.next().unwrap() {
    //     println!("Title = {}", statement.read::<String>(0).unwrap());
    // }
    // connection.iterate(query, |out| {
    //     for &(name, value) in out.iter(){
    //         println!("Output: {} = {}", name, value.unwrap());
    //     }
    //     true
    // }).unwrap();
}

// https://youtu.be/-gpc5Raf7zk?si=zLUwWHCffAm_ioYB
