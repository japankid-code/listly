@echo off

IF %1.==. GOTO No1

For /f "tokens=2-4 delims=/ " %%a in ('date /t') do (set mydate=%%c-%%a-%%b)
For /f "tokens=1-2 delims=/:" %%a in ('time /t') do (set mytime=%%a%%b)

echo creating migration...
diesel migration generate "%1"
echo %mydate%_%mytime%: migration created.
GOTO End1

:No1
  ECHO Provide migration Name: create-migration My-New-Migration
GOTO End1

:End1
PAUSE