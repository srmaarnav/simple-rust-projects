use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main(){
    let article: Article = Article{
        article: String::from("how to work with json"),
        author: String::from("srmaarnav"),
        paragraph: vec![
            Paragraph{
                name: String::from("First")
            },
            Paragraph{
                name: String::from("second")
            },
            Paragraph{
                name: String::from("Third")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is: {}", json);

    let json_pretty = serde_json::to_string_pretty(&article).unwrap();
    println!("The pretty json is: \n {}", json_pretty);
}