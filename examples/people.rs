use rearrange::Rearrange;

fn main() {
    let mut people = vec![
        // Please PR better a better example if you can think of one 😅
        Person::new("Michael", "Foo", 24),
        Person::new("Michael", "Foo", 10),
        Person::new("Jeff", "Bar", 25),
        Person::new("Maria", "Foo", 25),
        Person::new("Maria", "Bar", 26),
    ];

    #[rustfmt::skip]
    people.rearrange(|order| order
        .by_ref(|p| &p.last_name)
        .by_ref(|p| &p.first_name)
        .reverse_by_key(|p| p.age)
    );

    for person in people {
        println!(
            "{} {} ({})",
            person.first_name, person.last_name, person.age
        );
    }
}

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn new(first_name: impl Into<String>, last_name: impl Into<String>, age: u8) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: last_name.into(),
            age,
        }
    }
}
