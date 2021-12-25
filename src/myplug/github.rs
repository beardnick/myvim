use octocrab::models;
use octocrab::{Page, Error};
use octocrab::models::repos::Tag;

async fn get_tags(owner:&str,repo :&str) -> Result<Vec<Tag>,Error> {
    let github = octocrab::instance();
    let mut page = github
        .repos(owner,repo)
        .list_tags()
        .send()
        .await?;
    let mut tags : Vec<Tag> = Vec::new();
    loop {
        for tag in &page { // 这里不能用&tag，因为&page是引用
            tags.push(tag.clone());
        }
        page = match github
            .get_page::<Tag>(&page.next)
            .await?
        {
            Some(next_page) => next_page,
            None => break,
        };
    }
        return Ok(tags);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::main]
    #[test]
    async fn test_get_tags() {
        let tags = get_tags("neoclide","coc.nvim")
            .await
            .expect("should not get error");
        assert_ne!(tags.len(),0);
    }
}
