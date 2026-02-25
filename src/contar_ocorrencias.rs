fn contar_ocorrencias(vetor: &[i32], alvo: i32) -> usize {
    let mut contador = 0;
    for &item in vetor {
        if item == alvo {
            contador += 1;
        }
    }
    contador
}