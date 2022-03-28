use std::error::Error;

use newsapi::{Articles, get_articles};

use colour::{dark_green, yellow};

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        yellow!("- {}\n\n", i.url);
    }
}

fn main() -> Result<(), NewsApiError>{
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=3de2b67f23714f0e9deb07b94762a79f";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
