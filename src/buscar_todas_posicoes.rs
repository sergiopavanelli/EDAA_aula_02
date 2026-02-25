fn buscar_todas_posicoes(vetor: &[i32], alvo: i32) -> Vec<usize> {
    let mut posicoes = Vec::new();
    for (i, &item) in vetor.iter().enumerate() {
        if item == alvo {
            posicoes.push(i);
        }
    }
    posicoes
}