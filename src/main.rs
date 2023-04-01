use font_types::Pen;
use font_types::{FWord, Fixed, LongDateTime, Tag, UfWord};
use write_fonts::{tables, FontBuilder};

fn main() {
    let head_table = tables::head::Head {
        font_revision: Fixed::from_f64(0.0),
        checksum_adjustment: 0xa677dd1d,
        flags: 0b00000000_00000011,
        units_per_em: 1000,
        created: LongDateTime::new(3762796357),
        modified: LongDateTime::new(3762796357),
        x_min: 50,
        y_min: -200,
        x_max: 450,
        y_max: 800,
        mac_style: 0b00000000_00000000,
        lowest_rec_ppem: 6,
        index_to_loc_format: 1,
        magic_number: 0x5F0F3CF5,
        font_direction_hint: 2,
    };
    let head_bytes = write_fonts::dump_table(&head_table).unwrap();

    let hhea_table = tables::hhea::Hhea {
        ascender: FWord::new(1000),
        descender: FWord::new(-200),
        line_gap: FWord::new(0),
        advance_width_max: UfWord::new(500),
        min_left_side_bearing: FWord::new(50),
        min_right_side_bearing: FWord::new(50),
        x_max_extent: FWord::new(450),
        caret_slope_rise: 1,
        caret_slope_run: 0,
        caret_offset: 0,
        number_of_long_metrics: 1,
    };
    let hhea_bytes = write_fonts::dump_table(&hhea_table).unwrap();

    let maxp_table = tables::maxp::Maxp {
        num_glyphs: 1,
        max_points: Some(8),
        max_contours: Some(2),
        max_composite_points: Some(0),
        max_composite_contours: Some(0),
        max_zones: Some(1),
        max_twilight_points: Some(0),
        max_storage: Some(0),
        max_function_defs: Some(0),
        max_instruction_defs: Some(0),
        max_stack_elements: Some(0),
        max_size_of_instructions: Some(0),
        max_component_elements: Some(0),
        max_component_depth: Some(0),
    };
    let maxp_bytes = write_fonts::dump_table(&maxp_table).unwrap();

    let os2_table = tables::os2::Os2 {
        x_avg_char_width: 500,
        us_weight_class: 400,
        us_width_class: 5,
        fs_type: 0b00000000_00000100,
        y_subscript_x_size: 650,
        y_subscript_y_size: 600,
        y_subscript_x_offset: 0,
        y_subscript_y_offset: 75,
        y_superscript_x_size: 650,
        y_superscript_y_size: 600,
        y_superscript_x_offset: 0,
        y_superscript_y_offset: 350,
        y_strikeout_size: 50,
        y_strikeout_position: 300,
        s_family_class: 0,
        panose_10: [0; 10],
        ul_unicode_range_1: 0b00000000_00000000_00000000_00000000,
        ul_unicode_range_2: 0b00000000_00000000_00000000_00000000,
        ul_unicode_range_3: 0b00000000_00000000_00000000_00000000,
        ul_unicode_range_4: 0b00000000_00000000_00000000_00000000,
        ach_vend_id: Tag::new(b"None"),
        fs_selection: 0b00000000_01000000,
        us_first_char_index: 65535,
        us_last_char_index: 65535,
        s_typo_ascender: 800,
        s_typo_descender: -200,
        s_typo_line_gap: 200,
        us_win_ascent: 1000,
        us_win_descent: 200,
        ul_code_page_range_1: Some(0b00000000_00000000_00000000_00000001),
        ul_code_page_range_2: Some(0b00000000_00000000_00000000_00000000),
        sx_height: Some(500),
        s_cap_height: Some(700),
        us_default_char: Some(0),
        us_break_char: Some(32),
        us_max_context: Some(0),
        us_lower_optical_point_size: None,
        us_upper_optical_point_size: None,
    };
    let os2_bytes = write_fonts::dump_table(&os2_table).unwrap();

    let hmtx_table = tables::hmtx::Hmtx {
        h_metrics: vec![tables::hmtx::LongMetric {
            advance: 500,
            side_bearing: 50,
        }],
        left_side_bearings: vec![],
    };
    let hmtx_bytes = write_fonts::dump_table(&hmtx_table).unwrap();

    let cmap_table = tables::cmap::Cmap::from_mappings([]);
    let cmap_bytes = write_fonts::dump_table(&cmap_table).unwrap();

    let mut notdef_glyph = write_fonts::pens::BezPathPen::new();
    notdef_glyph.move_to(50., -250.);
    notdef_glyph.line_to(50., 750.);
    notdef_glyph.line_to(450., 750.);
    notdef_glyph.line_to(450., -250.);
    notdef_glyph.close();
    notdef_glyph.move_to(100., -200.);
    notdef_glyph.line_to(400., -200.);
    notdef_glyph.line_to(400., 700.);
    notdef_glyph.line_to(100., 700.);
    notdef_glyph.close();
    let notdef_glyph = tables::glyf::SimpleGlyph::from_kurbo(&notdef_glyph.into_inner()).unwrap();

    let mut loca = vec![0];
    let mut glyf: Vec<u8> = Vec::new();
    [notdef_glyph].iter().for_each(|g| {
        let bytes = write_fonts::dump_table(g).unwrap();
        loca.push(loca.last().unwrap() + bytes.len() as u32);
        glyf.extend(bytes);
    });
    let loca_bytes: Vec<u8> = loca
        .into_iter()
        .map(|b| b.to_be_bytes())
        .flatten()
        .collect();

    let mut name_table = tables::name::Name::default();
    name_table.name_record.insert(tables::name::NameRecord {
        name_id: font_types::NameId::new(1),
        platform_id: 3,
        encoding_id: 1,
        language_id: 0x409,
        string: write_fonts::OffsetMarker::new("New Font".into()),
    });
    name_table.name_record.insert(tables::name::NameRecord {
        name_id: font_types::NameId::new(2),
        platform_id: 3,
        encoding_id: 1,
        language_id: 0x409,
        string: write_fonts::OffsetMarker::new("Regular".into()),
    });
    name_table.name_record.insert(tables::name::NameRecord {
        name_id: font_types::NameId::new(3),
        platform_id: 3,
        encoding_id: 1,
        language_id: 0x409,
        string: write_fonts::OffsetMarker::new("0.000;NONE;NewFont-Regular".into()),
    });
    name_table.name_record.insert(tables::name::NameRecord {
        name_id: font_types::NameId::new(4),
        platform_id: 3,
        encoding_id: 1,
        language_id: 0x409,
        string: write_fonts::OffsetMarker::new("New Font Regular".into()),
    });
    name_table.name_record.insert(tables::name::NameRecord {
        name_id: font_types::NameId::new(5),
        platform_id: 3,
        encoding_id: 1,
        language_id: 0x409,
        string: write_fonts::OffsetMarker::new("Version 0.000".into()),
    });
    name_table.name_record.insert(tables::name::NameRecord {
        name_id: font_types::NameId::new(6),
        platform_id: 3,
        encoding_id: 1,
        language_id: 0x409,
        string: write_fonts::OffsetMarker::new("NewFont-Regular".into()),
    });
    let name_bytes = write_fonts::dump_table(&name_table).unwrap();

    let mut post_table = tables::post::Post::new_v2([".notdef"]);
    post_table.underline_position = FWord::new(-75);
    post_table.underline_thickness = FWord::new(50);
    let post_bytes = write_fonts::dump_table(&post_table).unwrap();

    let contents = FontBuilder::default()
        .add_table(Tag::new(b"head"), head_bytes)
        .add_table(Tag::new(b"hhea"), hhea_bytes)
        .add_table(Tag::new(b"maxp"), maxp_bytes)
        .add_table(Tag::new(b"OS/2"), os2_bytes)
        .add_table(Tag::new(b"hmtx"), hmtx_bytes)
        .add_table(Tag::new(b"cmap"), cmap_bytes)
        .add_table(Tag::new(b"loca"), loca_bytes)
        .add_table(Tag::new(b"glyf"), glyf)
        .add_table(Tag::new(b"name"), name_bytes)
        .add_table(Tag::new(b"post"), post_bytes)
        .build();

    std::fs::write("/tmp/f.ttf", contents).unwrap();
}
