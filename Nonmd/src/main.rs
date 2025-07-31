use std::fs::File;
use std::io::Write;
// use std::io;
use std::path::Path;

struct FileInfo{
    title: String,
    link: String,
    tags: String,
    map: String,
    nade_path: String
}
fn i_frame(x: String) -> String {
    let first_half = String::from("<iframe allowFullScreen=True class=\"grenLineUp\"");
    let src = String::from(" src=\"https://www.youtube.com/embed/");
    let second_half = String::from("></iframe>");

    return first_half + &src  + &x + "\"" + &second_half;
}
fn main() {
    let cwd = std::env::current_dir();
    let res = cwd.expect("").display().to_string();
    let l = res.len() - String::from("Nonmd").len();
    let whendeez_path = &res[..l];
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
        let nade_path = whendeez_path.to_owned() + map + "/"+ map + " Nades/";
        // Print a debug version of the record.
        let map = FileInfo{title:title.to_string(),
             link: link.to_string(), 
             tags: tags.to_string(), 
             map: map.to_string(),
            nade_path: nade_path.to_string()};
        files.push(map);
    }

    for f in files{
        let result = File::create(f.nade_path + &f.title+".md");
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
", tags, f.map, f.link, i_frame(f.link[17..28].to_string()));
        println!("{}", texrt);
        result.expect("Should be able to write to file").write_all(texrt.as_bytes());
    }
    let mut t = File::create(&path).expect("Couldn't open file");
    t.write_all(String::from("Title,Map,Tags,Link").as_bytes()).expect("");

}

// https://youtu.be/-gpc5Raf7zk?si=zLUwWHCffAm_ioYB