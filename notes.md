## Modes (:help vim-modes)
```
Normal mode		In Normal mode you can enter all the normal editor
			commands.  If you start the editor you are in this
			mode (unless you have set the 'insertmode' option,
			see below).  This is also known as command mode.

Visual mode		This is like Normal mode, but the movement commands
			extend a highlighted area.  When a non-movement
			command is used, it is executed for the highlighted
			area.  See |Visual-mode|.
			If the 'showmode' option is on "-- VISUAL --" is shown
			at the bottom of the window.

Select mode		This looks most like the MS-Windows selection mode.
			Typing a printable character deletes the selection
			and starts Insert mode.  See |Select-mode|.
			If the 'showmode' option is on "-- SELECT --" is shown
			at the bottom of the window.

Insert mode		In Insert mode the text you type is inserted into the
			buffer.  See |Insert-mode|.
			If the 'showmode' option is on "-- INSERT --" is shown
			at the bottom of the window.

Command-line mode	In Command-line mode (also called Cmdline mode) you
Cmdline mode		can enter one line of text at the bottom of the
			window.  This is for the Ex commands, ":", the pattern
			search commands, "?" and "/", and the filter command,
			"!".  |Cmdline-mode|

Ex mode			Like Command-line mode, but after entering a command
			you remain in Ex mode.  Very limited editing of the
			command line.  |Ex-mode|

							*Terminal-mode*
Terminal mode		In Terminal mode all input (except |c_CTRL-\_CTRL-N|)
			is sent to the process running in the current
			|terminal| buffer.
			If the 'showmode' option is on "-- TERMINAL --" is shown
			at the bottom of the window.

There are six ADDITIONAL modes.  These are variants of the BASIC modes:

				*Operator-pending* *Operator-pending-mode*
Operator-pending mode	This is like Normal mode, but after an operator
			command has started, and Vim is waiting for a {motion}
			to specify the text that the operator will work on.

Replace mode		Replace mode is a special case of Insert mode.  You
			can do the same things as in Insert mode, but for
			each character you enter, one character of the existing
			text is deleted.  See |Replace-mode|.
			If the 'showmode' option is on "-- REPLACE --" is
			shown at the bottom of the window.

Virtual Replace mode	Virtual Replace mode is similar to Replace mode, but
			instead of file characters you are replacing screen
			real estate.  See |Virtual-Replace-mode|.
			If the 'showmode' option is on "-- VREPLACE --" is
			shown at the bottom of the window.

Insert Normal mode	Entered when CTRL-O given in Insert mode.  This is
			like Normal mode, but after executing one command Vim
			returns to Insert mode.
			If the 'showmode' option is on "-- (insert) --" is
			shown at the bottom of the window.

Insert Visual mode	Entered when starting a Visual selection from Insert
			mode, e.g., by using CTRL-O and then "v", "V" or
			CTRL-V.  When the Visual selection ends, Vim returns
			to Insert mode.
			If the 'showmode' option is on "-- (insert) VISUAL --"
			is shown at the bottom of the window.

Insert Select mode	Entered when starting Select mode from Insert mode.
			E.g., by dragging the mouse or <S-Right>.
			When the Select mode ends, Vim returns to Insert mode.
			If the 'showmode' option is on "-- (insert) SELECT --"
			is shown at the bottom of the window.
```
## Key Notation (:help key-notation)

```
notation	meaning		    equivalent	decimal value(s)	~
-----------------------------------------------------------------------
<Nul>		zero			CTRL-@	  0 (stored as 10) *<Nul>*
<BS>		backspace		CTRL-H	  8	*backspace*
<Tab>		tab			CTRL-I	  9	*tab* *Tab*
							*linefeed*
<NL>		linefeed		CTRL-J	 10 (used for <Nul>)
<FF>		formfeed		CTRL-L	 12	*formfeed*
<CR>		carriage return		CTRL-M	 13	*carriage-return*
<Return>	same as <CR>				*<Return>*
<Enter>		same as <CR>				*<Enter>*
<Esc>		escape			CTRL-[	 27	*escape* *<Esc>*
<Space>		space				 32	*space*
<lt>		less-than		<	 60	*<lt>*
<Bslash>	backslash		\	 92	*backslash* *<Bslash>*
<Bar>		vertical bar		|	124	*<Bar>*
<Del>		delete				127
<CSI>		command sequence intro  ALT-Esc 155	*<CSI>*
<xCSI>		CSI when typed in the GUI		*<xCSI>*

<EOL>		end-of-line (can be <CR>, <LF> or <CR><LF>,
		depends on system and 'fileformat')	*<EOL>*

<Up>		cursor-up			*cursor-up* *cursor_up*
<Down>		cursor-down			*cursor-down* *cursor_down*
<Left>		cursor-left			*cursor-left* *cursor_left*
<Right>		cursor-right			*cursor-right* *cursor_right*
<S-Up>		shift-cursor-up
<S-Down>	shift-cursor-down
<S-Left>	shift-cursor-left
<S-Right>	shift-cursor-right
<C-Left>	control-cursor-left
<C-Right>	control-cursor-right
<F1> - <F12>	function keys 1 to 12		*function_key* *function-key*
<S-F1> - <S-F12> shift-function keys 1 to 12	*<S-F1>*
<Help>		help key
<Undo>		undo key
<Insert>	insert key
<Home>		home				*home*
<End>		end				*end*
<PageUp>	page-up				*page_up* *page-up*
<PageDown>	page-down			*page_down* *page-down*
<kHome>		keypad home (upper left)	*keypad-home*
<kEnd>		keypad end (lower left)		*keypad-end*
<kPageUp>	keypad page-up (upper right)	*keypad-page-up*
<kPageDown>	keypad page-down (lower right)	*keypad-page-down*
<kPlus>		keypad +			*keypad-plus*
<kMinus>	keypad -			*keypad-minus*
<kMultiply>	keypad *			*keypad-multiply*
<kDivide>	keypad /			*keypad-divide*
<kEnter>	keypad Enter			*keypad-enter*
<kPoint>	keypad Decimal point		*keypad-point*
<k0> - <k9>	keypad 0 to 9			*keypad-0* *keypad-9*
<S-...>		shift-key			*shift* *<S-*
<C-...>		control-key			*control* *ctrl* *<C-*
<M-...>		alt-key or meta-key		*META* *meta* *alt* *<M-*
<A-...>		same as <M-...>			*<A-*
<D-...>		command-key or "super" key	*<D-*
```

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
