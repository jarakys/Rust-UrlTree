mod url_tree;
mod node;

use url_tree::UrlTree;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let mut tree: UrlTree<String> = UrlTree::new();
    //File with urls in this format: "google.com", "yandex.com", "abc.com"
    let mut file = File::open("../UrlTree/src/strings.txt").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let infinity_array: Vec<&str> = contents
        .split(',')
        .map(|s| {
            s.trim()
                .trim_matches('"')
        })
        .collect();

    let tmp_strings: Vec<String> = infinity_array
        .iter()
        .map(|s| format!("{}/", s))
        .collect();

    let tmp_refs: Vec<&str> = tmp_strings
        .iter()
        .map(|s| s.as_str())
        .collect();


    let start = Instant::now();
    for rule in &infinity_array {
        tree.root.add_rule(rule, |_v| {
            //ALLOWED;Banned/something else
            "sdsdsds".to_string()
        });
    }

    for rule in &tmp_refs {
        let callback = TestCallback::new(rule);
        tree.lookup_main(rule, &callback);
    }
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}

struct TestCallback {
    matched: RefCell<bool>,
    expected_rule: String,
}

impl TestCallback {
    fn new(expected_rule: &str) -> Self {
        TestCallback {
            matched: RefCell::new(false),
            expected_rule: expected_rule.to_string(),
        }
    }

    fn is_matched(&self) -> bool {
        *self.matched.borrow()
    }
}

impl<T> EnumerateNodesCallback<T> for TestCallback {
    fn on_matched(&self, rule: &str, _value: &T) -> bool {
        false
    }
}

pub trait EnumerateNodesCallback<T> {
    fn on_matched(&self, rule: &str, value: &T) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check_rule_matching(rule: &str, url: &str) {
        let mut tree: UrlTree<String> = UrlTree::new();
        tree.root.add_rule(rule, |v| {
            let value =  format!("{}{}", rule, url);
            println!("{}", value.to_string());
            format!("{}{}", rule, url)
        });
        let callback = TestCallback::new(rule);
        tree.lookup_main(url, &callback);
        assert!(
            callback.is_matched(),
            "Rule '{}' should match URL '{}'",
            rule,
            url
        );
    }
    fn check_rule_not_matching(rule: &str, url: &str) {
        let mut tree: UrlTree<String> = UrlTree::new();
        tree.root.add_rule(rule, |v| {
            format!("{}{}", rule, url)
        });
        let callback = TestCallback::new(rule);
        tree.lookup_main(url, &callback);
        assert!(
            !callback.is_matched(),
            "Rule '{}' should NOT match URL '{}'",
            rule,
            url
        );
    }

    #[test]
    fn test_matching_rule_travemuende_test() {
        check_rule_matching("travemuende-tourism.de", "travemuende-tourism.de");
    }

    #[test]
    fn test_matching_rule_google_com_test() {
        check_rule_matching("google.com/test", "google.com/test/test2.php");
    }

    #[test]
    fn test_matching_rule_google_com_test_with_www() {
        check_rule_matching("google.com/test", "www.google.com/test");
    }

    #[test]
    fn test_matching_rule_wildcard_google_com_test() {
        check_rule_matching("*.google.com/test", "www.google.com/test");
    }

    #[test]
    fn test_matching_rule_google_com_wildcard() {
        check_rule_matching("google.com/*", "google.com/test");
    }

    #[test]
    fn test_matching_rule_google_com_wildcard_test() {
        check_rule_matching("google.com/*/test", "google.com/query/kks/dasda/test");
    }

    #[test]
    fn test_not_matching_rule_google_com_wildcard_test() {
        check_rule_not_matching("google.com/*/test", "google.com/query/kks/dasda/t1est");
    }

    #[test]
    fn test_matching_rule_google_com_wildcard_test_question() {
        check_rule_matching("google.com/*/test?", "google.com/query/kks/dasda/test1");
    }

    #[test]
    fn test_not_matching_rule_test_google_com() {
        check_rule_not_matching("test.google.com/", "google.com");
    }

    #[test]
    fn test_matching_rule_google_com_with_www() {
        check_rule_matching("google.com", "www.google.com/query/kks/dasda/test2");
    }

    #[test]
    fn test_matching_rule_google_com_with_subdomain() {
        check_rule_matching("google.com", "www.test1.google.com/query/kks/dasda/test2");
    }

    #[test]
    fn test_matching_rule_twitter_com_shentongroup() {
        check_rule_matching("twitter.com/shentongroup_", "twitter.com/shentongroup_");
    }

    #[test]
    fn test_matching_rule_twitter_com_wildcard_shentongroup() {
        check_rule_matching(
            "twitter.com/*/shentongroup_",
            "test.twitter.com/dasdasdadas/shentongroup_",
        );
    }
}