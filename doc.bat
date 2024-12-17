cargo doc  --open --no-deps

@echo off
:: Caminho de origem do arquivo (assumindo que o arquivo está na pasta atual)
set origem=%cd%\pegasus.png

:: Caminho de destino, três níveis abaixo de target/doc/peg/
set destino=%cd%\target\doc\peg\

:: Cria os diretórios de destino, se não existirem
mkdir "%destino%" 2>nul

:: Copia o arquivo para o destino
copy "%origem%" "%destino%\pegasus.png"

:: Mensagem de conclusão
echo Documentacao atualizada %destino%
pause