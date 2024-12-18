![Alt text](pegasus.png)

PEGASUS 1.0 🪽   | peg.exe 

Explora arquivos pas e analisa as dependências uses  

Usage: peg.exe <COMMAND>  

Commands:   
- `[explorar]`    Explora um diretório com base nos parâmetros informados   
- `[visualizar]`  Exporta visualizações com base nos nomes de arquivos fornecidos   
- `[limpar]`   Remove todas as análises anteriores

##   
`> peg.exe explorar`
PEGASUS 1.0 🪽  | peg.exe

Explora um diretório com base nos parâmetros informados  

[OPÇÕES]  
-c, --caminho <CAMINHO>  Caminho do diretório a ser analisado   
-d, --deep <DEEP>        Limite de arquivos a analisar (0 significa sem limite) [default: 0]   
-h, --help 

[EXEMPLO]     
`> peg.exe explorar "C:/git/pge-net"`          

## 
`> peg.exe visualizar`
PEGASUS 1.0 🪽  | peg.exe  

Exporta visualizações com base nos arquivos fornecidos  
[ARQUIVOS]...  [Array] Lista de arquivos para visualização  
-l, --limit <LIMIT>      Paginação [default: 50]  
-p, --page <PAGE>        Página [default: 1]   

[EXEMPLO]     
> `> peg.exe visualizar json AndamentosInternos AtosEletronicos --limit 50 `  
> Lista 50 primeiros fontes e suas dependências em um destes contextos (json, andamentosInternos, Atoseletronicos) paginado de 50 em 50 

> `> peg.exe visualizar json AndamentosInternos AtosEletronicos -p 2 `  
> Traz a segunda página

## 
`> peg.exe limpar`       
PEGASUS 1.0 🪽  | peg.exe  

Remove todas as análises anteriores

## Acessando o banco de dados

Conectar o arquivo grapho.db no Dbeaver, escolha a configuração SQLite

```sql
-- Contagem de dependentes e dependências por número de dependências
SELECT 
    "path",
    COUNT(DISTINCT uses) AS numero_de_depencias,  
    (SELECT COUNT(*) 
     FROM visualiza AS d2 
     WHERE d2.uses = v."path") AS numero_de_dependentes 
FROM visualiza v 
GROUP BY  "path"
ORDER BY 2 desc, 3 desc;
```
```markdown
-- +-----------------------------------------------------------------------------------------------------+
-- | por número de dependências                                                                          |
-- +-----------------------------------------------------------------------------------------------------+
-- | Nome do Fonte                                      | Número de Dependências | Número de Dependentes |
-- +----------------------------------------------------+------------------------+-----------------------+
-- | uspjDBase                                          | 992                    | 56                    |
-- | uspjDocumentos.DlgFinalizar                        | 303                    | 2                     |
-- | uspjProcessoServ                                   | 287                    | 18                    |
-- | uspjDocumentos.EdicaoDocumento                     | 274                    | 1                     |
-- | uspjVisualizaFluxoTrabalho                         | 240                    | 4                     |
-- | uspjAtividadeIndependenteCliente                   | 239                    | 6                     |
-- | uspjGestaoProcessual.ConsPendenciaPrazoRPDevBasica | 222                    | 3                     |
-- | uspjPastaDigital.VisualizaProcesso                 | 222                    | 2                     |
-- | uspjTelaMenu                                       | 209                    | 14                    |
-- | uspjGestaoProcessual.CadProcessoMovBase            | 205                    | 2                     |
-- | uspjPastaDigital.DigPecaProcessual                 | 199                    | 1                     |
``` 

```sql
--Configure os filtros 
update filtro set ativo = False;

INSERT INTO filtro (expressao) VALUES 
('json'), 
('form') ;

SELECT path||'-->'|| uses  AS "node: String" FROM visualiza v
JOIN filtro f  ON  ( upper("path") LIKE '%' || upper(f."expressao") || '%' OR upper(uses) LIKE '%' || upper(f."expressao") || '%') 
WHERE  uses > '' AND NOT uses LIKE '%{%' and f.ativo = True;
```





