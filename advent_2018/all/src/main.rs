#![feature(external_doc)]
#![doc(include = "../../README.md")]

fn main() {
    println!("\nAdvent of Code 2018 Answers:\n");

    let day_fns: Vec<(fn() -> (String, String))> = vec![
        one::answer,
        two::answer,
        three::answer,
        four::answer,
        five::answer,
        six::answer,
        seven::answer,
        eight::answer,
        nine::answer,
        ten::answer,
        eleven::answer,
        twelve::answer,
        thirteen::answer,
        fourteen::answer,
        template::answer,
    ];

    for (n, answer) in day_fns.iter().enumerate() {
        let (first, second) = answer();
        println!("{:02}: ({:?}, {:?})", n + 1, first, second);
    }
}
