fn main() {
    println!("Hello, world!");
    let blog_post = BlogPost {
        title: String::from("My Title"),
        author: String::from("My Author"),
        content: String::from("My Content"),
    };
    println!("{}", blog_post.get_full());
}


// strait forward but not corresponding to open-closed principle,i.e open for extension but closed for modification.
trait Article {
    // required methods
    fn new() -> Self;
    // required methods
    fn get_author(&self) -> String;
    // required methods
    fn get_title(&self) -> String;
    // required methods
    fn get_content(&self) -> String;
    // default methods
    fn get_full(&self) -> String {
        format!(
            "{} - {} - {}",
            self.get_author(),
            self.get_title(),
            self.get_content()
        )
    }
}

struct BlogPost {
    title: String,
    author: String,
    content: String,
}

impl Article for BlogPost {
    fn new() -> BlogPost {
        BlogPost {
            title: String::from(""),
            author: String::from(""),
            content: String::from(""),
        }
    }

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
