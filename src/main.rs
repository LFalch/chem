macro_rules! table {
    ($el:ident, $($k:ident; $v:expr,)*) => (
        #[derive(Debug, Copy, Clone)]
        #[repr(u8)]
        enum $el {
            $($k),*
        }

        impl $el {
            fn mass(&self) -> f64 {
                match *self {
                    $($el::$k => $v),*
                }
            }
        }

        impl std::str::FromStr for $el {
            type Err = ();
            fn from_str(s: &str) -> Result<$el, ()> {
                match s {
                    $(stringify!($k) => Ok($el::$k),)*
                    _ => Err(())
                }
            }
        }
    );
}

include!("table.rs");

fn main() {
    let stof = parse_arg(std::env::args().skip(1).collect::<Vec<_>>().join(" "));

    let mol_mass: f64 = stof.iter().map(|&El(n, e)| n as f64 * e.mass()).sum();

    println!("Molar mass: {} g/mol", mol_mass);
    let mellemregning = stof.iter()
        .map(|&El(n, e)| format!("{} * M({:?})", n, e))
        .collect::<Vec<_>>()
        .join(" + ");
    let mellemregning2 = stof.into_iter()
        .map(|El(n, e)| format!("{} * {}", n, e.mass()))
        .collect::<Vec<_>>()
        .join(" + ");

    println!("{} =\n{} = {}", mellemregning, mellemregning2, mol_mass);
}

#[derive(Debug)]
struct El(u8, Element);

fn parse_arg(arg: String) -> Vec<El> {
    let mut stof = Vec::<El>::new();
    let mut cur = String::new();

    for c in arg.chars().chain(Some('\x00')) {
        if c.is_uppercase() && !cur.is_empty() || !c.is_alphanumeric() {
            let num_index = cur.find(<char>::is_numeric).unwrap_or_else(|| cur.len());

            let element = cur[..num_index].parse().unwrap();
            let num;

            if !cur[num_index..].is_empty() {
                num = cur[num_index..].parse().unwrap();
            } else {
                num = 1;
            }

            stof.push(El(num, element));

            cur.clear();
        }
        cur.push(c);
    }
    stof
}
