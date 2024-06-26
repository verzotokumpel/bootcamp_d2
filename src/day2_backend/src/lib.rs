thread_local! {
    static BLOGS: std::cell::RefCell<Vec<String>> = std::cell::RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, last_name: i8) -> String {
    format!("Hello, {} {}!", name, last_name)
}

#[ic_cdk::update]
fn add_blog(blog: String) {
    BLOGS.with(|blogs| {
        blogs.borrow_mut().push(blog);
    });
}

#[ic_cdk::query]
fn read_blogs() -> Vec<String> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}

#[ic_cdk::update]
fn delete_blog(id: usize) {
    BLOGS.with(|blogs| {
        blogs.borrow_mut().remove(id);
    });
}

#[ic_cdk::update]
fn edit_blog(id: usize, new_blog: String) {
    BLOGS.with(|blogs| {
        let mut binding_blogs = blogs.borrow_mut();
        let blog = binding_blogs.get_mut(id);
        let old_blog = blog.unwrap();
        if old_blog != &new_blog {
            *old_blog = new_blog;
        }
    });
}
