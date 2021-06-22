use serde::Serialize;
use tera::Context;
use tera::Tera;
use worstbird_db::models;
#[derive(Serialize)]
pub struct TemplateInfo {
    pub sel_year: u32,
    pub years: Vec<i32>,
    pub months: Vec<String>,
    pub months_num: Vec<u8>,
    pub birds: Vec<(models::Bird, i32)>,
    pub max_vote: u32,
}

#[derive(Serialize)]
struct TemplateInfoEmpty {
    sel_year: u32,
    years: Vec<i32>,
    months: Vec<String>,
}
fn main() {
    let tera = match Tera::new("templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let height = 180;

    let bird1 = models::Bird {
        id: 1,
        name: "bird1".to_string(),
        assetid: "40709911".to_string(),
        description: "lol".to_string(),
        height,
        width: 320,
        url: "asdf".to_string(),
    };
    let bird2 = models::Bird {
        id: 2,
        name: "bird2".to_string(),
        assetid: "40709911".to_string(),
        description: "lol".to_string(),
        height,
        width: 320,
        url: "asdf".to_string(),
    };
    let bird3 = models::Bird {
        id: 3,
        name: "bird3".to_string(),
        assetid: "40709911".to_string(),
        description: "lol".to_string(),
        height,
        width: 320,
        url: "asdf".to_string(),
    };
    let bird4 = models::Bird {
        id: 4,
        name: "bird4".to_string(),
        assetid: "40709911".to_string(),
        description: "lol".to_string(),
        height,
        width: 320,
        url: "asdf".to_string(),
    };
    let bird5 = models::Bird {
        id: 5,
        name: "bird5".to_string(),
        assetid: "40709911".to_string(),
        description: "lol".to_string(),
        height,
        width: 320,
        url: "asdf".to_string(),
    };

    let mut birds: Vec<(models::Bird, i32)> = Vec::new();
    birds.push((bird1, 0));
    birds.push((bird2, 1));
    birds.push((bird3, 1));
    birds.push((bird4, 0));
    birds.push((bird5, 0));

    let ti = TemplateInfo {
        sel_year: 2021,
        years: vec![2021, 2022, 2023],
        months: vec!["Jun".to_string(), "Jul".to_string(), "Aug".to_string()],
        months_num: vec![6, 7, 8],
        birds,
        max_vote: 1,
    };

    println!(
        "{}",
        tera.render("year.tera", &Context::from_serialize(&ti).unwrap())
            .unwrap(),
    );
}
