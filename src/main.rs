mod tarefa;
mod armazenamento;

use clap::{Parser, Subcommand};
use tarefa::Tarefa;
use armazenamento::{carregar_tarefas, salvar_tarefas};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adiciona uma nova tarefa
    Add { 
        #[arg(short, long)]
        titulo: String,
    },
    /// Lista todas as tarefas
    List,
    /// Marca uma tarefa como concluída
    Done { 
        #[arg(short, long)]
        id: usize,
    },
}

/// Adiciona uma nova tarefa
fn adicionar_tarefa(titulo: &str) {
    let mut tarefas = carregar_tarefas();
    let nova_tarefa = Tarefa::nova(tarefas.len() + 1, titulo);
    tarefas.push(nova_tarefa);
    salvar_tarefas(&tarefas);
    println!("Tarefa adicionada com sucesso!");
}

/// Lista todas as tarefas
fn listar_tarefas() {
    let tarefas = carregar_tarefas();
    if tarefas.is_empty() {
        println!("Não há tarefas cadastradas.");
    } else {
        for tarefa in tarefas {
            println!("{}", tarefa);
        }
    }
}

/// Marca uma tarefa como concluída
fn marcar_como_concluida(id: usize) {
    let mut tarefas = carregar_tarefas();
    if let Some(tarefa) = tarefas.iter_mut().find(|t| t.id == id) {
        tarefa.marcar_como_concluida();
        salvar_tarefas(&tarefas);
        println!("Tarefa marcada como concluída!");
    } else {
        println!("Tarefa não encontrada.");
    }
}

/// Função principal
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { titulo }) => {
            adicionar_tarefa(titulo);
        }
        Some(Commands::List) => {
            listar_tarefas();
        }
        Some(Commands::Done { id }) => {
            marcar_como_concluida(*id);
        }
        None => {
            println!("Nenhum comando fornecido. Use --help para ver as opções disponíveis.");
        }
    }
}
