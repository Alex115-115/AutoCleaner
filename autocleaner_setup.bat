@echo off
:: Build the xtask project
cargo build -p xtask --release

:: Get absolute path to xtask.exe
set "XTASK_PATH=%cd%\target\release\xtask.exe"

:: Run xtask.exe as administrator
powershell -NoProfile -Command ^
    "Start-Process -FilePath '%XTASK_PATH%' -ArgumentList 'install' -Verb RunAs"
