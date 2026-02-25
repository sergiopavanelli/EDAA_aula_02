fn experimento_buscas_multiplas(tamanho: usize, num_buscas: usize) { 
let vetor = gerar_vetor(tamanho); 
// Busca Simples 
let inicio = Instant::now(); 
for _ in 0..num_buscas { 
let _ = busca_sequencial_simples(&vetor, 5); 
    } 
let tempo_simples = inicio.elapsed(); 
// Busca com Interrupção 
let inicio = Instant::now(); 
for _ in 0..num_buscas { 
let _ = busca_sequencial_interrompida(&vetor, 5); 
    } 
let tempo_interrupcao = inicio.elapsed(); 


println!("\n{} buscas em vetor de tamanho {}:", num_buscas, tamanho); 
println!("  Simples: {:?}", tempo_simples); 
println!("  Interrupção: {:?}", tempo_interrupcao);

}