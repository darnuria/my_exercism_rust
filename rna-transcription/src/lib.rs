#[derive(Debug, PartialEq)]
enum DnaBase {
    A=0,T,C,G
}

#[derive(Debug, PartialEq)]
enum RnaBase {
    U=0,A,G,C
}

#[derive(Debug, PartialEq)]
pub struct Dna {
    buf: Vec<DnaBase>,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    buf: Vec<RnaBase>
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut buf = Vec::with_capacity(dna.len());
        for (i, n) in dna.chars().enumerate() {
            let base = match n {
                'A' => DnaBase::A,
                'T' => DnaBase::T,
                'C' => DnaBase::C,
                'G' => DnaBase::G,
                _ => return Err(i),
            };
            buf.push(base)
        }
        Ok(Dna { buf })
    }

    pub fn into_rna(self) -> Rna {
        let buf = self.buf.iter().map(|e| 
            {
                use DnaBase::*;
                match e {
                    A => RnaBase::U,
                    T => RnaBase::A,
                    C => RnaBase::G,
                    G => RnaBase::C,
                }     
        }).collect();
        Rna { buf }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut buf = Vec::with_capacity(rna.len());
        for (i, n) in rna.chars().enumerate() {
            let base = match n {
                'U' => RnaBase::U,
                'A' => RnaBase::A,
                'G' => RnaBase::G,
                'C' => RnaBase::C,
                _ => return Err(i),
            };
            buf.push(base)
        }
        Ok(Rna { buf })
    }
}
