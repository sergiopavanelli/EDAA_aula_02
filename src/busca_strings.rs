fn busca_strings(vetor: &[String], alvo: &str) -> Option<usize> {
    for (i, item) in vetor.iter().enumerate() {
        if item == alvo {
            return Some(i);
        }
    }
    None
}