// make support for US QWERTY keyboard first

// on my US QWERTY keyboard it is 17 keys across and 6 down
// it will be like a grid, 1:1 will be escape, 1:2 will be f1... etc

pub fn key_map() {
    // first row
    let key_1x1 = key_escape;
    let key_1x2 = key_f1;
    let key_1x3 = key_f2;
    let key_1x4 = key_f3;
    let key_1x5 = key_f4;
    let key_1x6 = key_f5;
    let key_1x7 = key_f6;
    let key_1x8 = key_f7;
    let key_1x9 = key_f8;
    let key_1x10 = key_f9;
    let key_1x11 = key_f10;
    let key_1x12 = key_f11;
    let key_1x13 = key_f12;
    // idk what 1x14 is supposed to be
    let key_1x15 = key_pauseBreak;
    let key_1x16 = key_insert;
    let key_1x17 = key_delete;

    // second row
    let key_2x1 = key_tild;
    let key_2x2 = key_1;
    let key_2x3 = key_2;
    let key_2x4 = key_3;
    let key_2x5 = key_4;
    let key_2x6 = key_5;
    let key_2x7 = key_6;
    let key_2x8 = key_7;
    let key_2x9 = key_8;
    let key_2x10 = key_9;
    let key_2x11 = key_0;
    let key_2x12 = key_dash;
    let key_2x13 = key_equal;
    let key_2x14 = key_backspace;

    // third row
    let key_3x1 = key_tab;
    let key_3x2 = key_q;
    let key_3x3 = key_w;
    let key_3x4 = key_e;
    let key_3x5 = key_r;
    let key_3x6 = key_t;
    let key_3x7 = key_y;
    let key_3x8 = key_u;
    let key_3x9 = key_i;
    let key_3x10 = key_o;
    let key_3x11 = key_p;
    let key_3x12 = key_lSquareBracket;
    let key_3x13 = key_rSquareBracket;
    let key_3x14 = key_fSlash;
}
