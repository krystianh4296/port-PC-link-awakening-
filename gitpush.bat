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
    echo Brak zmian do zatwierdzenia lub wystapil blad.
    pause
    exit /b 1
)

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
echo ========================================
echo       ZAKONCZONE POMYSLNIE
echo ========================================

cargo run --release > "log\log-$(Get-Date -Format 'yyyy-MM-dd_HH-mm-ss').txt" 2>&1
pause