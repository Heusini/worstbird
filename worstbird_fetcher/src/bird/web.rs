use reqwest::header::HeaderValue;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use imagesize;

pub fn get_bird_name(data: &str) -> Result<String> {
    let name: String = data
        .split(r#"<h1 class="Media--hero-title">"#)
        .nth(1)
        .ok_or("no bird name")?
        .chars()
        .take_while(|e| e != &'<')
        .collect();
    Ok(name)
}

pub fn get_description(data: &str) -> Result<String> {
    let description: String = data
        .split(r#"<p class="u-stack-sm">"#)
        .nth(1)
        .ok_or("description not found")?
        .chars()
        .take_while(|e| e != &'<')
        .collect::<String>()
        .trim()
        .to_string();
    Ok(description)
    // println!("Description:\n{}", description);
}

pub fn get_image_size(embed_id: &str) -> Result<(usize, usize)> {
    let url = format!(
        "https://cdn.download.ams.birds.cornell.edu/api/v1/asset/{}/320",
        embed_id
    );
    println!("{}", &url);
    let response = reqwest::blocking::get(&url).unwrap();
    match imagesize::blob_size(&response.bytes().unwrap()) {
        Ok(dim) => Ok((dim.width, dim.height)),
        Err(_) => Err("Could not get image dimensions".into()),
    }
}

pub fn get_embbed(data: &str) -> Result<String> {
    let embed: String = data
        .split(r#"var photoAssetsJson"#)
        .nth(1)
        .ok_or("error no photoAssets Script")?
        .split("citationUrl\" : ")
        .nth(1)
        .ok_or("no citiaon Url in photoAssets")?
        .chars()
        .take_while(|e| e != &',')
        .collect::<String>()
        .split("/")
        .last()
        .ok_or("No data in citiation Url")?
        .trim_end_matches('\"')
        .to_string();
    Ok(embed)
}

pub fn suprise_me(url: &str) -> Result<(String, String)> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Referer",
        HeaderValue::from_str("https://ebird.org/explore")?,
    );
    // headers.insert("Host", HeaderValue::from_str("ebird.org")?);
    headers.insert(
        "Accept",
        HeaderValue::from_str(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        )?,
    );
    headers.insert(
        "User-Agent",
        HeaderValue::from_str(
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0",
        )?,
    );

    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        HeaderValue::from_str("gzip, deflate, br")?,
    );

    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        HeaderValue::from_str("en-US,en;q=0.5")?,
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .gzip(true)
        // important for redirect with cookie
        .cookie_store(true)
        .build()?;

    let res = client.get(url).send()?;
    Ok((res.url().to_string(), res.text()?))
}
