pub fn generate(templates: &[(&[&str], usize)]) -> Vec<String> {
    let mut names = vec![];
    for &(variants, count) in templates.iter() {
        let variants = variants.iter().map(|variant| {
            let i = variant.find('#').unwrap();
            (&variant[0..i], &variant[(i + 1)..])
        }).collect::<Vec<_>>();
        for i in 0..count {
            for &(prefix, suffix) in variants.iter() {
                names.push(format!("{}{}{}", prefix, i, suffix));
            }
        }
    }
    names
}

#[cfg(test)]
mod tests {
    #[test]
    fn generate() {
        let names = super::generate(&[(&["a#b", "c#d"], 2), (&["e#f", "g#h"], 1)]);
        assert_eq!(&names[..], &["a0b", "c0d", "a1b", "c1d", "e0f", "g0h"]);
    }
}
