// // Using OOP in Rust, the traditional way.
// // Will later use an approach that is more natural to Rust.

use blog::{Post, ViewDraftContent};

// fn main() {
//     // Post starts of with "Draft" state
//     let mut post = Post::new();

//     // `add_text` can only work on "Draft" state
//     post.add_text("I ate a salad for lunch today!");
//     assert_eq!("I ate a salad for lunch today!", post.draft_content());
//     assert_eq!("", post.content());

//     // Change to "PendingReview" state
//     post.request_review();
//     post.add_text("New text that won't be added");
//     assert_eq!("I ate a salad for lunch today!", post.draft_content());
//     assert_eq!("", post.content());

//     // Reject to "PendingReview" content to send back to "Draft" state
//     post.reject();
//     post.add_text("New content after rejecting former.");
//     assert_eq!("New content after rejecting former.", post.draft_content());
//     assert_eq!("", post.content());

//     post.approve(); // Send to "PendingReview" state
//     post.approve(); // Send to "Published" state
//     assert_eq!("New content after rejecting former.", post.content());
//     assert_eq!("", post.draft_content());
// }

// -------------------

// Now, using OOP in Rust, through encoding states and behaviour as types.
// This is the Rust way.

fn main() {
    // let mut post = Post::new();
    // post.add_text("I ate a salad for lunch today!");

    // let post = post.request_review();

    // let post = post.approve();
    // assert_eq!("I ate a salad for lunch today!", post.content());

    // -------------

    // Let's add more functionality as well as the new `ViewDraftPost` trait behaviour
    let mut post = Post::new(); // Creates "DraftPost" type
    post.add_text("I ate a salad for lunch today!");
    post.add_text("New text content #1");
    println!("{}", post.draft_content());

    let post = post.request_review(); // Creates "PendingReviewPost" type
    let post = post.approve(); // Creates "Post" type
    assert_eq!("New text content #1", post.content());

    let mut post1 = Post::new(); // Creates "DraftPost" type
    post1.add_text("New text content #2");

    let post1 = post1.request_review(); // Creates "PendingReviewPost" type
    let mut post1 = post1.reject(); // Reverts to "DraftPost" type
    post1.add_text("New text content #2-1");

    let post1 = post1.request_review();
    println!("{}", post1.draft_content());

    let post1 = post1.approve(); // Creates "Post" type
    assert_eq!("New text content #2-1", post1.content());
}
