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

    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);

    let filename: &str = &args[1];

    let mut path: String = env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    path.push_str("/");
    path.push_str(filename);

    let re_xy = Regex::new(r#"<svg width="(\d+)" height="(\d+)">$"#).unwrap();
    let re = Regex::new(r#"<circle cx="(\d+)" cy="(\d+)" r="(\d)" fill="(.*)"/>$"#).unwrap();

    let lines = lines_from_file(path);

    let text1: String = lines[3].to_string().trim().to_string();
    let caps = re_xy.captures(&text1).unwrap();
    let x: u32 = caps[1].to_owned().parse::<u32>().unwrap();
    let y: u32 = caps[2].to_owned().parse::<u32>().unwrap();

    let mut img = Image::new(x, y);

    for line in lines {
        let text: String = line.to_string().trim().to_string();

        // let caps = re.captures(&text).unwrap();
        // println!("{}", caps[1].to_owned().parse::<u32>().unwrap());
        // let cx: u32 = caps[1].to_owned().parse::<u32>().unwrap();
        // let cy: u32 = caps[2].to_owned().parse::<u32>().unwrap();
        // let fill = &caps[4].to_owned();
        // if fill == "white" {
        //     img.set_pixel(cx, cy, Pixel::new(255 ,255, 255));
        // } else {
        //     img.set_pixel(cx, cy, Pixel::new(0 ,0, 0));
        // }

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

    let mut path1: String = env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    path1.push_str("/");
    path1.push_str(filename);
    path1.push_str(".bmp");
    let _ = img.save(path1);
}

