use crate::EnumerateNodesCallback;
use crate::node::Node;

pub struct UrlTree<T> {
    pub root: Node<T>,
}

impl <T> UrlTree<T> {
    pub fn new() -> UrlTree<T> {
        UrlTree { root: Node {char: ' ', value: None, children: None} }
    }

    pub fn lookup_main<C: EnumerateNodesCallback<T>>(&mut self, url: &str, callback: &C) {
        let slash_index = url.find('/');
        if let Some(slash_index) = slash_index {
            self.lookup(&url[0..slash_index], &url[slash_index..], callback)
        }
    }

    fn lookup<C: EnumerateNodesCallback<T>>(&mut self, hostname: &str, path: &str, callback: &C) {
        let mut url = String::with_capacity(hostname.len() + path.len());
        url.push_str(hostname);
        url.push_str(path);
        if self.lookup_impl(url.as_str(), &self.root, false, "", callback) {
            let hostname_dot_pos = hostname.find(".");
            if let Some(position) = hostname_dot_pos {
                self.lookup(&hostname[position + 1..], path, callback);
            }
        }
    }

    fn lookup_impl<C: EnumerateNodesCallback<T>>(&self,  url: &str, node: &Node<T>, after_slash: bool, sb_rule: &str, callback: &C) -> bool {
        let mut slash_found = after_slash;
        let mut current_node = node;
        let mut sb_rule_new = String::with_capacity(sb_rule.len() + 1);
        for (index, char) in url.char_indices() {
            let Some(current_children) = &current_node.children else {
                return true
            };
            if char == '/' {
                slash_found = true;
            };

            let mut node_found = false;
            for child in  current_children {
                if !self.lookup_imp_wildcards(url, index, &child, slash_found, sb_rule, callback) {
                    return false
                }

                if child.char == char {
                    current_node = &child;
                    node_found = true;
                    break;
                }

                if child.char > char {
                    return true
                }
            }
            if !node_found {
                return true
            }
            sb_rule_new.push_str(sb_rule);
            sb_rule_new.push(current_node.char);

            let Some(value) = &current_node.value else {
                continue;
            };
            let next_char = url[index + char.len_utf8()..].chars().next();

            if slash_found || next_char == Some('/') || index + char.len_utf8() >= url.len() {
                if !callback.on_matched(sb_rule_new.as_str(), value) {
                    return false;
                }
            }
            sb_rule_new.clear();
        }
        true
    }

    fn lookup_imp_wildcards<C: EnumerateNodesCallback<T>>(&self,
                                                          url: &str,
                                                          pos: usize,
                                                          node: &Node<T>,
                                                          after_slash: bool,
                                                          sb_rule: &str,
                                                          callback: &C) -> bool {
        match node.char {
            '*' => self.lookup_wildcard(&url[pos..], node, after_slash, sb_rule, callback),
            '?' => self.lookup_impl_wildcard_char(&url[pos + 1..], node, after_slash, sb_rule, '?', callback),
            _ => true
        }
    }

    fn lookup_impl_wildcard_char<C: EnumerateNodesCallback<T>>(&self,
                                                               url: &str,
                                                               node: &Node<T>,
                                                               after_slash: bool,
                                                               sb_rule: &str,
                                                               wildcard_char: char,
                                                               callback: &C) -> bool {
        if let Some(value) = &node.value {
            let mut sb_rule_new = String::with_capacity(sb_rule.len() + 1);
            sb_rule_new.push_str(sb_rule);
            sb_rule_new.push(wildcard_char);
            return callback.on_matched(sb_rule_new.as_str(), value);
        }

        if url.is_empty() {
            return true
        }


        if let Some(children) = &node.children {
            let mut slash_found = after_slash;
            let url_char = url.chars().nth(0).unwrap();
            let mut sb_new_rule = String::with_capacity(sb_rule.len() + 2);
            sb_new_rule.push_str(sb_rule);
            sb_new_rule.push(wildcard_char);
            sb_new_rule.push(url_char);
            for child in children {
                if !self.lookup_imp_wildcards(url, 0, child, slash_found, sb_new_rule.as_str(), callback) {
                    return false;
                }
                if child.char > url_char {
                    break;
                }
                if child.char != url_char {
                    continue;
                }
                if url_char == '/' && !slash_found {
                    slash_found = true;
                }
                if !self.lookup_impl(&url[1..], child, slash_found, sb_new_rule.as_str(), callback) {
                    return false;
                }
            }
        };
        true
    }

    fn lookup_wildcard<C: EnumerateNodesCallback<T>>(&self,
                                                     url: &str,
                                                     node: &Node<T>,
                                                     after_slash: bool,
                                                     sb_rule: &str,
                                                     callback: &C) -> bool {
        if let Some(value) = &node.value {
            let mut sb_rule_new = String::with_capacity(sb_rule.len() + 1);
            sb_rule_new.push_str(sb_rule);
            sb_rule_new.push('*');
            let sb_new_rule_str = sb_rule_new.as_str();
            return callback.on_matched(sb_new_rule_str, value);
        }
        if url.is_empty() {
            return true
        }

        if let Some(children) = &node.children {
            let mut sb_rule_new = String::with_capacity(sb_rule.len() + 1);
            for child in children {
                let mut url_copy = url;
                loop {
                    let mut pos = url_copy.find(child.char);
                    if pos == None && !after_slash {
                        pos = url_copy.find('/');
                    }
                    if pos == None {
                        break;
                    }
                    if child.char != '/' && url_copy.chars().nth(pos.unwrap()).unwrap() == '/' {
                        break
                    }
                    sb_rule_new.push_str(sb_rule);
                    sb_rule_new.push(node.char);
                    let sb_new_rule_str = sb_rule_new.as_str();
                    url_copy = &url_copy[pos.unwrap() + 1..];
                    if !self.lookup_impl(url_copy, child, after_slash, sb_new_rule_str, callback) {
                        return false
                    }
                    sb_rule_new.clear()
                }
            }
        };
        true
    }
}