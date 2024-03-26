# BBTR (Block Based Text Representation) Specification


```
# Top Level
Contract := ContractName Interface Data

# Contract Level
ContractName := [a-zA-Z][a-zA-Z_0-9]*
Data := ???
Interface := Function*

# Contract Interface
Function := FunctionName (FunctionArg+[, FunctionArg]*) FunctionOutType FunctionBody

# Function Definition
FunctionName := [a-zA-Z][a-zA-Z_0-9]*
FunctionArg := (ArgName, ArgType)*
ArgName := [a-zA-Z][a-zA-Z_0-9]*
ArgType := TypeName
FunctionOutType := TypeName
FunctionBody := Statement

# Statement Defintion
Statement := AssignmentStatement | DataStateStatement | ReturnStatement
DataStateStatement := StateInitializer | StateCaller
StateInitializer := state = state.fetch
StateCaller := state.StateMethod
StateMethod := FunctionName (ArgName+[, ArgName]*)
ReturnStatement := ???

# Base Types
Types := Address | Bytes | Map | Num | String | Symbol | Vector
Address :=
Bytes := 
Event :=
Map := [(KEY, VALUE)* ]
Num := i256 | u256 | Duration | Timepoint
String :=
Symbol := 
Vector :=

TypeName :=
```
