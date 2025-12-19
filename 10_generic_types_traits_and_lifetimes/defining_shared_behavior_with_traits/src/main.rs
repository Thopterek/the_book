use std::fmt::{Display, Debug};

fn main() {
    println!("Hello, shared traits!");
    let post = SocialPost {
        username: String::from("Thopterek"),
        content: String::from("well thats way too many applications anyway"),
        reply: true,
        repost: false,
    };
    println!("The post: {}", post.summarize());
    // using the default
    let article = NewsArticle {
        headline: String::from("We are going for it"),
        location: String::from("Heilbronn I think"),
        author: String::from("Me"),
        content: String::from("There is a Core game and we are joining it"),
    };
    println!("The article: {}", article.summarize());
    {
        // trying out the notify after fn with implmented trait
        notify(&article);
        notify(&post);
    }
    // and the less syntax sugar vesion
    bound_notify(&article);
    bound_notify(&post);
    let summarizable_thing = return_summarizable();
    println!("{}, I returned summarizable thing", summarizable_thing.summarize());
    println!("One type at the time only tho");
    // there are also blanked implementation like to_string
    // if the Trait has a Display it will also have a to_string version
    // impl<T: Display> ToString for T {}
}

/*
 * Getting a summary of other types through trait
 * each type implementing this trait must provide its custom version
*/
pub trait Summary {
    // version to implement for each
    // fn summarize(&self) -> String;
    // added default behaviour
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

/*
 * Usage of impl Trait syntax
 * to accept traits as params
 * can be modified with more traits:
 * pub fn notify(item: &(impl Summary + Display))
*/
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// or remade with trait bound snytax
// same for this one with more traits
// pub fn bound_notify<T: Summary + Display>(item: &T) {
pub fn bound_notify<T: Summary>(item: &T) {
    println!("Bound obeying news! {}", item.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // now using the default
    //fn summarize(&self) -> String {
    //    format!("{}, by {} ({})",
    //        self.headline,
    //        self.author,
    //        self.location)
    //}
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*
 * Implementing functions with trait bounds
 * can be huge like the one below or with where
*/
fn some_fn<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {}
// or the smaller version of the same thing
fn updated_some<T, U>(_t: &T, _u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{}

/*
 * Returning a type that implements a trait
 * syntax: -> impl [Type]
*/
fn return_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("Eeee"),
        content: String::from("its crazy"),
        reply: false,
        repost: false,
    }
}
