use super::config::ProcessorConfig;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::preprocess::PreprocessorContext;

pub mod old_tag_adder;
pub mod tag_adder;

fn chapter_handle(chapter: &mut Chapter, config: &ProcessorConfig, src: &std::path::Path) {
    if config.use_old_tag_adder {
        old_tag_adder::add_a_tag(&mut chapter.content, config.add_link_for_chinese)
    } else {
        tag_adder::add_heading_anchors(&mut chapter.content, config.add_link_for_chinese);
    }
    if config.display_processed_contents {
        log::debug!("new content: {0}", chapter.content);
    }
    if config.do_link_check {
        crate::link_checker::check_link(
            &chapter.content,
            &chapter.source_path,
            src,
            &config.link_checker_config,
        );
    }
}

/// Preprocessed core handle.
pub fn handle(mut book: Book, config: ProcessorConfig, ctx: &PreprocessorContext) -> Book {
    let src = ctx.root.join(&ctx.config.book.src);
    book.for_each_mut(|book_item| {
        if let BookItem::Chapter(chapter) = book_item {
            chapter_handle(chapter, &config, &src)
        }
    });
    book
}
