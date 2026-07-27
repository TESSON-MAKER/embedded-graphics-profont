import argparse
from font_api import get_table_names, get_value, load, load_table

NUMBER_OF_METADATA_ENTRIES = 8
CHAR_HEADER_LEN = 4


def reverse_bits_u8(value: int) -> int:
    """Reverse the 8 bits (b7→b0, …) for each exported byte."""
    v = value & 0xFF
    r = 0
    for _ in range(8):
        r = (r << 1) | (v & 1)
        v >>= 1
    return r

def main() -> None:
    parser = argparse.ArgumentParser(description="Convert a C font file to Rust format")
    parser.add_argument("input_c", help="Path to the input C file")
    parser.add_argument("output_rs", help="Path to the output RS file")
    args = parser.parse_args()

    load(args.input_c)

    table_names = get_table_names()
    if not table_names:
        raise ValueError("No font tables found in the C file")

    font_name = table_names[0]
    print(f"Font name extracted: {font_name}")

    table = load_table(font_name)
    
    identifier = get_value(table, 0)

    first_character_lsb = get_value(table, 2)
    first_character_msb = get_value(table, 3)
    first_character = (first_character_msb << 8) | first_character_lsb

    last_character_lsb = get_value(table, 4)
    last_character_msb = get_value(table, 5)
    last_character = (last_character_msb << 8) | last_character_lsb

    max_height = get_value(table, 6)
    proportional_width = False if get_value(table, 7) == 1 else True

    print("identifier:", identifier)
    print("first_character:", first_character)
    print("last_character:", last_character)
    print("max_height:", max_height)
    print("proportional_width:", proportional_width)

    number_of_characters = last_character - first_character + 1

    with open(args.output_rs, "w", encoding="utf-8") as f:

        f.write("/// This file contains the lookup table for the font characters and bitmap data.\n")
        f.write("\n")
        f.write("use embedded_graphics_profont::font::{Font, GlyphEntry};\n\n")

        rust_font_name = font_name.replace("-", "_")
        rust_upper = rust_font_name.upper()
        f.write("pub const %s: Font = Font {\n" % rust_upper)
        f.write("   lookup_table: &%s_LOOKUP_TABLE,\n" % rust_upper)
        f.write("   data: &%s_DATA,\n" % rust_upper)
        f.write("   ascii_begin: %d,\n" % first_character)
        f.write("   ascii_end: %d,\n" % last_character)
        f.write("   max_height: %d,\n" % max_height)
        f.write("   proportional_width: %s,\n" % str(proportional_width).lower())
        f.write("};\n\n")   

        f.write("pub static %s_LOOKUP_TABLE: [GlyphEntry; %d] = [\n" % (rust_upper, number_of_characters))

        for i in range(0, number_of_characters * CHAR_HEADER_LEN, CHAR_HEADER_LEN):
            char_code = first_character + i // CHAR_HEADER_LEN

            char_width = get_value(table, NUMBER_OF_METADATA_ENTRIES + i)
            raw_lsb = get_value(table, NUMBER_OF_METADATA_ENTRIES + i + 1)
            raw_msb = get_value(table, NUMBER_OF_METADATA_ENTRIES + i + 2)
            char_location = ((raw_msb << 8) | raw_lsb) - NUMBER_OF_METADATA_ENTRIES - number_of_characters * CHAR_HEADER_LEN

            f.write("    GlyphEntry { width: 0x%02X, offset: 0x%04X }, // Character: %s (ASCII %d)\n" %
                        (char_width, char_location, chr(char_code), char_code))
        f.write("];\n")
        f.write("\n")


        size_of_character_data = len(table) - NUMBER_OF_METADATA_ENTRIES - number_of_characters * CHAR_HEADER_LEN
        f.write("pub static %s_DATA: [u8; %d] = [\n" % (rust_upper, size_of_character_data))

        for letter in range(number_of_characters):
            char_code = first_character + letter

            base = NUMBER_OF_METADATA_ENTRIES + letter * 4
            char_width = get_value(table, base)
            char_location_lsb = get_value(table, base + 1)
            char_location_msb = get_value(table, base + 2)
            # Format MikroE : width, offset_LSB, offset_MSB, 0x00
            char_location = (char_location_msb << 8) | char_location_lsb

            # Bytes per glyph line (width in pixels → ceil(width/8))
            columns_of_bytes = (char_width + 7) // 8
            number_of_bytes = columns_of_bytes * max_height

            if letter < number_of_characters - 1:
                next_base = NUMBER_OF_METADATA_ENTRIES + (letter + 1) * 4
                char_location_next_base_lsb = get_value(table, next_base + 1)
                char_location_next_base_msb = get_value(table, next_base + 2)
                char_location_next_base = (char_location_next_base_msb << 8) | char_location_next_base_lsb

                verif_number_of_bytes = char_location_next_base - char_location

                if verif_number_of_bytes != number_of_bytes:
                    print(
                        f"Error: Character {char_code} has an unexpected number of bytes. "
                        f"Expected {number_of_bytes}, but got {verif_number_of_bytes}."
                    )
                else:
                    print(f"Character {char_code} has a valid number of bytes: {number_of_bytes}.")
            else:
                print(f"Character {char_code} is the last character, skipping byte count verification.")

            # RS: max_height bytes per line; comment on the last line of the glyph
            first_line = True
            for byte_index in range(number_of_bytes):
                char_reversed = reverse_bits_u8(get_value(table, char_location + byte_index))
                end_of_line = (byte_index + 1) % max_height == 0
                is_last = byte_index == number_of_bytes - 1

                if first_line:
                    f.write("\t0x%02X, " % char_reversed)
                    first_line = False
                else:
                    f.write("0x%02X, " % char_reversed)

                if is_last:
                    f.write(" // Character: %s (ASCII %d)\n" % (chr(char_code), char_code))
                    first_line = True
                elif end_of_line:
                    f.write("\n")
                    first_line = True
        f.write("];\n")

if __name__ == "__main__":
    main()
