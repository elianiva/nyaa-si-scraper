use rustc_serialize::json::{Json, ToJson};
use std::env;
use std::fmt::{self, Display};
use std::str::FromStr;
use xmlJSON::XmlDocument;

#[derive(Debug)]
struct ResultItem<'a> {
    link: &'a str,
    title: &'a str,
    size: &'a str,
    seeders: &'a str,
    leechers: &'a str,
    downloads: &'a str,
    info_hash: &'a str,
    category: &'a str,
}

impl<'a> Display for ResultItem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\x1b[1;37m{} \x1b[0;0m
\x1b[1;33m Title: \x1b[0;0m{}
\x1b[1;33m﩯Category: \x1b[0;0m{}
\x1b[1;33m Link: \x1b[0;0;4;3;37m{}\x1b[0;0m
\x1b[1;33m遲Size: \x1b[0;0m{}
\x1b[1;33mﯲ Leechers: \x1b[0;0m{}
\x1b[1;33mﯴ Seeders: \x1b[0;0m{}
\x1b[1;33m Downloads: \x1b[0;0m{}
\x1b[1;33m Info Hash: \x1b[0;0m{}",
            "─".repeat(60),
            self.title,
            self.category,
            self.link,
            self.size,
            self.leechers,
            self.seeders,
            self.downloads,
            self.info_hash,
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let url: String = format!(
        "https://nyaa.si/?page=rss&q={}",
        args[1]
            .chars()
            .map(|x| if x == ' ' { '+' } else { x })
            .collect::<String>()
    );

    println!("\x1b[1;34mVisiting {}...", url);
    let res = reqwest::blocking::get(&url)?.text().unwrap();

    let document: XmlDocument = XmlDocument::from_str(&res).unwrap();
    let json_result: Json = document.to_json();

    for item in json_result
        .search("item")
        .unwrap()
        .clone()
        .into_array()
        .unwrap()
        .into_iter()
    {
        println!(
            "{}",
            ResultItem {
                link: get_item_from_object(&item, "link"),
                title: get_item_from_object(&item, "title"),
                info_hash: get_item_from_object(&item, "nyaa:infoHash"),
                downloads: get_item_from_object(&item, "nyaa:downloads"),
                size: get_item_from_object(&item, "nyaa:size"),
                category: get_item_from_object(&item, "nyaa:category"),
                leechers: get_item_from_object(&item, "nyaa:leechers"),
                seeders: get_item_from_object(&item, "nyaa:seeders"),
            }
        );
    }

    Ok(())
}

fn get_item_from_object<'a>(json_object: &'a Json, key: &str) -> &'a str {
    match json_object.find(key) {
        Some(result) => match result.find("_") {
            Some(result) => result.as_string().unwrap(),
            None => "Not Found",
        },
        None => "Not Found",
    }
}
