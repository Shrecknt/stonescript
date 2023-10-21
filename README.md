# StoneScript

A programming language that compiles to Minecraft datapacks

Thank you [PyPylia](https://github.com/PyPylia) for making my code less bad

## The General Idea

 - Tokenizer
 - - Parse source files into token streams
 - Parser
 - - Convert the token stream into an AST
 - Pre-Processor
 - - Inline small functions
 - - Evaluate macros
 - - Determine the memory usage of functions
 - Datapack Generator
 - - Evaluate imports
 - - Generate function and variable names
 - - Convert the pree-processed AST into valid mcfunction files
 - Standard Library
 - - A standard library that all programs are dependent on
 - - Handles `alloc`, `dealloc`, `read`, and `write` functions
