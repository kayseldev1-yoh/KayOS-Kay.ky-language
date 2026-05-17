pub fn get(url: &str) -> String {
    reqwest::blocking::get(url).unwrap().text().unwrap()
}
