pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
        }
    }
}

pub enum ApprovalResult {
    Post(Post),
    PendingReviewPost(PendingReviewPost),
}

pub struct PendingReviewPost {
    content: String,
    approvals: usize,
}

impl PendingReviewPost {
    pub fn approve(mut self) -> ApprovalResult {
        self.approvals += 1;
        if self.approvals >= 2 {
            ApprovalResult::Post(Post {
                content: self.content,
            })
        } else {
            ApprovalResult::PendingReviewPost(self)
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::post::post_types::ApprovalResult;

    use super::Post;

    #[test]
    fn test_positive_post_lifecycle() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();

        let post = match post {
            ApprovalResult::PendingReviewPost(post) => post,
            ApprovalResult::Post(_) => {
                panic!("Post approved after just one approval");
            }
        };

        let post = post.approve();

        let post = match post {
            ApprovalResult::PendingReviewPost(_) => {
                panic!("Post still not approved after two approvals");
            }
            ApprovalResult::Post(post) => post,
        };

        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn post_lifecycle_with_reject() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.reject();

        let post = post.request_review();

        if let ApprovalResult::PendingReviewPost(post) = post.approve() {
            if let ApprovalResult::Post(post) = post.approve() {
                assert_eq!("I ate a salad for lunch today", post.content());
            }
        }
    }
}
