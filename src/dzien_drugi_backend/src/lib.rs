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

#[ic_cdk::update]
fn remove_post(id: usize){
    POSTS.with(|posts: &RefCell<Vec<String>>| {
        posts.borrow_mut().remove(id)
    });
}

#[ic_cdk::update]
fn edit_post(id: usize, new_post: String){
    POSTS.with(|posts: &RefCell<Vec<String>>| {
        let mut binding = posts.borrow_mut();
        let post = binding.get_mut(id);
        let old_post = post.unwrap();
        *old_post = new_post;
    });
}