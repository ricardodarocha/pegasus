use crate::StdError;
use crate::{fencoding::detect_and_convert_to_utf8, fextract::extrair_uses, fextract::extrair_filename};

    #[derive(Debug)]
    pub struct Profile {
        pub filepath: String,
        pub filename: String,
        pub interf: Vec<String>,
        pub implemen: Vec<String>, 
    }

    impl Profile {
        pub fn save(self) {
            let con = get_con(&DATABASENAME).unwrap();
            con.execute(
                "INSERT OR REPLACE INTO dependencias (name, path, uses_interface, uses_implementation) VALUES (
                ?1,
                ?2,
                ?3,
                ?4
                )",
                (self.filename, self.filepath.replace("\\", &"/"), self.interf.join(", "), self.implemen.join(", ")),
            ).unwrap();
        }
    }
    
use rusqlite::{Connection, Error as Er, Result as Res};

    pub fn migrate(con: Connection) -> Res<(), Er> {
    con.execute(
        "CREATE TABLE IF NOT EXISTS dependencias (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            uses_interface TEXT NOT NULL,
            uses_implementation TEXT NOT NULL,
            escopo VARCHAR NOT NULL DEFAULT 'todos',
            UNIQUE (path, name)
        )",
        [],
    )?;

    con.execute("CREATE TABLE IF NOT EXISTS visualiza (
        path TEXT NOT NULL,
        uses TEXT NOT NULL,
        tipo TEXT NOT NULL DEFAULT 'uses interface',
        id INTEGER NOT NULL references dependencias(id),
        PRIMARY KEY (path, uses)  -- Usa path e uses como chave primária para garantir unicidade
    )",
        [],
    )?;

    Ok(())
    }    
    pub const DATABASENAME: &'static str = "grapho.db";
    pub fn get_con(databasename: &str) -> Res<Connection, Er> {
        if databasename != DATABASENAME {
            let _ = std::fs::copy(DATABASENAME, databasename).unwrap();

        };
        let conn = Connection::open(databasename)?;
        Ok(conn)
    }

    pub fn search(filepath: &str) -> Result<Profile, Box<dyn StdError>> {

        let conteudo_arquivo = detect_and_convert_to_utf8(filepath)?;

        // println!("conteudo: {}", &conteudo_arquivo);
        let filename = extrair_filename(&conteudo_arquivo);
        let interf = extrair_uses(&conteudo_arquivo, "interface");
        let implemen = extrair_uses(&conteudo_arquivo, "implementation");
        let filepath = filepath.to_string();
                
        Ok(Profile {filepath, filename, interf, implemen }) 
        
        }