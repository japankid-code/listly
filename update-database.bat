@echo off

For /f "tokens=2-4 delims=/ " %%a in ('date /t') do (set mydate=%%c-%%a-%%b)
For /f "tokens=1-2 delims=/:" %%a in ('time /t') do (set mytime=%%a%%b)

echo Choose an option:
echo 1. diesel migration run
echo 2. diesel migration redo

set /p choice=Type 1 or 2, then press Enter: 
if %choice%==1 (
  echo applying migration...
  diesel migration run
) else if %choice%==2 (
  echo redoing migration...
  diesel migration redo
) else (
  echo Invalid choice. Exiting.
  GOTO End1
)

echo %mydate%_%mytime%: migration completed.
GOTO End1

:End1
PAUSE