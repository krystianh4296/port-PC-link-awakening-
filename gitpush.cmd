@echo off

cd /d "%~dp0"

echo ========================================
echo       Git Auto Commit + Push
echo ========================================
echo.

git add .

git commit -m "Auto commit %date% %time%"

if errorlevel 1 (
    echo.
    echo Brak zmian do zatwierdzenia - uruchamiam emulator mimo to.
    echo.
) else (
    echo.
    echo Commit utworzony pomyslnie.
    echo.
    echo Wysylanie do GitHub - reverse-engineering...

    git push origin reverse-engineering

    if errorlevel 1 (
        echo.
        echo BLAD podczas pushowania!
        pause
        exit /b 1
    )

    echo.
    echo GitHub PUSH OK
)

echo.
echo Uruchamianie emulatora...
echo.

if not exist "log" mkdir "log"

powershell -NoProfile -Command "$date = Get-Date -Format 'yyyy-MM-dd_HH-mm-ss'; cargo run --release *> ('log\log-' + $date + '.txt')"

echo.
echo Emulator zakonczyl dzialanie.
echo.

pause