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
    BLOGS.with(|blogs| {
        blogs.borrow().clone()
    })
}