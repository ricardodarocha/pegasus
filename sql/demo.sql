-- grapho do fonte uspjFuncoesServidor
SELECT path||'-->'|| uses FROM visualiza v
WHERE ( "path" = 'uspEstiloDxGrid') 
AND uses > '' AND NOT uses LIKE '%{%'

-- grapho de fontes que dependente de uspjFuncoesServidor
SELECT "path"||'-->'|| uses FROM visualiza v
WHERE uses = 'uspEstiloDxGrid' 
AND uses > '' AND NOT uses LIKE '%{%'

SELECT DISTINCT f.PATH, p.*
FROM dependencias p
JOIN visualiza f ON p.name LIKE '%' || f."path" || '%'


--Lista de todos os fontes que atendem a um determinado filtro
SELECT DISTINCT f.expressao, p.*
FROM dependencias p
JOIN filtro f ON (p.name LIKE '%' || f."expressao" || '%') AND f.ativo;


--Configure os filstros 
INSERT INTO filtro (expressao) VALUES 
('json'), 
('form') ;

SELECT path||'-->'|| uses  AS "node: String" FROM visualiza v
JOIN filtro f  ON  ( upper("path") LIKE '%' || upper(f."expressao") || '%' OR upper(uses) LIKE '%' || upper(f."expressao") || '%') 
WHERE  uses > '' AND NOT uses LIKE '%{%';




