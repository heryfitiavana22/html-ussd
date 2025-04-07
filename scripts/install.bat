@echo off
setlocal

set VERSION=v0.1.0
set REPO=heryfitiavana22/html-ussd
set URL=https://github.com/%REPO%/releases/download/%VERSION%/html-ussd-windows.zip
set INSTALLATION_DIR=%USERPROFILE%\.cargo\bin

echo 📦 Downloading %URL%...
powershell -Command "Invoke-WebRequest -Uri '%URL%' -OutFile 'html-ussd.zip'"

echo 📂 Extracting...
powershell -Command "Expand-Archive -Path 'html-ussd.zip' -DestinationPath 'html-ussd-bin'"

echo 🚀 Installing to %%USERPROFILE%%\.cargo\bin
if not exist "%INSTALLATION_DIR%" mkdir "%INSTALLATION_DIR%"
copy html-ussd-bin\html-ussd.exe "%USERPROFILE%\.cargo\bin\html-ussd.exe"

echo %PATH% | find /I "%INSTALLATION_DIR%" >nul
if %ERRORLEVEL% neq 0 (
    setx PATH "%PATH%;%INSTALLATION_DIR%"
)

echo ✅ html-ussd installed.
echo 🔄 Please restart your terminal or open a new one to use html-ussd.

del html-ussd.zip
rd /s /q html-ussd-bin

endlocal
