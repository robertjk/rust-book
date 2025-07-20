use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}

fn page_title(url: &str) -> impl Future<Output = (&str, Option<String>)> {
    async move {
        let text = trpl::get(url).await.text().await;
        let title = Html::parse(&text)
            .select_first("title")
            .map(|title_element| title_element.inner_html());
        (url, title)
    }
}
