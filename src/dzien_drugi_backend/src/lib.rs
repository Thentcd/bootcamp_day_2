use std::cell::RefCell;

thread_local! {
    static POSTS: RefCell<Vec<String>> = RefCell::default();
}


#[ic_cdk::query]
fn greet(name: String, surname: String) -> String {
    format!("Hello, {} {}!", name, surname)
}

#[ic_cdk::update]
fn add_post(post: String){
    POSTS.with(|posts: &RefCell<Vec<String>>| {
        posts.borrow_mut().push(post)
    })
}

#[ic_cdk::query]
fn read_posts() -> Vec<String>{
    POSTS.with(|posts: &RefCell<Vec<String>>| {
        posts.borrow().clone()
    })
}