impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut trie = Trie::new();

        let chars: Vec<char> = s.chars().collect();
        for word in &dictionary {
            trie.add(word);
        }

        let mut cache = vec![None; s.len()];

        Self::backtrack(&trie.root, 0, &chars, s.len(), &mut cache)
    }

    fn backtrack(root: &TrieNode, i: usize, chars: &[char], n: usize, cache: &mut Vec<Option<i32>>) -> i32 {
        if i >= n {
            return 0
        }

        if let Some(v) = cache[i] {
            return v;
        }

        let mut best = 1 + Self::backtrack(root, i+1, chars, n, cache);

        let mut node = root;
        for j in i..n {
            let Some(next) = node.children.get(&chars[j]) else { break };

            if next.word {
                best = min(best, Self::backtrack(root, j+1, chars, n, cache))
            }

            node = next;
        }

        cache[i] = Some(best);
        best
    }
}


#[derive(Debug)]
struct TrieNode {
    word: bool, children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            word: false, children: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn add(&mut self, word: &str) {
        let mut cur = &mut self.root;

        for c in word.chars() {
            cur = cur.children.entry(c).or_insert(TrieNode::new());
        }
        cur.word = true;
    }
}