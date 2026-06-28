use crate::cmd;
use crate::models;

use log::{debug, info};

pub fn run_create(args: cmd::CreateArgs) {
    info!("Create new post or page");

    // 1. load config to get source directory
    let config = models::Config::from_file(models::DEFAULT_CONFIG_FILE).unwrap();
    debug!("Loaded config: {}", models::DEFAULT_CONFIG_FILE);

    // 2. create output file
    let content_dir = if args.page {
        config.get_pages_dir()
    } else {
        config.get_posts_dir()
    };
    let slug = args.path.trim_end_matches(".md").replace('/', "-");
    let mut path = std::path::PathBuf::new();
    path.push(content_dir);
    path.push(&args.path);
    // get basename as post title
    let basename = path.file_stem().unwrap().to_str().unwrap();

    create_empty_content(path.to_str().unwrap(), basename, &slug, args.page);
    let content_type = if args.page { "page" } else { "post" };
    info!("Created {}: {}", content_type, path.to_str().unwrap());
}

fn create_empty_content(path: &str, title: &str, slug: &str, is_page: bool) {
    let bytes = include_bytes!("initdata/new_post.md");
    let mut content = models::Post::new();
    content.meta.title = title.to_string();
    content.meta.slug = slug.to_string();
    content.meta.date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    content.meta.tags = Some(vec![]);
    if is_page {
        content.meta.template = Some("page.hbs".to_string());
    }
    content
        .content_markdown
        .push_str(&String::from_utf8_lossy(bytes));
    content.to_file(path).unwrap();
}
