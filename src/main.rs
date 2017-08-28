#[derive(Debug, Copy, Clone)]
struct Element {
    mass: f32
}

macro_rules! table {
    ($($k:expr; $v:expr,)*) => (
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($k.to_owned(), Element{mass: $v});
            )*
            map
        }
    );
}

fn main() {
    let table = include!("table.rs");

    let mut stof = Vec::new();
    for c in std::env::args().skip(1).collect::<String>().chars() {
        if c.is_uppercase() {
            stof.push(c.to_string());
        } else if c.is_numeric() {
            let el = stof.last().unwrap().clone();
            for _ in 1..c.to_string().parse::<u8>().unwrap() {
                stof.push(el.clone());
            }
        } else {
            stof.last_mut().unwrap().push(c);
        }
    }

    let mol_mass: f32 = stof.iter().map(|s| table.get(s).unwrap().mass).sum();

    println!("Molar mass: {}", mol_mass);
}
