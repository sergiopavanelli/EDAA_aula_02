/// Busca sequencial simples - sempre percorre todo o vetor
fn busca_sequencial_simples (vetor: &[i32], alvo: i32) -> (Option<usize>, usize)
{
    let mut operacoes = 0;
    let mut resultado = None;
    
    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            resultado = Some(i);
            // Continua procurando mesmo após encontrar
        }
    }
    
    (resultado, operacoes)
}