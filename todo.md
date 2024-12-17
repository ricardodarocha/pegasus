- [x] Ler um arquivo com a extensão .pas
- [x] Ler uma pasta, e todo o conteúdo desta pasta
- [x] Ler o conteúdo de subpastas
- [x] Varrer o conteúdo de um arquivo .pas e extrair o uses interface e o uses implementation

```
UPDATE visualiza SET escopo = (SELECT escopo FROM dependencias WHERE "path" LIKE '%' || visualiza."path" || '%')
WHERE EXISTS (SELECT escopo FROM dependencias WHERE "path" LIKE '%' || visualiza."path" || '%')
```