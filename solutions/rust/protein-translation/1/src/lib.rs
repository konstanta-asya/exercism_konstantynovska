pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let codon_map = [
        ("AUG", "Methionine"),
        ("UUU", "Phenylalanine"),
        ("UUC", "Phenylalanine"),
        ("UUA", "Leucine"),
        ("UUG", "Leucine"),
        ("UCU", "Serine"),
        ("UCC", "Serine"),
        ("UCA", "Serine"),
        ("UCG", "Serine"),
        ("UAU", "Tyrosine"),
        ("UAC", "Tyrosine"),
        ("UGU", "Cysteine"),
        ("UGC", "Cysteine"),
        ("UGG", "Tryptophan"),
        ("UAA", "STOP"),
        ("UAG", "STOP"),
        ("UGA", "STOP"),
    ];

    let mut proteins = Vec::new();

    for codon_bytes in rna.as_bytes().chunks(3) {
        if codon_bytes.len() != 3 {
            return None;
        }

        let codon = std::str::from_utf8(codon_bytes).ok()?;

        match codon_map.iter().find(|&&(c, _)| c == codon) {
            Some(&(_, "STOP")) => break,
            Some(&(_, amino_acid)) => proteins.push(amino_acid),
            None => return None,
        }
    }

    Some(proteins)
}
