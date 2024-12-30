use crate::fextractor::{get_con, DATABASENAME};
use rusqlite::params;
use crate::Result;
use indicatif::ProgressBar;

#[derive(Debug)]
struct Dependencia {
    id: i32,
    path: String,
    user_interface: String,
}

fn limpa_nome(name: String) -> String {
  // Remover qualquer coisa antes de "unit " e o próprio prefixo "unit "
  if let Some(pos) = name.find("unit ") {
      // Extrair a parte da string após o "unit "
      let result = &name[pos + 5..]; // +5 para pular o "unit "
      
      // Remover o ponto e vírgula, se houver
      let result_trimmed = result.trim_end_matches(';');
      
      result_trimmed.to_string()
  } else {
      name.to_string() // Caso não contenha "unit ", retorna o nome original
  }
}

pub fn prepara(tipo: &str, pb: &mut ProgressBar) -> Result<()> {
  let mut conn = get_con(DATABASENAME).unwrap();
  let transaction = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Deferred)?;
  pb.set_message(format!("Preparando visualização [{}]", tipo));
  // pb.reset_eta();

  pb.reset();


  let ro_connection = get_con("grapho.ro").unwrap();
  let mut stmt = if tipo == "uses interface" {
    ro_connection.prepare("SELECT DISTINCT id, name, uses_interface FROM dependencias where uses_interface is not null")?} 
  else {
    ro_connection.prepare("SELECT DISTINCT id, name, uses_implementation FROM dependencias where uses_implementation is not null")?};
  let dependencias_iter = stmt.query_map([], |row| {
      pb.inc(1);
      Ok(Dependencia {
          id: row.get(0)?,
          path: limpa_nome(row.get(1)?),
          user_interface: row.get(2)?, 
      })
  })?;

  // Iterar sobre os registros e dividir as dependências
  for dependencia in dependencias_iter {
      let dependencia = dependencia?;
      let dependencias_split: Vec<&str> = dependencia.user_interface.split(',').collect();


      // Inserir cada dependência na tabela 'visualiza'
      for dep in dependencias_split {
          let dep_trimmed = dep.trim().to_string().replace(";", "");  
          pb.inc(1);
          transaction.execute(
              "INSERT OR REPLACE INTO visualiza (id, path, uses, tipo) VALUES (?1, ?2, ?3, ?4)",
              params![dependencia.id, dependencia.path, dep_trimmed, tipo],
          )?;
      }
      
  }
  transaction.commit()?;
  println!("Dependências atualizadas com sucesso!");

Ok(()) 

}