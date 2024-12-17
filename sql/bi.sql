SELECT DISTINCT path, uses_interface, escopo FROM dependencias;

SELECT path||'-->'|| uses FROM visualiza v
WHERE escopo = 'cliente' AND tipo = 'uses implementation'
AND uses > '' AND NOT uses LIKE '%{%'

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


-- Contagem de dependentes e dependências por número de dependentes
SELECT 
    "path",
    COUNT(DISTINCT uses) AS numero_de_depencias,  
    (SELECT COUNT(*) 
     FROM visualiza AS d2 
     WHERE d2.uses = v."path") AS numero_de_dependentes 
FROM visualiza v 
GROUP BY  "path"
ORDER BY 3 desc, 2 desc;

-- +-------------------------------------------------------------------------------+
-- | por número de dependentes                                                     |
-- +-------------------------------------------------------------------------------+
-- | Nome do Fonte                | Número de Dependências | Número de Dependentes |
-- +------------------------------+------------------------+-----------------------+
-- | uspjFuncoesServidor          | 54                     | 76                    |
-- | uspjDBase                    | 992                    | 56                    |
-- | uspjComum.Attributes         | 3                      | 50                    |
-- | uspjIntegracaoCliente        | 2                      | 50                    |
-- | uspjComum.ValidateAttributes | 7                      | 36                    |
-- | uspjConectarNoServidor       | 23                     | 32                    |
