use mdbook::book::{Book, BookItem, Chapter};

fn content_handle(context: &mut String) {
    log::debug!("context: {context}");
    todo!()
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
