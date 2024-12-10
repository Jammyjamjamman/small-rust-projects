@group(0) @binding(0) var<storage, read> str: array<u32>;
@group(0) @binding(1) var<storage, read_write> max_doublechar_len: array<u32>;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) gid : vec3<u32>) {
    let i = gid.x;
    if (i < arrayLength(&max_doublechar_len)) {
        var char1 = str[i];
        var char2 = str[i+1];
        var eol = false;
        var doublechar_len: u32 = 2;

        for (var j: u32 = 2; j < arrayLength(&max_doublechar_len); j++) {
            if(i+j < arrayLength(&str)) {
                let testchar = str[i+j];
                char2 = select(char2, testchar, char1 == char2);
                eol |= testchar != char1 && testchar != char2;
                doublechar_len += select(u32(1), u32(0), eol);
            }
        }
        max_doublechar_len[i] = doublechar_len;
    }
}
