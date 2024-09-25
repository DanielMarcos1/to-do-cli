use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::tarefa::Tarefa;

/// Carrega as tarefas do arquivo JSON  
pub fn carregar_tarefas() -> Vec<Tarefa> {
    let path = Path::new("tarefas.json");
    if path.exists() {
        let mut file = File::open(path).expect("Não foi possível abrir o arquivo");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Não foi possível ler o arquivo");
        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

/// Salva as tarefas no arquivo JSON
pub fn salvar_tarefas(tarefas: &Vec<Tarefa>) {
    let serialized = serde_json::to_string_pretty(tarefas).expect("Não foi possível serializar as tarefas");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tarefas.json")
        .expect("Não foi possível criar o arquivo");
    file.write_all(serialized.as_bytes()).expect("Não foi possível escrever no arquivo");
}
