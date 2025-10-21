use std::fs::File;
use std::io::Write;
use sqlite3 as sqlite;
// use std::io;
use std::path::Path;

#[derive(Clone)]
struct FileInfo{
    title: String,
    link: String,
    tags: String,
    map: String,
    nade_path: String,
    start: String,
    end: String,
    note_type: String,
    embed: String
}
fn i_frame(x: String, st:String, e: String) -> String {
    let first_half = String::from("<iframe allowFullScreen=True class=\"grenLineUp\"");
    let src = String::from(" src=\"https://www.youtube.com/embed/");
    let clip_times:String;
    let second_half = String::from("></iframe>");
    if st =="X"{
        return first_half + &src  + &x + "\"" + &second_half;
    } else {
        let start_time:i32 = st.parse().unwrap();
        let end_time:i32 = e.parse().unwrap();
        let dur = start_time + end_time;
        clip_times = format!("?start={}&end={}",st,dur);
        return first_half + &src  + &x + &clip_times+ "\"" + &second_half;
    }
    
}



fn tweet(x:String) -> String {
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
    let json:Vec<&str> = text.split(",").collect();
    let html = &json[3][8..];
    let mut obj: String = serde_json::from_str(&format!("\"{}\"",html)).expect("Failed Parsing Json");
	
    let end_handle = "</blockquote>\n<script async src=\"https://platform.twitter.com/widgets.js\" charset=\"utf-8\"></script>";
    obj.push_str(end_handle);
    // return html[7..].to_string();
    return obj;
}

fn nade_file(f:FileInfo) -> String{
    let tag_list:Vec<&str> = f.tags.split(',').collect();
    let nadelen = tag_list[1].len()-1;
    let start_end:String;
    if f.start != "X"{
        let start_time:i32 = f.start.parse().unwrap();
        let end_time:i32 = f.end.parse().unwrap();
        let dur = start_time + end_time;
        start_end = format!("Start:{}\nEnd:{}",f.start, dur);
    } else {
        start_end =String::from("");
    }
    let tags = format!(
"Side: {}
Nade: {}", &tag_list[0][1..], &tag_list[1][..nadelen]);
    let texrt = format!(
"---
{}
Map: {}
Link: {}
{}
---

{}
", tags, f.map, f.link, start_end, f.embed);

        
        return texrt.to_string();
}

fn tip_file(f:FileInfo) -> String{
    let text = format!(
"---
Map: {}
Side: {}
Link: {}
---

{}
",
f.map, f.tags, f.link, f.embed);
return text.to_string();
}

fn exec_file(f:FileInfo) -> String{
    let text = format!(
"---
Map: {}
Side: {}
Link: {}
Status: TODO
---
{}
",
f.map, f.tags, f.link, String::from("I gotta do this lol"));
return text.to_string();
}
fn main() {
    
    // const TIPS:String = "Tips & Tricks".to_string();
    // const EXEC:String = "Exec".to_string();
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
        let title:String;
        let map = &record[1];
        let note_type = &record[2];
        let tags = &record[3];
        let link = &record[4];
        let start = &record[5];
        let end = &record[6];
        let embed:String;
        let nade_path:String;
        match note_type{
            "Nade" => {
                nade_path = whendeez_path.to_owned() + map + "/"+ map + " Nades/";
                title = record[0].to_string();
            },
            "Exec" =>{ 
                nade_path = whendeez_path.to_owned() + map + "/Execs/";
                title =  record[0].to_string();
            },
            "Tip" => {
                nade_path = whendeez_path.to_owned() + map + "/Tips & Tricks/";
                title = record[0].to_string();
            },
            _ => {
                nade_path = String::from("Not yet implemented");
                title = String::from("Not yet implemented");
            },
        }
        let split_links:Vec<&str> = link.split('/').collect();
        // println!("{:?}",embed);
        match split_links[2]{
            "youtu.be" => embed = i_frame(split_links[3][..11].to_string(),start.to_string(),end.to_string()),
            "www.youtube.com" => embed = i_frame(split_links[4][..11].to_string(),start.to_string(),end.to_string()),
            "x.com" => embed = tweet(link.to_string()),
            _ => embed = "Not Implemented Yet".to_string(),
        }
        // Print a debug version of the record.
        let map = FileInfo{title:title.to_string(),
             link: link.to_string(), 
             tags: tags.to_string(), 
             map: map.to_string(),
            nade_path: nade_path.to_string(),
            start: start.to_string(),
            end: end.to_string(),
            note_type: note_type.to_string(),
            embed: embed};
        files.push(map);
    }
    for file in files{
        let f = file.clone();
        let result = File::create(f.nade_path + &f.title+".md");
        let text:String;
        
        match f.note_type.as_str(){
            "Nade" => text = nade_file(file),
            "Exec" => text = exec_file(file),
            "Tip" => text = tip_file(file),
            _ => text = String::from("Not yet bro"),
        }
        // println!("{}",text);
        result.expect("Should be able to write to file").write_all(text.as_bytes());
    }
    let mut t = File::create(&path).expect("Couldn't open file");
    t.write_all(String::from("Title,Map,Type,Tags,Link,Start,End").as_bytes()).expect("");
    // Tags are "(Side, Nade Type) | Side"
    // let connection = sqlite::open(":memory:").unwrap();
    // connection.execute(
    //     "SELECT * FROM NOTES"
    // )
    // println!("{}",connection);
}

// https://youtu.be/-gpc5Raf7zk?si=zLUwWHCffAm_ioYB
