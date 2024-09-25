use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tarefa {
    pub id: usize,
    pub titulo: String,
    pub concluida: bool,
    pub criada_em: DateTime<Utc>,
}

/// Cria uma nova tarefa
impl Tarefa {
    pub fn nova(id: usize, titulo: &str) -> Self {
        Self {
            id,
            titulo: titulo.to_string(),
            concluida: false,
            criada_em: Utc::now(),
        }
    }

    /// Marca uma tarefa como concluída
    pub fn marcar_como_concluida(&mut self) {
        self.concluida = true;
    }
}

/// Formata a tarefa para exibição
impl std::fmt::Display for Tarefa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} - {} (Criada em: {})",
            if self.concluida { "X" } else { " " },
            self.id,
            self.titulo,
            self.criada_em.format("%d/%m/%Y %H:%M")
        )
    }
}
