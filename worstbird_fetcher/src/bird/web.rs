use reqwest::header::HeaderValue;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use imagesize;

pub fn get_bird_name(data: &str) -> Result<String> {
    let name: String = data
        .split(r#"Media--hero-title">"#)
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

fn calculate_lines_to_add(chars_count: i32, max_chars_count: i32) -> i32 {
    chars_count / max_chars_count + (chars_count % max_chars_count != 0) as i32
    // chars_count / max_chars_count
}

pub fn get_text_size(embed_id: &str) -> Result<usize> {
    let url = format!("https://macaulaylibrary.org/asset/{}/embed/320", embed_id);
    println!("{}", &url);
    let response = reqwest::blocking::get(&url)?;
    let text = response.text()?;

    let result = text
        .split("<span")
        .skip(1)
        .take(4)
        .map(|e| {
            e.chars()
                .skip_while(|c| c != &'>')
                .skip(1)
                .take_while(|c| c != &'<')
                .collect()
        })
        .collect::<Vec<String>>();
    println!("{:?}", result);

    let first_span = result.get(0).ok_or("Couldn't get span 1")?.len()
        + result.get(1).ok_or("Couldn't get span 2")?.len();
    let character_count = 50;

    let author_span = result
        .get(2)
        .ok_or("Couldn't get author span (span 3)")?
        .len();

    let location_max_count = 40;
    let location_span = result
        .get(3)
        .ok_or("Couldn't get location span (span 4)")?
        .len();

    let new_lines_to_add = calculate_lines_to_add(first_span as i32, character_count) * 21
        + calculate_lines_to_add(author_span as i32, character_count) * 20
        + calculate_lines_to_add(location_span as i32, location_max_count) * 24;

    println!("{}", new_lines_to_add);
    Ok(new_lines_to_add as usize)
}

pub fn get_image_size(embed_id: &str) -> Result<(usize, usize)> {
    let url = format!(
        "https://cdn.download.ams.birds.cornell.edu/api/v1/asset/{}/320",
        embed_id
    );
    println!("{}", &url);
    let response = reqwest::blocking::get(&url)?;
    match imagesize::blob_size(&response.bytes()?) {
        Ok(dim) => Ok((dim.width, dim.height)),
        Err(_) => Err("Could not get image dimensions".into()),
    }
}

pub fn get_iframe_size(embed_id: &str) -> Result<(usize, usize)> {
    let default_bottom_height = 32;
    let margin_top_bottom = 21;
    let (width, height) = get_image_size(embed_id)?;
    let add_lines = get_text_size(embed_id)?;

    Ok((
        width,
        height + add_lines + default_bottom_height + margin_top_bottom,
    ))
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
        HeaderValue::from_str("de;q=0.5")?,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_character_size() {
        println!("{:?}", get_iframe_size("120288421").unwrap());
    }
    #[test]
    fn check_character_size2() {
        println!("{:?}", get_iframe_size("216006931").unwrap());
    }
    #[test]
    fn check_character_size3() {
        println!("{:?}", get_iframe_size("188044411").unwrap());
    }
    #[test]
    fn check_character_size4() {
        println!("{:?}", get_iframe_size("104239521").unwrap());
    }
}
