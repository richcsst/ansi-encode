# ANSIEncode

![ANSIEncode Logo](ANSI-Encode.png?raw=true "ANSIEncode Logo Title Text 1")

## Description

Markup text to ANSI encoder

## Usage

**ansi-encode** [options] [text file name]

## Options

### -**v** or --**version**
```
    Show version and licensing info
```

### -**h** or --**help**
```
    Usage information
```

### -**t** or --**tokens**
```
    Show most used tokens
```

### -**c** or --**colors**
```
    Show color grid for use with "ANSI" and "GREY" tokens
```

### -**s** or --**symbols** [search]
```
    Show all of the symbol character tokens
	Use search to shorten the huge list
````

## Tokens
```
Tokens have to be encapsulated inside [% token %]

NOTE:  Use "less -r" to view in "less"

╭────────────────────────────────────────────────────────────────────────╮
│    ::::::::::::   ...      :::  .   .,:::::::::.    :::. .::::::.      │
│    ;;;;;;;;''''.;;;;;;;.   ;;; .;;,.;;;;''''`;;;;,  `;;;;;;`    `      │
│         [[    ,[[     \[[, [[[[[/'   [[cccc   [[[[[. '[['[==/[[[[,     │
│         $$    $$$,     $$$_$$$$,     $$""""   $$$ "Y$c$$  '''    $     │
│         88,   "888,_ _,88P"888"88o,  888oo,__ 888    Y88 88b    dP     │
│         MMM     "YMMMMMP"  MMM "MMP" """"YUMMMMMM     YM  "YMmMY"      │
├─────────────────────┬────────── GENERAL ───────────────────────────────┤
│ RETURN              │ ASCII RETURN (13)                                │
│ LINEFEED            │ ASCII LINEFEED (10)                              │
│ NEWLINE             │ RETURN + LINEFEED (13 + 10)                      │
│ CLEAR               │ Places cursor at top left, screen cleared        │
│ CLS                 │ Same as CLEAR                                    │
│ CLEAR LINE          │ Clear to the end of line                         │
│ CLEAR DOWN          │ Clear down from current cursor position          │
│ CLEAR UP            │ Clear up from current cursor position            │
│ RESET               │ Reset all colors and attributes                  │
├─────────────────────┼────────── CURSOR ────────────────────────────────┤
│ UP                  │ Moves cursor up one step                         │
│ DOWN                │ Moves cursor down one step                       │
│ RIGHT               │ Moves cursor right one step                      │
│ LEFT                │ Moves cursor left one step                       │
│ SAVE                │ Save cursor position                             │
│ RESTORE             │ Place cursor at saved position                   │
│ BOLD                │ Bold text (not all terminals support this)       │
│ FAINT               │ Faded text (not all terminals support this)      │
│ ITALIC              │ Italicized text (not all terminals support this) │
│ UNDERLINE           │ Underlined text (not all terminals support this) │
│ SLOW BLINK          │ Slow cursor blink                                │
│ RAPID BLINK         │ Rapid cursor blink                               │
├─────────────────────┼───────── ATTRIBUTES ─────────────────────────────┤
│ INVERT              │ Invert text (flip background and foreground)     │
│ REVERSE             │ Reverse                                          │
│ CROSSED OUT         │ Crossed out                                      │
│ DEFAULT FONT        │ Default font                                     │
├─────────────────────┼─────────── COLORS ───────────────────────────────┤
│ NORMAL              │ Sets colors to default                           │
├─────────────────────┼───────── FOREGROUND ─────────────────────────────┤
│ DEFAULT             │ Default foreground color                         │
│ BLACK               │ Black                                            │
│ RED                 │ Red                                              │
│ PINK                │ Hot pink                                         │
│ ORANGE              │ Orange                                           │
│ NAVY                │ Deep blue                                        │
│ GREEN               │ Green                                            │
│ YELLOW              │ Yellow                                           │
│ BLUE                │ Blue                                             │
│ MAGENTA             │ Magenta                                          │
│ CYAN                │ Cyan                                             │
│ WHITE               │ White                                            │
│ BRIGHT BLACK        │ Bright black (dim grey)                          │
│ BRIGHT RED          │ Bright red                                       │
│ BRIGHT GREEN        │ Lime                                             │
│ BRIGHT YELLOW       │ Bright Yellow                                    │
│ BRIGHT BLUE         │ Bright blue                                      │
│ BRIGHT MAGENTA      │ Bright magenta                                   │
│ BRIGHT CYAN         │ Bright cyan                                      │
│ BRIGHT WHITE        │ Bright white                                     │
│ ANSI0 - ANSI231     │ Term256 colors (use -c to see these)             │
│ GREY0 - GREY23      │ Levels of grey                                   │
├─────────────────────┼───────── BACKGROUND ─────────────────────────────┤
│ B_DEFAULT           │ Default background color                         │
│ B_BLACK             │  Black                                           │
│ B_RED               │  Red                                             │
│ B_GREEN             │  Green                                           │
│ B_YELLOW            │  Yellow                                          │
│ B_BLUE              │  Blue                                            │
│ B_MAGENTA           │  Magenta                                         │
│ B_CYAN              │  Cyan                                            │
│ B_WHITE             │  White                                           │
│ B_PINK              │  Hot pink                                        │
│ B_ORANGE            │  Orange                                          │
│ B_NAVY              │  Deep blue                                       │
│ BRIGHT B_BLACK      │  Bright black    (grey)                          │
│ BRIGHT B_RED        │  Bright red                                      │
│ BRIGHT B_GREEN      │  Lime                                            │
│ BRIGHT B_YELLOW     │  Bright yellow                                   │
│ BRIGHT B_BLUE       │  Bright blue                                     │
│ BRIGHT B_MAGENTA    │  Bright magenta                                  │
│ BRIGHT B_CYAN       │  Bright cyan                                     │
│ BRIGHT B_WHITE      │  Bright white                                    │
│ B_ANSI0 - B_ANSI231 │ Term256 background colors (use -c to see these)  │
│ B_GREY0 - B_GREY23  │ Levels of grey                                   │
╰─────────────────────┴──────────────────────────────────────────────────╯
```

