fn main() {
    let blog = BlogPost::new(
        String::from("My Title"),
        String::from("My Author"),
        String::from("My Content"),
    );
    println!(
        "method in Article{}",
        <BlogPost as Article>::get_full(&blog)
    ); // method in Article
    println!("method in BlogPost: {}", BlogPost::get_full(&blog)); // method in BlogPost
}

struct BlogPost {
    title: String,
    author: String,
    content: String,
}

impl BlogPost {
    fn new(title: String, author: String, content: String) -> BlogPost {
        BlogPost {
            title,
            author,
            content,
        }
    }
    fn get_full(&self) -> String {
        "yyyyy".to_string()
    }
}
// BlogPost implements ArticleAble so it can use the default implementation of Article
impl ArticleAble for BlogPost {
    fn get_author(&self) -> String {
        self.author.clone()
    }
    fn get_title(&self) -> String {
        self.title.clone()
    }
    fn get_content(&self) -> String {
        self.content.clone()
    }
}

// Any type that implements ArticleAble can use the default implementation of Article
trait ArticleAble {
    fn get_author(&self) -> String;
    fn get_title(&self) -> String;
    fn get_content(&self) -> String;
}

// Default implementation of Article. not strait forward but corresponding to open-closed principle,i.e open for extension but closed for modification.
trait Article {
    fn get_full<T>(a: &T) -> String
    where
        T: ArticleAble,
    {
        format!(
            "{} - {} - {}",
            a.get_author(),
            a.get_title(),
            a.get_content(),
        )
    }
}

// Automatically implement Article for any type that implements ArticleAble
impl<T> Article for T {}

