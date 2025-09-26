#[derive(Debug)]
struct RatCrate {
    id: String,
    pub name: String,
    pub updated_at : Option<String>,
    pub keywords : Option<Vec<String>>,
}



fn main() {
    println!("Hello, world!");

    let s1 = RatCrate {
        id: "1".to_string(),
        name: "test".to_string(),
        updated_at: None,
        keywords: None,
    };

    println!("{:?}", s1);


    let s2 = RatCrate {
        id: "2".to_string(),
        name: "test2".to_string(),
        updated_at: None,
        keywords: Some(vec!["music".to_string(), "productivity".to_string()]),
    };

    println!("{:?}", s2);


    if let Some(keywords) = &s2.keywords {
        println!("{:?}", keywords);
    }



 }



