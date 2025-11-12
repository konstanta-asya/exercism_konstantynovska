#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (idx, ch) in dna.chars().enumerate() {
            if !"ACGT".contains(ch) {
                return Err(idx);
            }
        }
        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        let rna_str: String = self.0.chars().map(|ch| match ch {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => unreachable!("We already validated DNA"),
        }).collect();

        Rna(rna_str)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (idx, ch) in rna.chars().enumerate() {
            if !"ACGU".contains(ch) {
                return Err(idx);
            }
        }
        Ok(Rna(rna.to_string()))
    }
}
