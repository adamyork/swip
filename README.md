# swip
somewhat working interpreter

## repl
By default, the swip binary will run in repl mode. Accepted flags are :
- -d (debug mode on)
- -f (load file flag) filename.swip
    - swip will now be in headless mode in await state

## syntax

### variable declaration
```
*" foo (a immutable string (defaults to empty string))
**" foo  (mutable string (defaults to empty string))
*$ foo (mutable number (defaults to 0))
**$ foo (immutable number (defaults to 0))
*b foo (mutable boolean (defaults to false))
**b foo (immutable boolean (defaults to false))
```
### initialization
```
*"" someImmutableString "hello"
**"" someMutableString "world"
**b someMutableBoolean true
```
### assignment
#### left gets right value variable
```
**b someBool true
**b someOtherBool someBool (someOtherBool is true now)
```
#### left gets right value operation result (boolean AND)
```
**b someBool true
**b someOtherBool false
**b someThirdBool someBool.someOtherBool (someThirdBool is false)
```
#### left gets right value operation result (boolean OR)
```
**b someBool true
**b someOtherBool false
**b someThirdBool someBool\someOtherBool (someThirdBool is true)
```
#### string concatenation
```
**" someString "hello"
**" someOtherString "world"
**" someThirdString someString.someOtherString (someThirdString is "helloworld")
```
#### numeric operations
```
someNumber.someOtherNUmber addition
someNumber-someOtherNumber subtraction
someNumber/someOtherNumber division
someNumber'someOtherNumber multiplication
```
### ordered operation
TBD
### control flow
someNumber>=someOtherNumber -> "hi" --> "goodbye" if / else
someNumber>=someOtherNumber -> "hi" _> >= someOtherOtherNumber -> "hello" --> "goodbye" if / else if / else
### debugging
#### print 
```
# "hi" 
```


