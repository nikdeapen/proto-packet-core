/// An element with comment lines.
pub trait WithComments: Sized {
    /// Gets the comments.
    fn comments(&self) -> &[String];

    /// Adds the comment.
    fn with_comment<S>(mut self, comment: S) -> Self
    where
        S: Into<String>,
    {
        self.add_comment(comment);
        self
    }

    /// Adds the comment.
    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>;
}
