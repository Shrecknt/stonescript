# StoneScript

A programming language that compiles to Minecraft datapacks.

Thank you [PyPylia](https://github.com/PyPylia) for making my code less bad.

## The General Idea

 - Tokenizer
   - Parse source files into token streams
 - Parser
   - Convert the token stream into an AST
 - Parser 2nd Pass
   - Handle imports
   - Mangle variable names
   - Inline small functions
   - De-nest nested blocks
 - Pre-Processor
   - Evaluate macros (or don't)
   - Determine the memory usage of functions
   - Insert stack de/allocation instructions
 - Datapack Generator
   - Evaluate imports
   - Generate function and variable names
   - Convert the pre-processed AST into valid mcfunction files
 - Standard Library
    -  A standard library that all programs are dependent on
    - Handles `alloc`, `dealloc`, `read`, and `write` functions

## Goals
 - High level abstractions like structs and enums
 - Compile to several minecraft versions
 - Minecraft version independent abstractions of commands

## Maybe Goals
 - First class functions
 - Compile to native executables
