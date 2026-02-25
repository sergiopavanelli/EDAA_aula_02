use std::time::Instant; 
/// Busca sequencial simples - sempre percorre todo o vetor
fn busca_sequencial_simples(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) 
{ 
let mut operacoes = 0; 
let mut resultado = None; 
for i in 0..vetor.len() { 
        operacoes += 1; 
if vetor[i] == alvo { 
            resultado = Some(i); 
        } 
    } 
    (resultado, operacoes) 
} 
/// Busca sequencial com interrupção antecipada
fn busca_sequencial_interrompida(vetor: &[i32], alvo: i32) -> (Option<usize>, 
usize) { 
let mut operacoes = 0; 

for i in 0..vetor.len() { 
        operacoes += 1; 
if vetor[i] == alvo { 
return (Some(i), operacoes); 
        } 
    } 
    (None, operacoes) 
} 
/// Gera um vetor com valores de 1 até n
fn gerar_vetor(tamanho: usize) -> Vec<i32> { 
    (1..=tamanho as i32).collect() 
}

// Exercício 1: Busca em Strings [cite: 226, 227]
fn busca_strings(vetor: &[String], alvo: &str) -> Option<usize> {
    for (i, item) in vetor.iter().enumerate() {
        if item == alvo {
            return Some(i);
        }
    }
    None
}

// Exercício 2: Contar ocorrências [cite: 228, 229]
fn contar_ocorrencias(vetor: &[i32], alvo: i32) -> usize {
    let mut contador = 0;
    for &item in vetor {
        if item == alvo {
            contador += 1;
        }
    }
    contador
}

// Exercício 4: Retornar todas as posições [cite: 232, 233]
fn buscar_todas_posicoes(vetor: &[i32], alvo: i32) -> Vec<usize> {
    let mut posicoes = Vec::new();
    for (i, &item) in vetor.iter().enumerate() {
        if item == alvo {
            posicoes.push(i);
        }
    }
    posicoes
}
/// Executa experimento comparativo entre os dois algoritmos
fn executar_experimento(tamanho: usize, alvo: i32) { 
let vetor = gerar_vetor(tamanho); 
println!("\n{}", "=".repeat(60)); 
println!("Tamanho do vetor: {}", tamanho); 
println!("Elemento procurado: {}", alvo); 
println!("{}", "-".repeat(60)); 
// Busca Sequencial Simples 
let inicio = Instant::now(); 
let (resultado1, ops1) = busca_sequencial_simples(&vetor, alvo); 
let tempo1 = inicio.elapsed(); 
println!("\n📌 BUSCA SEQUENCIAL SIMPLES:"); 
println!("   Resultado: {:?}", resultado1); 
println!("   Operações: {}", ops1); 
println!("   Tempo: {:?}", tempo1); 
// Busca Sequencial com Interrupção 
let inicio = Instant::now(); 
let (resultado2, ops2) = busca_sequencial_interrompida(&vetor, alvo); 
let tempo2 = inicio.elapsed(); 
println!("\n📌 BUSCA SEQUENCIAL COM INTERRUPÇÃO:"); 
println!("   Resultado: {:?}", resultado2); 
println!("   Operações: {}", ops2); 
println!("   Tempo: {:?}", tempo2); 
// Análise Comparativa 
println!("\n📊 ANÁLISE COMPARATIVA:"); 
println!("   Diferença de operações: {} operações", 
ops1.saturating_sub(ops2)); 
if tempo1 > tempo2 { 
println!("   Algoritmo com interrupção foi mais rápido"); 
    } 
else if tempo2 > tempo1 { 
println!("   Algoritmo simples foi mais rápido (provavelmente devido à variação)"); 
    } 
else { 
println!("   Tempos praticamente iguais"); 
    } 
println!("{}", "=".repeat(60)); 
} 
/// Realiza múltiplas buscas para comparar o tempo total (Experimento 7.1) 
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

    println!("\n🚀 {} buscas em vetor de tamanho {}:", num_buscas, tamanho); 
    println!("  Simples: {:?}", tempo_simples); 
    println!("  Interrupção: {:?}", tempo_interrupcao);
}
fn main() { 
println!("\n🔬 EXPERIMENTO: COMPARAÇÃO DE ALGORITMOS DE BUSCA\n"); 
// Cenário 1: Elemento no início do vetor 
println!("\n🎯 CENÁRIO 1: Elemento no início (melhor caso para 
interrupção)"); 
    executar_experimento(1_000, 5); 
    executar_experimento(100_000, 5); 
    executar_experimento(1_000_000, 5); 
// Cenário 2: Elemento no meio do vetor 
println!("\n\n🎯 CENÁRIO 2: Elemento no meio do vetor"); 
    executar_experimento(1_000, 500); 
    executar_experimento(100_000, 50_000); 
    executar_experimento(1_000_000, 500_000); 
// Cenário 3: Elemento no final do vetor (pior caso) 
println!("\n\n🎯 CENÁRIO 3: Elemento no final (pior caso)"); 
    executar_experimento(1_000, 1_000); 
    executar_experimento(100_000, 100_000); 
    executar_experimento(1_000_000, 1_000_000); 
// Cenário 4: Elemento não existe 
println!("\n\n🎯 CENÁRIO 4: Elemento não existe no vetor"); 
    executar_experimento(1_000, -1); 
    executar_experimento(100_000, -1); 
    executar_experimento(1_000_000, -1);

// --- TESTES DOS EXERCÍCIOS PROPOSTOS ---
println!("\n🚀 TESTANDO EXERCÍCIOS PROPOSTOS");

// Teste Exercício 1: Strings 
let frutas = vec!["Maçã".to_string(), "Banana".to_string(), "Uva".to_string()];
println!("\n🍎 Ex 1 (Busca String):");
println!("   Procurando 'Banana': {:?}", busca_strings(&frutas, "Banana"));

// Teste Exercício 2: Contar Ocorrências 
let lista_repetida = vec![1, 2, 3, 2, 4, 2, 5];
println!("\n🔢 Ex 2 (Contar Ocorrências):");
println!("   O número 2 aparece {} vezes", contar_ocorrencias(&lista_repetida, 2));

// Teste Exercício 4: Todas as Posições 
println!("\n📍 Ex 4 (Todas as Posições):");
println!("   Posições do número 2: {:?}", buscar_todas_posicoes(&lista_repetida, 2));

experimento_buscas_multiplas(100_000, 1_000);

println!("\n\n✅ Experimento concluído!\n"); 
}