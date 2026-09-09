impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut res = HashSet::new();

        for email in emails {
            let (local, domain) = email.split_once('@').unwrap();

            let mut email = String::new();
            for c in local.chars() {
                if c == '.' {
                    continue;
                } else if c == '+' {
                    break;
                } else {
                    email.push(c);
                }
            }

            res.insert(format!("{email}@{domain}"));
        }

        res.len() as i32
    }
}
