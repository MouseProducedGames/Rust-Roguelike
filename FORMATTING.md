# Formatting for Source Files

## Arrays

### Array Access

array_name[ array_index ]

### Array Declarations

array_name[ Type; /* capacity as usize */ ]

### Array Definitions

[ std::default::Default; /* capacity as usize */ ],

Or;

[ val0, val1, val1, ... ]

## Contexts

// Context-specific comments here.
{
    // Code here.
}

### Nested Contexts

// Context-specific comments here.
{
    // Code here
    // Context-specific comments here.
    {
        // Code here.
    }

    // Code here
    // Context-specific comments here.
    {
        // Code here.
    }
}

### While Contexts

As per general Contexts formatting. While statement should immediately preceed context.

while check0 [equality operator]
    check1 [equality operator]
    check2 [equality operator]
    ...
{
    // Code here.
}

// Code here.

## Functions

### Function Declarations

[pub] fn func_name( arg_name0, arg_name1, arg_name2... );

### Function Definitions

[pub] fn func_name( arg_name0, arg_name1, arg_name2... )
{
    // Code

    // return using a return expression.
    // return expressions are not preceeded by 'return',
    // and have no closing semi-colon.
    return_expr
}

## If Statements

See Contexts.

## Includes

Prefer referencing types directly rather than through an include_prefix::.
Order includes alphabetically within their region.

// External Includes
// External crates first.
use crate crate_name;
// Then external modules.
mod include_name;
// Then included external modules and types.
use include_name;
use include_name::IncludeType;
// Alphabetical ordering of included types.
use include_name::{ AIncludeType, BIncludeType, CIncludeType, ... };

// Internal Includes
// External modules.
mod include_name;
// Then included external modules and types.
use include_name;
use include_name::IncludeType;
// Alphabetical ordering of included types.
use include_name::{ AIncludeType, BIncludeType, CIncludeType, ... };

## Match Expressions

// Code here.
match expr {
    case_formatting_option0 => ,
    case_formatting_option1 => { }, // If line is within 80 chars.
    case_formatting_option0 => {
    }, // If line is longer than 80 chars, or contains more than two statements.
}

// Code here.

## Templates

### Function Templates

[pub] fn func_name< CamelType0, CamelType1, CamelType2, ... > // remainder as per Functions.

### Struct Templates

[pub] struct struct_name< CamelType0, CamelType1, CamelType2, ... >

### Trait Templates

[pub] trait struct_name< CamelType0, CamelType1, CamelType2, ... >

## Variables

let variable_name;
let mut variable_name;

If a variable is accessed from elsewhere two or more times:
* In the same context, prefer pulling the variable in to the context.
* In contiguous contexts, consider pulling the variable in to the outer context IF all subcontexts of the outer context reference that variable.x