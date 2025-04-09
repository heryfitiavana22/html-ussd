@echo off
setlocal

set VERSION=v1.0.0
set REPO=heryfitiavana22/html-ussd
set URL=https://github.com/%REPO%/releases/download/%VERSION%/html-ussd-windows.zip
set INSTALLATION_DIR=%LOCALAPPDATA%\Programs\html-ussd\bin

echo 📦 Downloading %URL%...
powershell -Command "Invoke-WebRequest -Uri '%URL%' -OutFile 'html-ussd.zip'"

echo 📂 Extracting...
powershell -Command "Expand-Archive -Path 'html-ussd.zip' -DestinationPath 'html-ussd-bin'"

echo 🚀 Installing to %%INSTALLATION_DIR%%
if not exist "%INSTALLATION_DIR%" mkdir "%INSTALLATION_DIR%"
copy /Y html-ussd-bin\html-ussd.exe "%INSTALLATION_DIR%\html-ussd.exe"

echo %PATH% | find /I "%INSTALLATION_DIR%" >nul
if %ERRORLEVEL% neq 0 (
    setx PATH "%PATH%;%INSTALLATION_DIR%"
)

echo ✅ html-ussd installed.
echo 🔄 Please restart your terminal or open a new one to use html-ussd.

del html-ussd.zip
rd /s /q html-ussd-bin

endlocal
