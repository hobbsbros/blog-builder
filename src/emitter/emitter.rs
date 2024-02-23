//! Emitter for the Blog Builder.

use super::Metadata;

use crate::Expression;

pub struct Emitter {
    metadata: Metadata,
}

impl Emitter {
    pub fn new(metadata: &Metadata) -> Self {
        Self {
            metadata: metadata.to_owned(),
        }
    }

    pub fn emit(&self, expressions: Vec<Expression>, pagename: &str) -> Vec<u8> {
        let mut output = String::new();

        output.push_str("<html>\n<head>");

        if let Some (t) = self.metadata.get_analytics() {
            output.push_str(&t);
        }

        if let Some (ss) = self.metadata.get_stylesheet() {
            output.push_str(&ss);
        }

        if let Some (lk) = self.metadata.get_links() {
            output.push_str(&lk);
        }

        if self.metadata.is_icon() {
            output.push_str("\n<link rel=\"icon\" type=\"image/x-icon\" href=\"/media/favicon.ico\">\n");
        }

        let full_pagename = if let Some (st) = self.metadata.get_sitename() {
            format!("<title>{} | {}</title>", pagename, st)
        } else {
            format!("<title>{}</title>", pagename)
        };
        output.push_str(&full_pagename);

        output.push_str("</head>\n");

        output.push_str("\n<body>\n");

        for expr in expressions {
            let output_str = format!("{}\n", expr);
            output.push_str(&output_str);
        }

        output.push_str("\n</body>\n");

        output.push_str("</html>");

        if let Some (m) = self.metadata.get_menu() {
            output = str::replace(&output, "<menu>", &m);
        }

        // Assemble footnotes
        let mut footnote_number = 1;
        let mut new_output = String::new();
        let mut footnotes = Vec::<String>::new();
        for footnote in output.split("<footnote>") {
            let end_index = footnote.find("</footnote>").unwrap_or(footnote.len());
            if end_index < footnote.len() {
                let content = &footnote[0..end_index];
                let rest = &footnote[end_index+11..];
                new_output.push_str(
                    &format!(
                        "<a id=\"citation-{}\" href=\"#footnote-{}\"><sup>{}</sup></a>",
                        footnote_number,
                        footnote_number,
                        footnote_number,
                    ),
                );
                footnote_number += 1;
                new_output.push_str(&rest);
                footnotes.push(content.to_string());
            } else {
                new_output.push_str(&footnote);
            }
        }
        output = new_output;

        let mut footnote_text = "<h3>Footnotes</h3>\n".to_string();
        for (index, footnote) in footnotes.iter().enumerate() {
            footnote_text.push_str(
                &format!(
                    "<p>[<a id=\"footnote-{}\" href=\"#citation-{}\">{}</a>] {}</p>",
                    index + 1,
                    index + 1,
                    index + 1,
                    footnote,
                ),
            );
        }

        output = str::replace(&output, "<footnotes>", &footnote_text);

        
        output.chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>()
    }
}