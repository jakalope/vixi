## Normal Mode

## Operator-Pending Mode

Mode entered after an `<op>` has been parsed.

```
[count1]<op>[count2]<motion-or-object>

for i in [ 0 ... (count1 * count2) ] {
    <op><motion>
}
```

### `<op>`

keys | effect
-----| -------
`c`  | change (results in Insert Mode)
`d`  | delete
`y`  | yank into register (does not change the text)
`~`  | swap case (only if 'tildeop' is set)
`g~` | swap case
`gu` | make lowercase
`gU` | make uppercase
`!`  | filter through an external program
`=`  | filter through 'equalprg' or C-indenting if empty
`gq` | text formatting
`g?` | ROT13 encoding
`>`  | shift right
`<`  | shift left
`zf` | define a fold
`g@` | call function set with the 'operatorfunc' option


### `<motion-or-object>`

Not a full list, just a smattering, to wet the appetite.

#### Left-right

```
hj
0^$
g_ g0 g^ gm g$ |
fFtT;,
```

#### Up-down

```
:some_command_that_moves<CR>
/thing_to_search_for<CR>
{count}%
```

#### Words

```
wWeEbB
ge gE
```

#### Text objects

```
(){} ]] ][ [[ []
```

## Insert

If `{count}` is greater than one when leaving Insert, `{count}` copies
of the resulting insertion take place.
