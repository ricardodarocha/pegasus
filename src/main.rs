#![doc=include_str!("../readme.md")]
pub mod fencoding;
pub mod fextract;
pub mod fextractor;
pub mod fmonitor;
pub mod fprocessor;
pub mod fvisualization;
use std::error::Error as StdError;

pub type Result<T, E = Box<dyn StdError>> = std::result::Result<T, E>;

pub const LIMIT: u32 = 9000;

use clap::{Parser, Subcommand};
use fextractor::{get_con, migrate};
use fmonitor::monitore;
use fprocessor::processa_pasta;
use fvisualization::prepara;
use rusqlite::params;
use crate::fextractor::DATABASENAME;

/// 🪽 PEGASUS 1.0 (peg.exe)
/// .pas Explorer, Grapho Analyse Uses
#[derive(Parser)]
#[command(name = "📗 Comandos")]
#[command(about = "Explora arquivos .pas e analisa as dependências uses")]
struct Args {
    /// Subcomando escolhido pelo usuário
    #[command(subcommand)]
    command: Commands,
}

/// Subcomandos disponíveis
#[derive(Subcommand)]
pub enum Commands {
    /// Explora um diretório com base nos parâmetros informados
    Explorar {
        /// Caminho do diretório a ser analisado
        #[arg(short, long)]
        caminho: Option<String>,

        /// Limite de arquivos a analisar (0 significa sem limite)
        #[arg(short, long, default_value_t = 9000)]
        deep: u32,
    },
    
    /// Gera visualizações com base nos arquivos fornecidos
    Visualizar {
        /// Lista de arquivos para visualização
        // #[arg(short, long)]
        arquivos: Vec<String>,

        /// Limite de arquivos a analisar (0 significa sem limite)
        #[arg(short, long, default_value_t = 50)]
        limit: u32,

        /// Limite de arquivos a analisar (0 significa sem limite)
        #[arg(short, long, default_value_t = 1)]
        page: u32,
    },
    
    Limpar,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, )]
pub struct Node {
    pub value: String,
}



use std::io::{self, Write};
use std::fs::File;


impl Args {
    /// Método que chama a função explore
    fn explore(caminho: String, deep: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut count: u32 = 0;
        dbg!(deep);

        // Chama a função para processar o diretório
        monitore(&"🗂️  explorando... ", || {
    
            let _ = processa_pasta(&caminho, &mut count, deep);
        });

        // Monta as visualizações de uses interface
        monitore(&"uses interface", || {
            match prepara("uses interface") {
                Ok(_) => {},
                Err(err) => println!("{err}"),
            };
        });

    // Monta as visualizações de uses implementation
    monitore(&"uses implementation", || {
        match prepara("uses implementation") {
            Ok(_) => {},
            Err(err) => println!("{err}"),
        };
    });

    println!("Finalizado");
    Ok(())
    }


    /// Método que chama a função visualiza
    fn visualiza(arquivos: Vec<String>, limit: u32, page: u32) -> Result<(), Box<dyn std::error::Error>> {
        // let conn = get_con(DATABASENAME).unwrap();
        let mut conn = get_con(DATABASENAME).unwrap();
        let transaction = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)?;

        //Primeiro cria uma lista de filtros
        for arquivo in arquivos.into_iter() {
            transaction.execute(
                "INSERT OR REPLACE INTO filtro (expressao, ativo) VALUES (
                ?1,
                True
                )",
                params!(arquivo),
            ).unwrap();
        };
        transaction.commit().unwrap();

        let conn: rusqlite::Connection = get_con(DATABASENAME).unwrap();

        let mut stmt = conn.prepare(r#"
            SELECT path||'-->'|| uses  AS "node" 
            FROM visualiza v
            JOIN filtro f  ON  ( upper("path") LIKE '%' || upper(f."expressao") || '%' 
                           OR    upper(uses) LIKE '%' || upper(f."expressao") || '%') 
WHERE  uses > '' AND NOT uses LIKE '%\{%' limit $1 offset $2;
        "#)?;
        let node_iter = stmt.query_map([limit, (page-1) * limit], |row| {
            Ok(Node {
                value: row.get(0)?
            })
        })?;

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        std::fs::create_dir_all("Grapho").unwrap();
        let file = File::create("Grapho/output.mmd")?;
        let mut file_writer = io::BufWriter::new(file);


        writeln!(handle, "stateDiagram-v2")?;
        writeln!(file_writer, "stateDiagram-v2")?;


        
        for node in node_iter {
            let value = node.unwrap().value;
            writeln!(handle, "{value}" )?;
            writeln!(file_writer, "{value}" )?;
        }
        handle.flush()?;
        file_writer.flush()?;

        
        Ok(())
    }

    fn limpar()-> Result<(), Box<dyn std::error::Error>> {
        let mut conn = get_con(DATABASENAME).unwrap();
        let transaction = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)?;

        transaction.execute(
            "UPDATE FILTRO SET ATIVO = FALSE     ",
            params![],
            
        )?;
        transaction.execute(
            "DELETE FROM VISUALIZA ",
            params![],
        )?;
        transaction.execute(
            "DELETE FROM DEPENDENCIAS ",
            params![],
        )?;

        let _ = transaction.commit().unwrap();

        Ok(())

    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("%% PEGASUS 1.0 🪽  | peg.exe ");
    let _ = migrate(get_con(&DATABASENAME).unwrap());

    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();


    match args.command {
        Commands::Explorar { caminho, deep } => {
            let caminho =         match caminho {
                Some(value) => value,   
                _ => std::env::current_dir()?.to_str().unwrap().to_string() // Diretório atual
            };
            Args::explore(caminho, deep)
        },
        Commands::Visualizar { arquivos, limit, page } => {
            Args::visualiza(arquivos, limit, page)
        },
        Commands::Limpar => {
            Args::limpar()
        }, 
    }

    
}
