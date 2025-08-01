use std::fs::File;
use std::io::Write;
// use std::io;
use std::path::Path;

struct FileInfo{
    title: String,
    link: String,
    tags: String,
    map: String
}
fn i_frame(x: String) -> String {
    let first_half = String::from("<iframe allowFullScreen=True class=\"grenLineUp\"");
    let src = String::from(" src=");
    let second_half = String::from("></iframe>");

    return first_half + &src + "\"" + &x + "\"" + &second_half;
}
fn main() {
    let path = Path::new("./Quick Links.csv");
    let contents = path.display();
    let mut files:Vec<FileInfo> = Vec::new();
    let file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}", contents, why),
        Ok(file) => file,
    };
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        let title = &record[0];
        let map = &record[1];
        let tags = &record[2];
        let link = &record[3];
        // Print a debug version of the record.
        let map = FileInfo{title:title.to_string(),
             link: link.to_string(), 
             tags: tags.to_string(), 
             map: map.to_string()};
        files.push(map);
    }

    for f in files{
        let result = File::create(f.title+".md");
        let tag_list:Vec<&str> = f.tags.split(',').collect();
        let nadelen = tag_list[1].len()-1;
        let tags = format!(
"Side: {}
Nade: {}", &tag_list[0][1..], &tag_list[1][..nadelen]);
        let texrt = format!(
"---
{}
Map: {}
Link: {}
---

{}
", tags, f.map, f.link, i_frame(f.link[33..].to_string()));
        println!("{}", texrt);
        result.expect("Should be able to write to file").write_all(texrt.as_bytes());
    }
    
}

// https://youtu.be/-gpc5Raf7zk?si=zLUwWHCffAm_ioYB