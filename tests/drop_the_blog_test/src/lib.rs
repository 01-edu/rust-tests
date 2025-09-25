#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use drop_the_blog::*;

    #[test]
    fn test_is_dropped_and_drops() {
        let blog = Blog::new();
        let (pid, article) = blog.new_article(String::from("gnome-shell"));
        let (pid0, article0) = blog.new_article(String::from("i3"));
        let (pid1, article1) = blog.new_article(String::from("shell"));
        let (pid2, article2) = blog.new_article(String::from("spotify"));

        article.discard();
        assert_eq!(blog.drops.get(), 1);
        article0.discard();
        assert_eq!(blog.drops.get(), 2);

        assert!(blog.is_dropped(pid), "{} should have been dropped", pid);
        assert!(blog.is_dropped(pid0), "{} should have been dropped", pid0);
        assert!(
            !blog.is_dropped(pid1),
            "{} should not have been dropped",
            pid1
        );
        assert!(
            !blog.is_dropped(pid2),
            "{} should not have been dropped",
            pid2
        );

        article1.discard();
        article2.discard();
        assert_eq!(blog.drops.get(), 4);
    }

    #[test]
    fn test_using_rc() {
        let blog = Blog::new();
        let (_, article) = blog.new_article(String::from("Xorg"));
        let article = Rc::new(article);
        let article_clone = Rc::clone(&article);

        assert_eq!(Rc::strong_count(&article), 2);
        drop(article_clone);
        assert_eq!(Rc::strong_count(&article), 1);
    }

    #[test]
    #[should_panic]
    fn test_drop_same_article() {
        let blog = Blog::new();
        let (_, article) = blog.new_article(String::from("gsd-rfkill"));
        let article_clone = article.clone();
        article.discard();
        article_clone.discard();
    }
}
