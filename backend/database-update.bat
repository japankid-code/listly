@echo off

IF "%1"=="" (
  echo Choose an option:
  echo 1. run
  echo 2. redo
  echo 3. revert
  set /p choice=Type 1, 2 or 3, then press Enter: 
) ELSE (
  set choice=%1
)

set mydate=%date:~10,4%-%date:~4,2%-%date:~7,2%
set mytime=%time:~0,2%%time:~3,2%

IF "%choice%"=="1" (
  echo applying migration...
  diesel migration run
) ELSE IF "%choice%"=="2" (
  echo redoing migration...
  diesel migration redo
) ELSE IF "%choice%"=="3" (
  echo reverting migration...
  diesel migration revert
) ELSE (
  echo Invalid choice. Exiting.
  exit /b 1
)

echo %mydate%_%mytime%: migration completed.