/// Busca sequencial com interrupção antecipada
fn busca_sequencial_interrompida(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0;
    
    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            return (Some(i), operacoes);
        }
    }
    
    (None, operacoes)
}