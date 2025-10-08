use super::config::ProcessorConfig;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::preprocess::PreprocessorContext;

fn add_a_tag(context: &mut String, check_language: bool) {
    let mut in_code_block = false;
    let mut new_content = String::new();
    let mut id_counter = 0;

    for line in context.lines() {
        // Handling Code Block Markers
        if line.starts_with("```") || line.starts_with("~~~") {
            in_code_block = !in_code_block;
            new_content.push_str(line);
            new_content.push('\n');
            continue;
        }

        // Skip content within a code block
        if in_code_block {
            new_content.push_str(line);
            new_content.push('\n');
            continue;
        }

        // Check Header Line
        let header_level = line.chars().take_while(|&c| c == '#').count();
        if header_level == 0 || header_level > 6 {
            new_content.push_str(line);
            new_content.push('\n');
            continue;
        }

        let rest_of_line = &line[header_level..];
        if rest_of_line.is_empty() || !rest_of_line.starts_with(' ') {
            // Auto-fill spaces
            let new_line = format!("{} {}", &line[..header_level], rest_of_line.trim_start());
            new_content.push_str(&new_line);
            new_content.push('\n');
            //continue;
        }

        let title_content = rest_of_line.trim();

        // Check if it contains Chinese characters
        if check_language
            && !title_content
                .chars()
                .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c))
        {
            new_content.push_str(line);
            new_content.push('\n');
            continue;
        }

        // Generate Unique ID
        let id = if title_content.is_empty() {
            // Roll back those that are not suitable
            format!("header-{}", id_counter)
        } else {
            title_content
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>()
                .to_lowercase()
        };

        id_counter += 1;

        // Reconstruct row line
        let new_line = format!(
            "{} <a id=\"{}\"></a>{}",
            &line[..header_level],
            id,
            &line[header_level..]
        );
        new_content.push_str(&new_line);
        new_content.push('\n');
    }

    *context = new_content;
}

fn chapter_handle(chapter: &mut Chapter, config: &ProcessorConfig, ctx: &PreprocessorContext) {
    add_a_tag(&mut chapter.content, config.add_link_for_chinese);
    if config.display_processed_contexts {
        log::debug!("new context: {0}", chapter.content);
    }
    if config.do_link_check {
        crate::link_checker::check_link(&chapter.content, &chapter.source_path, &ctx.root);
    }
}

pub fn handle(mut book: Book, config: ProcessorConfig, ctx: &PreprocessorContext) -> Book {
    book.for_each_mut(|book_item| {
        if let BookItem::Chapter(chapter) = book_item {
            chapter_handle(chapter, &config, ctx)
        }
    });
    book
}
