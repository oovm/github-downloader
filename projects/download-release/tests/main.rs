use download_release::GithubDownloader;

#[test]
fn ready() {
    println!("it works!")
}


#[tokio::test]
async fn test() {
    let client = GithubDownloader::default().with_token("");
    client.list_releases("oovm", "wee").await.unwrap();
}