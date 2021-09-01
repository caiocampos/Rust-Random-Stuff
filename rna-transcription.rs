#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: String,
}

impl DNA {
    fn create(nucleotides: String) -> Self {
        DNA {
            nucleotides: nucleotides,
        }
    }

    pub fn new(dna: &str) -> Result<DNA, usize> {
        match dna
            .chars()
            .position(|c| c != 'G' && c != 'C' && c != 'T' && c != 'A')
        {
            Some(pos) => Err(pos),
            None => Ok(DNA::create(dna.to_owned())),
        }
    }

    pub fn into_rna(self) -> RNA {
        let rna: String = self
            .nucleotides
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                _ => 'U',
            })
            .collect();
        RNA::new(rna.as_ref()).unwrap()
    }
}

impl RNA {
    fn create(nucleotides: String) -> Self {
        RNA {
            nucleotides: nucleotides,
        }
    }

    pub fn new(rna: &str) -> Result<RNA, usize> {
        match rna
            .chars()
            .position(|c| c != 'C' && c != 'G' && c != 'A' && c != 'U')
        {
            Some(pos) => Err(pos),
            None => Ok(RNA::create(rna.to_owned())),
        }
    }
}
