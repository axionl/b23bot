use curl::easy::Easy;
use regex::Regex;
use json;

fn download(url: &String, dst: &mut Vec<u8>) {
    let mut easy = Easy::new();

    easy.useragent("Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.7113.93 Safari/537.36").unwrap();
    
    easy.url(url).unwrap();

    let mut transfer = easy.transfer();

    transfer.write_function(|data| {
        dst.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();

    transfer.perform().unwrap();
}

pub fn get_aid(url: &String) -> Result<String, json::Error> {
    let mut dst: Vec<u8> = Vec::new();

    download(url, &mut dst);

    let msg = std::str::from_utf8(&dst).unwrap();

    let re = Regex::new(r"(?i)(?:bilibili\.com/(?:video|bangumi/play)|b23\.tv|acg\.tv)/(?:(?P<bvid>bv\w+)|av(?P<aid>\d+)|ep(?P<epid>\d+)|ss(?P<ssid>\d+))").unwrap();

    let mut bid = String::new();

    for caps in re.captures_iter(msg) {
        bid = caps.get(1).unwrap().as_str().to_string();
        break;
    }

    let api_url = format!("https://api.bilibili.com/x/web-interface/view?bvid={}", bid);

    let mut json_data: Vec<u8> = Vec::new();

    download(&api_url, &mut json_data);

    let json_str = std::str::from_utf8(&json_data).unwrap();

    let parsed = json::parse(json_str)?;

    let result = format!("https://b23.tv/av{}", parsed["data"]["aid"]);

    log::info!("Result url: {}", result);

    Ok(result)
}