use mdbook::book::{Book, BookItem, Chapter};

fn add_a_tag(context: &mut String) {
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
        // For convenience, all header are now added
        /*
        if !title_content
            .chars()
            .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c))
        {
            new_content.push_str(line);
            new_content.push('\n');
            continue;
        }
        */

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

fn content_handle(context: &mut String) {
    add_a_tag(context);
    log::debug!("new context: {context}");
}

fn chapter_handle(chapter: &mut Chapter) {
    content_handle(&mut chapter.content);
}

pub fn handle(mut book: Book) -> Book {
    // for the future
    #[allow(clippy::single_match)]
    book.for_each_mut(|book_item| match book_item {
        BookItem::Chapter(chapter) => chapter_handle(chapter),
        _ => (),
    });
    book
}
