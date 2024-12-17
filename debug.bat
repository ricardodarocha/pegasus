@REM Gera a versão debug e copia para pasta bin
cargo build --debug

@echo off
:: Caminho de origem do arquivo (assumindo que o arquivo está na pasta atual)
set origem=%cd%\target\debug\peg.exe

:: Caminho de destino, três níveis abaixo de target/doc/peg/
set destino=%cd%\bin

:: Cria os diretórios de destino, se não existirem
mkdir "%destino%" 2>nul

:: Copia o arquivo para o destino
copy "%origem%" "%destino%\bin.exe"

:: Mensagem de conclusão
echo -----------------------------------------------------------------
echo  debug gerada. Iniciando "%destino%\bin.exe"
echo -----------------------------------------------------------------

"%destino%\bin.exe"