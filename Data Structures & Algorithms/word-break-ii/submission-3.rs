impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        let mut res = vec![];
        let mut arr = vec![];

        for word in word_dict {
            trie.add(&word);
        }

        Self::backtrack(&trie.root, &trie.root, &s, &s.chars().collect(), 0, 0, &mut arr, &mut res);

        res
    }

    fn backtrack(node: &TrieNode, root: &TrieNode, s: &str, chars: &Vec<char>, i: usize, start: usize, arr: &mut Vec<String>, res: &mut Vec<String>) {
        if i >= chars.len() {
            if i == start && !arr.is_empty() {
                res.push(arr.join(" "));
            }
            return;
        }

        let c = chars[i];
        let Some(next) = node.children.get(&c) else {
            return;
        };

        if next.word {
            arr.push(s[start..=i].to_string());
            Self::backtrack(root, root, s, chars, i + 1, i + 1, arr, res);
            arr.pop();
        }

        Self::backtrack(next, root, s, chars, i + 1, start, arr, res);
    }
}

#[derive(Debug)]
struct TrieNode {
    word: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode {
                word: false, children: HashMap::new(),
            }
        }
    }

    fn add(&mut self, word: &str) {
        let mut cur = &mut self.root;

        for c in word.chars() {
            cur = cur.children.entry(c).or_insert(
                TrieNode {
                    word: false, children: HashMap::new(),
                }
            )
        }
        cur.word = true;
    }

    fn find(&self, word: &str) -> bool {
        let mut cur = &self.root;

        for c in word.chars() {
            if !cur.children.contains_key(&c) {
                return false;
            }
            cur = cur.children.get(&c).unwrap();
        }
        cur.word
    }

    fn prefix(&self, word: &str) -> bool {
        let mut cur = &self.root;

        for c in word.chars() {
            if !cur.children.contains_key(&c) {
                return false;
            }
            cur = cur.children.get(&c).unwrap();
        }
        true
    }
}