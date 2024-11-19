use ltnt::Parsable;

ltnt::ltnt! {
    enum Root {
        ~End,

        A = A,
        B = B,
        C = C,
    };

    enum A {
        And = Box<Root>,
    };

    enum B {
        And = Box<Root>,
    };

    enum C {
        And = Box<Root>,
    };
}

fn main() {
    let (mut cmd, _) = Root::parse().unwrap();

    while !matches!(cmd, Root::End) {
        match cmd {
            Root::A(A::And(next)) => {
                println!("A");
                cmd = *next;
            }
            Root::B(B::And(next)) => {
                println!("B");
                cmd = *next;
            }
            Root::C(C::And(next)) => {
                println!("C");
                cmd = *next;
            }
            _ => unreachable!(),
        }
    }
    println!("End");
}
