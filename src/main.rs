use std::fmt;
use std::io;

struct HarvardWestminsterSource {
    author_last_name: String,
    author_first_initial: String,
    year_published: String,
    title: String,
    accessed_date: String,
    url: String,
}

impl fmt::Display for HarvardWestminsterSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}. ({}) {}. [Online] Available at: {} [Accessed {}].",
            self.author_last_name,
            self.author_first_initial,
            self.year_published,
            self.title,
            self.url,
            self.accessed_date
        )
    }
}

fn main() {
    let mut author_last_name = String::new();
    let mut author_first_initial = String::new();
    let mut year_published = String::new();
    let mut title = String::new();
    let mut accessed_date = String::new();
    let mut url = String::new();

    println!("Enter the author's last name:");
    io::stdin()
        .read_line(&mut author_last_name)
        .expect("Failed to read line");

    println!("Enter the author's first initial:");
    io::stdin()
        .read_line(&mut author_first_initial)
        .expect("Failed to read line");

    println!("Enter the year of publication:");
    io::stdin()
        .read_line(&mut year_published)
        .expect("Failed to read line");

    println!("Enter the title:");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    println!("Enter the accessed date:");
    io::stdin()
        .read_line(&mut accessed_date)
        .expect("Failed to read line");

    println!("Enter the URL:");
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    let source = HarvardWestminsterSource {
        author_last_name: author_last_name.trim().to_string(),
        author_first_initial: author_first_initial.trim().to_string(),
        year_published: year_published.trim().to_string(),
        title: title.trim().to_string(),
        accessed_date: accessed_date.trim().to_string(),
        url: url.trim().to_string(),
    };

    println!("{}", source);
}
