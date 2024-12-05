use std::collections::{HashMap, HashSet};

pub fn solve() -> String {
    let (page_rules, updates) = super::get_input();

    let mut following_pages: HashMap<i32, Vec<i32>> = HashMap::new();

    for (first, second) in page_rules {
        let following_entry = following_pages.entry(first).or_default();
        following_entry.push(second);
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
            continue;
        }

        let cmp = |a: &i32, b: &i32| {
            let empty_vec = vec![];
            let following_b = following_pages.get(b).unwrap_or(&empty_vec);
            let following_a = following_pages.get(a).unwrap_or(&empty_vec);
            if following_b.contains(a) {
                return std::cmp::Ordering::Less;
            } else if following_a.contains(b) {
                return std::cmp::Ordering::Greater;
            }

            std::cmp::Ordering::Equal
        };

        let mut sorted_update = update.clone();
        sorted_update.sort_by(cmp);
        sum += middle_value(&sorted_update);
    }

    sum.to_string()
}

fn middle_value(v: &[i32]) -> i32 {
    v[v.len() / 2]
}
