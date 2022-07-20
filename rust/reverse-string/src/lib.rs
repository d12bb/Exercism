use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut instr = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let mut res = String::new();
    while let Some(c) = instr.pop() {
        res.push_str(c);
    }
    res
}
