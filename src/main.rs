fn sapa(nama: &str) -> String {
    format!("Hello, {}! 🦀", nama)
}

fn main() {
    println!("🦀 Hello World dari Rust!");
    println!("Belajar Rust bareng AI — let's go!");
    println!("{}", sapa("Audyari"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sapa_world() {
        assert_eq!(sapa("World"), "Hello, World! 🦀");
    }

    #[test]
    fn test_sapa_nama() {
        assert_eq!(sapa("Audyari"), "Hello, Audyari! 🦀");
    }

    #[test]
    fn test_sapa_mengandung_nama() {
        assert!(sapa("Budi").contains("Budi")); // bisa jelaskan ini apa
    }
}
