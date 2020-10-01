// QMK keymap

keymaps!(
    rows => MATRIX_ROWS,
    cols => MATRIX_COLS,
    layer_cnt => 4,
    layer!( // layer 0
        r!(     '`'     | 1  | 2  | 3 | 4 | 5 |  6  | 7 | 8 | 9 |   0   |  -  | =  |BSPC |DEL  ),
        r!( [TAB &LT{3}]| Q  | W  | E | R | T |  Y  | U | I | O |   P   | '[' |']' |BSLS |HOME ),
        r!(    [MO{2}]  | A  | S  | D | F | G |  H  | J | K | L |   ;   |QUOTE|xxxx| '⏎' |PGUP ),
        r!(     LSFT    |xxxx| Z  | X | C | V |  B  | N | M | , |   .   |  /  |RSFT| '↑' |PGDN ),
        r!(     LCTL    |LALT|LGUI|xxx|xxx|xxx|SPACE|xxx|xxx|xxx|[MO{1}]|RGUI |'←' | '↓' | '→' ),
    ),
    layer!( // layer 1
        r!( ESC  |F1 | F2 | F3  | F4  | F5  |  F6   |  F7   |   F8   | F9 |F10 |F11 |F12 |[  ]|'🔇' ),
        r!( T_BR |PSD|BRI | PSI |EDG_I|[   ]| [   ] | [   ] |U_T_AGCR|[  ]|PSCR|SLCK|PAUS|[  ]|END  ),
        r!( T_PTD|PTP|BRD | PTN |[   ]|[   ]|  '←'  |  '↓'  |  '↑'   |'→' |[  ]|[  ]|xxxx|[  ]|'🔊' ),
        r!( [  ] |xxx|T_MD|T_ONF|[   ]|EDG_M|MD_BOOT|TG_NKRO| [    ] |[  ]|[  ]|[  ]|[  ]|PGUP|'🔉' ),
        r!( [  ] |[ ]|[  ]|xxxxx|xxxxx|xxxxx|DBG_FAC|xxxxxxx|xxxxxxxx|xxxx|[  ]|[  ]|HOME|PGDN|END  ),
    ),
    ...
);