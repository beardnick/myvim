use octocrab::models;
use octocrab::{Page, Error};
use octocrab::models::repos::Tag;

use crate::error::PlugError;


pub async fn get_tags(owner:&str,repo :&str) -> Result<Vec<Tag>,Error> {
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

pub fn repo(plug:&str) ->  Result<(&str,&str),PlugError> {
    let mut plug = plug.split("/");
    let owner = plug.next().ok_or(PlugError::InvalidRepo)?;
    let repo = plug.next().ok_or(PlugError::InvalidRepo)?;
    Ok((owner,repo))
}

#[cfg(test)]
mod tests {
    use crate::tokio_block;

    use super::*;

    //#[async_std::main]
    #[tokio::main]
    #[test]
    async fn test_get_tags() {
        let tags = get_tags("neoclide","coc.nvim")
            .await
            .expect("should not get error");
        assert_ne!(tags.len(),0);
    }

    #[test]
    fn repo_test() {
        let (owner,rep) = repo("a/b").unwrap();
        assert_eq!((owner,rep),("a","b"));
        assert!(repo("ab").is_err());
    }

    #[test]
    fn test_get_tags_sync(){
        let tags = tokio_block!( get_tags("neoclide","coc.nvim")).unwrap();
        assert_ne!(tags.len(),0);
    }

}
