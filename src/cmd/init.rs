use crate::models;
use log::{debug, info};

pub fn run_init() {
    info!("Initializing site...");

    // 1. create config
    let config = models::Config::default();
    config.to_file(models::DEFAULT_CONFIG_FILE).unwrap();
    debug!("initialized config file: {}", models::DEFAULT_CONFIG_FILE);

    // 2. create site directory
    config.mkdir_all().unwrap();

    // 3. init post
    let post_file = config.build_post_uri("hello-world.md");
    create_default_content(
        &post_file,
        "Hello World",
        "/hello-world",
        &["hello", "world"],
        None,
        include_bytes!("initdata/post.md"),
    );
    debug!("initialized default post: {}", post_file);

    // 4. init page
    let page_file = config.build_page_uri("about.md");
    create_default_content(
        &page_file,
        "About",
        "/about",
        &[],
        None,
        include_bytes!("initdata/page.md"),
    );
    debug!("initialized default page: {}", page_file);

    // 5. init theme
    models::ThemeEmbedAssets::extract(&config.directory.themes).unwrap();

    info!("Initializing success!");
}

fn create_default_content(path: &str, title: &str, slug: &str, tags: &[&str], template: Option<&str>, content_bytes: &[u8]) {
    let mut content = models::Post::new();
    content.meta.title = title.to_string();
    content.meta.slug = slug.to_string();
    content.meta.date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    if !tags.is_empty() {
        content.meta.tags = Some(tags.iter().map(|s| s.to_string()).collect());
    }
    if let Some(tpl) = template {
        content.meta.template = Some(tpl.to_string());
    }
    content
        .content_markdown
        .push_str(&String::from_utf8_lossy(content_bytes));
    content.to_file(path).unwrap();
}
