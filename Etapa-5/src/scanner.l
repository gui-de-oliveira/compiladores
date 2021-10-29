%%

[ \t]+ ;
//+[^\r\n]* ;
/[*](~(.*[*]/.*))[*]/ ;
\n|(\r\n) ;

int "TK_PR_INT"
float "TK_PR_FLOAT"
bool "TK_PR_BOOL"
char "TK_PR_CHAR"
string "TK_PR_STRING"
then "TK_PR_THEN"
else "TK_PR_ELSE"
do "TK_PR_DO"
if "TK_PR_IF"
for "TK_PR_FOR"
while "TK_PR_WHILE"
input "TK_PR_INPUT"
output "TK_PR_OUTPUT"
return "TK_PR_RETURN"
break "TK_PR_BREAK"
continue "TK_PR_CONTINUE"
const "TK_PR_CONST"
static "TK_PR_STATIC"
foreach "TK_PR_FOREACH"
switch "TK_PR_SWITCH"
case "TK_PR_CASE"
class "TK_PR_CLASS"
private "TK_PR_PRIVATE"
public "TK_PR_PUBLIC"
protected "TK_PR_PROTECTED"

false "TK_LIT_FALSE"

true "TK_LIT_TRUE"

end "TK_PR_END"
default "TK_PR_DEFAULT"

<= "TK_OC_LE"
>= "TK_OC_GE"
== "TK_OC_EQ"
!= "TK_OC_NE"
&& "TK_OC_AND"
\|\| "TK_OC_OR"
>> "TK_OC_SR"
<< "TK_OC_SL"

[,] ","
[;] ";"
[:] ":"
[(] "("
[)] ")"
[\[] "["
[\]] "]"
[{] "{"
[}] "}"
[.] "."
[$] "$"

[+] "+"
[-] "-"
[*] "*"
[/] "/"
[<] "<"
[>] ">"
[=] "="
[!] "!"
[&] "&"
[|] "|"
[%] "%"
[#] "#"
\^ "^"
[?] "?"

[a-zA-Z_][a-zA-Z_0-9]* "TK_IDENTIFICADOR"

[0-9]+ "TK_LIT_INT"
[0-9]+\.[0-9]+([eE][+-]?[0-9]+)? "TK_LIT_FLOAT"

'.' "TK_LIT_CHAR"

"(\\.|[^"\r\n])*" "TK_LIT_STRING"

. "TOKEN_ERRO"
