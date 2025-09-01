// // Firstly, implementing the state pattern in a more traditional object-oriented way:

// #[allow(unused)]
// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     pub fn new() -> Self {
//         Self {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }

//     // Change to add text only in "Draft" state
//     // "Draft" state object responsible for `Post.content` but cannot modify `Post`
//     pub fn add_text(&mut self, text: &str) {
//         // self.content.push_str(text);

//         self.state
//             .as_ref()
//             .unwrap()
//             .add_text(&mut self.content, text);
//     }

//     pub fn content(&self) -> &str {
//         // Adding a placeholder implementation for the content method on Post
//         // that always returns an empty string slice
//         // ""

//         // Now implementing the appropriate content method
//         self.state.as_ref().unwrap().content(self)
//     }

//     // Add method to check content from "Draft" state before "Published" state
//     pub fn draft_content(&self) -> &str {
//         self.state.as_ref().unwrap().draft_content(self)
//     }

//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review());
//         }
//     }

//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve());
//         }
//     }

//     pub fn reject(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.reject());
//         }
//     }
// }

// // The `State` trait defines the behaviour shared by post states.
// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     // A interesting error occurs when I try to return `self` from any of the trait methods. I get:
//     // the size for values of type `Self` cannot be known at compilation time
//     // consider further restricting `Self`: ` where Self: Sized`
//     fn reject(self: Box<Self>) -> Box<dyn State>;

//     #[allow(unused)]
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }

//     #[allow(unused)]
//     fn draft_content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }

//     #[allow(unused)]
//     fn add_text(&self, content: &mut String, text: &str) {}
// }

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn add_text<'a>(&self, content: &mut String, text: &str) {
//         content.clear();
//         content.push_str(text);
//     }
// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Draft {})
//     }
// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
//     #[allow(unused)]
//     fn draft_content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }

// --------------------

// Encoding states and behaviour as types
// Taking advantage of Rust's strengths instead of traditional Object-oriented methods
// This way we can count on the compiler to enforce our code instead of incurring runtime performance.

pub struct Post {
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
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        if self.content.len() != 0 {
            self.content.clear();
        }
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

pub trait ViewDraftContent {
    fn draft_content(&self) -> &str;
}

impl ViewDraftContent for DraftPost {
    fn draft_content(&self) -> &str {
        &self.content
    }
}

impl ViewDraftContent for PendingReviewPost {
    fn draft_content(&self) -> &str {
        &self.content
    }
}
