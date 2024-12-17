pub fn extrair_uses(code: &str, tipo: &str) -> Vec<String> {
    // Cria a regex com base no tipo fornecido (interface ou implementation)
    // Encontra a posição de "interface"

    let code_lower = code.to_lowercase();
    let tipo_lower = tipo.to_lowercase();

    if let Some(interface_pos) = code_lower.find(&tipo_lower) {
        // Encontra a primeira ocorrência de "uses" após "interface" ou "implementation"
        if let Some(uses_pos) = code_lower[interface_pos..].find("uses") {
            // A posição do "uses" após "interface"
            let start_pos = interface_pos + uses_pos + 4;

            // Encontrar o ponto e vírgula para determinar o fim da seção de "uses"
            let result = if let Some(end_pos) = code[start_pos..].find(';') {
                // Captura o bloco entre "uses" e o ponto e vírgula
                let uses_block = &code[start_pos..start_pos + end_pos + 1];
                Some(uses_block.to_string())
            } else {
                None
            };

            if let Some(value) = result {
                return value
                    .split(',')
                    .map(|s| s.trim().to_string()) // Remove espaços em branco e converte para String
                    .collect();
            } else {
                return Vec::new();
            }
        } else {
            return Vec::new();
        }
    } else {
        return Vec::new();
    }
}

pub fn extrair_filename(code: &str) -> String {
    
            // Encontrar o ponto e vírgula para determinar o fim da seção de "uses"
            let result = if let Some(end_pos) = code[0..].find(';') {
                // Captura o bloco entre "uses" e o ponto e vírgula
                let uses_block: &str = &code[0..end_pos + 1];
                Some(uses_block.to_string())
            } else {
                None
            };

            if let Some(value) = result {
                return value
                } else {
                return "".to_string();
        }
}
