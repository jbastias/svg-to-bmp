use regex::Regex;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    env
};
use bmp::{
    Image, 
    Pixel
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
 
    let mut path: String = env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    let filename: &str = "/big-svg.svg";
    path.push_str(filename);

    let re = Regex::new(r#"<circle cx="(\d{1,3})" cy="(\d{1,3})" r="(\d)" fill="(.*)"/>$"#).unwrap();
    
    let mut img = Image::new(512, 414);

    let lines = lines_from_file(path);
    for line in lines {
        let text: String = line.to_string().trim().to_string();
        for cap in re.captures_iter(&text) {
            let cx: u32 = cap[1].to_owned().parse::<u32>().unwrap();
            let cy: u32 = cap[2].to_owned().parse::<u32>().unwrap();
            let fill = &cap[4].to_owned();

            if fill == "white" {
                img.set_pixel(cx, cy, Pixel::new(255 ,255, 255));
            } else {
                img.set_pixel(cx, cy, Pixel::new(0 ,0, 0));
            }
        }
    }

    let _ = img.save("./big-svg.bmp");

}

