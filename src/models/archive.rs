use crate::models;
use chrono::Datelike;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Archive {
    pub year: String,
    pub posts_index: Vec<usize>,
}

impl Archive {
    pub fn parse(posts: &[models::Post]) -> Vec<Archive> {
        let mut archives = std::collections::HashMap::new();
        for (index, p) in posts.iter().enumerate() {
            let year = p.datetime.unwrap().year();
            let archive = archives.entry(year.to_string()).or_insert(Archive {
                year: year.to_string(),
                posts_index: vec![],
            });
            archive.posts_index.push(index);
        }
        let mut values: Vec<Archive> = archives.into_values().collect();
        values.sort_by(|a, b| b.year.cmp(&a.year));
        values
    }
}

#[cfg(test)]
mod archive_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let posts = vec![
            models::Post {
                meta: models::post::PostMetadata {
                    title: "test1".to_string(),
                    comments: Some(true),
                    ..models::post::PostMetadata::default()
                },
                datetime: Some(
                    chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
                ..Default::default()
            },
            models::Post {
                meta: models::post::PostMetadata {
                    title: "test2".to_string(),
                    comments: Some(true),
                    ..models::post::PostMetadata::default()
                },
                datetime: Some(
                    chrono::NaiveDate::from_ymd_opt(2022, 1, 2)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
                ..Default::default()
            },
            models::Post {
                meta: models::post::PostMetadata {
                    title: "test3".to_string(),
                    comments: Some(true),
                    ..models::post::PostMetadata::default()
                },
                datetime: Some(
                    chrono::NaiveDate::from_ymd_opt(2023, 1, 3)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
                ..Default::default()
            },
        ];
        let archives = Archive::parse(&posts);
        assert_eq!(archives.len(), 3); // 2021, 2022, 2023
        assert_eq!(archives[0].year, "2023"); // year is in descending order
        assert_eq!(archives[0].posts_index.len(), 1); // index of post3
        assert_eq!(archives[0].posts_index[0], 2); // the index of the third post
    }
}
