use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str,  action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house, {}", self.name);
                println!("{}", note);

                if self.age < 21 {
                    println!("Do not server alchol to {}", self.name);
                }
            },
            VisitorAction::Probation => println!("{} is now a probation member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn main() {
    let mut visitors_list = vec![
        Visitor::new("simon", VisitorAction::Accept, 31),
        Visitor::new("dexter", VisitorAction::AcceptWithNote { note: String::from("Lactose-free milk is in the fridge") }, 1),
        Visitor::new("luffy", VisitorAction::Probation, 1),
        Visitor::new("maria", VisitorAction::Refuse, 20),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");

        let your_name = what_is_your_name();
        // let allow_them_in =
        if your_name.is_empty() {
            break;
        }

        is_visitor_allowed(&mut visitors_list, &your_name);
    }

    println!("New list: {:#?}", visitors_list);
    // if allow_them_in {
    //     println!("Welcome to the Treehouse, {}!", your_name);
    // } else {
    //     println!("Sorry, you aren't on the list");
    // }
}

fn is_visitor_allowed(visitors_list: &mut Vec<Visitor>, your_name: &String) {
    let known_visitor = visitors_list
        .iter()
        .find(|visitor| visitor.name == *your_name);


    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => {
            println!("{} is not on the visitor list.", your_name);
            visitors_list.push(Visitor::new(&your_name, VisitorAction::Probation, 0));
        },
    }

    // let mut allow_them_in: bool = false;

    // for visitor in &visitors_list {
    //     if *visitor.name == *your_name {
    //         allow_them_in = true;
    //     }
    // }

    // allow_them_in
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failied to read line");

    your_name.trim().to_lowercase()
}
