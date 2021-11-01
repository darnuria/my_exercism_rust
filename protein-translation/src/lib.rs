use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    proteins: HashMap<&'a str, &'a str>,
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.proteins.get(codon).map(|&e| e)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut proteins = Vec::new();
        let codons = rna.as_bytes().chunks(3);
        for r in codons {
            let r = unsafe { std::str::from_utf8_unchecked(r) };
            match r {
                "UAA"| "UAG" | "UGA" => break,
                _ => proteins.push(*self.codons.get(r)?),
            }
        }
        Some(proteins)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let proteins = pairs.iter().map(|(k, v)| (*k, *v)).collect();

    let mut codons = HashMap::new();
    for (code, protein ) in pairs {
        let rnas = code.as_bytes().chunks(3);
        for rna in rnas {
            codons
                .insert(unsafe { std::str::from_utf8_unchecked(rna) }, protein);
        }
    }
    CodonsInfo { proteins, codons }
}
