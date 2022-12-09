@echo off

:: Verify parameters
::===================
if %1A==A goto USAGE

:: Two-digits day with zero padding
set day=0%1
set day=%day:~-2%

:: Get batch files directory and remove suffix day00\rust\mr-kaffee\
set template_dir=%~dp0
set repo_dir=%template_dir:~0,-21%

:: Get year from repo dir
set year=%repo_dir:~-5,4%

set executable=%template_dir%target\release\mr-kaffee-%year%-0.exe

:: check if template executable exists
if exist %executable% goto RUN


:: Build the executable
::======================
:BUILD

echo Building executable %executable% ...

set base_dir=%cd%
cd %template_dir%
if errorlevel 1 (
    echo Could not move to directory %template_dir% to build executable.
    goto END
)

set cmd=cargo build --release
call %cmd%
if errorlevel 1 (
    echo Build failed.
    cd %base_dir%
    goto END
)

cd %base_dir%


:: Run the template 
::==================
:RUN

:: Define command to execute
set target_dir=%repo_dir%day%day%\rust\mr-kaffee\
set runner_dir=%repo_dir%day00\rust\mr-kaffee\
set cmd=%executable% init -r %runner_dir% -t %target_dir% -l ../../../day00/rust/mr-kaffee/aoc -y %year% -d %1

echo Executing '%cmd%' ...

:: Execute command
call %cmd%
if errorlevel 1 (
    echo Failed to execute template.
    goto END
)


:: Test and build newly created files
::====================================
set base_dir=%cd%
cd %target_dir%
if errorlevel 1 (
    echo Could not move to %target_dir% to test & build.
    goto END
)

:: test
set cmd=cargo test
call %cmd%
if errorlevel 1 (
    echo Test failed.
    cd %base_dir%
    goto END
)

:: build
set cmd=cargo build
call %cmd%
if errorlevel 1 (
    echo Build failed.
    cd %base_dir%
    goto END
)

cd %base_dir%


:: code
set cmd=code %target_dir% %target_dir%src\lib.rs %target_dir%input.txt %target_dir%README.adoc
call %cmd%
if errorlevel 1 (
    echo Could not open VSCode
    goto END
)

:: End execution
goto END


:: Print usage message
::=====================
:USAGE

:: Print usage message
echo USAGE: %0 ^<DAY^>

:: End execution
goto END


:: End of file
::=============
:END
