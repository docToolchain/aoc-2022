@echo off

:: Verify parameters
::===================
if %1A==A goto USAGE
if %2A==A goto USAGE

:: Get batch files directory and remove suffix day00\rust\mr-kaffee\
set template_dir=%~dp0
set repo_dir=%template_dir:~0,-21%

:: Get year from repo dir
set year=%repo_dir:~-5,4%


:: Run the template 
::==================
:RUN

:: Define command to execute
set cmd=cargo run --release -- submit -y %year% -d %1 -p %2

echo Executing '%cmd%' ...

:: Execute command
call %cmd%
if errorlevel 1 (
    echo Failed to submit.
    goto END
)

:: End execution
goto END


:: Print usage message
::=====================
:USAGE

:: Print usage message
echo USAGE: %0 ^<DAY^> ^<PART^>

:: End execution
goto END


:: End of file
::=============
:END
