use std::collections::{HashMap, HashSet};

pub fn solve() -> String {
    let (page_rules, updates) = super::get_input();

    let mut following_pages: HashMap<i32, Vec<i32>> = HashMap::new();

    for (first, second) in page_rules {
        let entry = following_pages.entry(first).or_default();
        entry.push(second);
    }

    let mut sum = 0;

    for update in updates {
        let mut valid = true;
        let mut prev_pages = HashSet::new();
        'updates: for page in &update {
            if let Some(following) = following_pages.get(page) {
                for p in following {
                    if prev_pages.contains(p) {
                        valid = false;
                        break 'updates;
                    }
                }
            }
            prev_pages.insert(page);
        }

        if valid {
            sum += middle_value(&update);
        }
    }

    sum.to_string()
}

fn middle_value(v: &[i32]) -> i32 {
    v[v.len() / 2]
}
