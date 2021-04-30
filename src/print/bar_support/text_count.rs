// based on https://github.com/spencerwooo/cwim
// MIT License
//
// Copyright (c) 2020 Spencer Woo
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

pub fn count(content: &String) -> usize {
    let mut word_count = 0;
    let mut line_count = 0;
    let mut blank_lines_count = 0;

    // regex to remove unnecessary whitespace inside markdown file
    // see VS Code documentation: https://vscode-docs.readthedocs.io/en/stable/extensions/example-word-count/
    let whitespace_re = Regex::new(
        r"(?x)
      (< ([^>]+)<)
      -
      ^\s\s*
      -
      \s\s*$
      ",
    )
    .unwrap();
    // match multiple spaces and change to single space
    let multiple_spaces_re = Regex::new(r"\s+").unwrap();
    // match links and files in grammar "[](...)"
    let link_re = Regex::new(r"]\((.*?)\)").unwrap();

    // process document
    for line in content.lines() {
        line_count = line_count + 1;
        let clean_line = String::from(line.trim());

        if !clean_line.is_empty() {
            // remove whitespace
            let clean_line = replace_whitespace(&clean_line, "", &whitespace_re);
            let clean_line = multiple_spaces_re.replace_all(&clean_line, " ");
            let clean_line = link_re.replace_all(&clean_line, "]");

            // split words using unicode standards
            let words: Vec<&str> = clean_line.unicode_words().collect();
            word_count = word_count + words.len();
        } else {
            blank_lines_count = blank_lines_count + 1;
        }
    }

    word_count
}

// replace whitespace according to regex pattern
fn replace_whitespace(input: &str, placeholder: &str, re: &Regex) -> String {
    re.replace_all(input, placeholder).into()
}
