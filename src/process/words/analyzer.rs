use std::collections::HashMap;

pub type WordCounter = HashMap<String, i32>;

pub fn analyze_words(sentence: &str) -> WordCounter {
    let words = sentiment::analyze(sentence.to_string());
    words
        .positive
        .words
        .iter()
        .fold(HashMap::new(), |mut acc, w| {
            acc.entry(w.to_string())
                .and_modify(|v| *v += 1)
                .or_insert(1);
            acc
        })
}
pub fn get_top_ten_words(words: WordCounter) -> WordCounter {
    let mut sorted_words: Vec<(&String, &i32)> = words.iter().collect();
    sorted_words.sort_by_key(|&(_, freq)| -freq);

    let mut top_ten_words: WordCounter = HashMap::new();

    sorted_words.iter().take(10).for_each(|(k, &v)| {
        top_ten_words.insert(k.to_string(), v);
    });

    top_ten_words
}

pub fn merge_words(acc: &mut WordCounter, words: WordCounter) {
    words
        .into_iter()
        .for_each(|(word, count)| *acc.entry(word).or_default() += count);
}

pub fn merge_words_per_id(id: u32, acc: &mut HashMap<u32, WordCounter>, words: WordCounter) {
    let words_entry: &mut WordCounter = acc.entry(id).or_default();
    merge_words(words_entry, words);
}
#[test]
fn test_analyze_words_should_work() {
    let words = analyze_words("you are awesome and nice! very nice");
    assert_eq!(words["awesome"], 1);
    assert_eq!(words["nice"], 2);
}
