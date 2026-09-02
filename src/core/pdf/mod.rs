// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2026 jgabaut, gioninjo

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::app::core::{CISBA_LOGO_DATA, PROJECT_LOGO_DATA, PROJECT_LOGO_NAME_DATA};
use crate::core::BUILD_DATE;
use esox::domain::hfbi::{AnagraficaHFBI, RisultatoHFBI};
use esox::domain::niseci::{AnagraficaNISECI, RiferimentoNISECI};

use esox::{
    domain::niseci::RisultatoNISECI, engines::niseci::full::calculate_stato_ecologico_niseci,
};

use esox::engines::hfbi::full::calculate_stato_ecologico_hfbi;
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::{Chunk, Content, Filter, Finish, Name, Pdf, Rect, Ref, Str};
use png::Decoder;
use std::io::Cursor;
use std::path::PathBuf;

// Decode PNG into RGB and optional alpha mask
fn decode_png(data: &[u8]) -> (Vec<u8>, Option<Vec<u8>>, u32, u32) {
    // Wrap the byte slice in a Cursor so it implements BufRead + Seek
    let cursor = Cursor::new(data);
    let decoder = Decoder::new(cursor);
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![
        0;
        reader
            .output_buffer_size()
            .expect("PNG buffer size unknown")
    ];
    let info = reader.next_frame(&mut buf).unwrap();

    let mut rgb = Vec::with_capacity((info.width * info.height * 3) as usize);
    let mut alpha = Vec::with_capacity((info.width * info.height) as usize);

    match info.color_type {
        png::ColorType::Rgb => {
            rgb.extend_from_slice(&buf);
        }
        png::ColorType::Rgba => {
            for chunk in buf.chunks_exact(4) {
                rgb.extend_from_slice(&chunk[0..3]);
                alpha.push(chunk[3]);
            }
        }
        other => panic!("unsupported color type: {:?}", other),
    }

    let mask = if alpha.is_empty() { None } else { Some(alpha) };
    (rgb, mask, info.width, info.height)
}

struct PdfImage {
    id: Ref,
    name: &'static [u8],
    width: i32,
    height: i32,
}

fn prep_image(data: &[u8], alloc: &mut Ref, pdf: &mut Pdf, name: &'static [u8]) -> PdfImage {
    let image_id = alloc.bump();
    let s_mask_id = alloc.bump();

    // Decode the image.

    let (rgb, mask, width, height) = decode_png(data);

    let level = CompressionLevel::DefaultLevel as u8;
    let encoded = compress_to_vec_zlib(&rgb, level);
    let mask_encoded = mask.as_ref().map(|a| compress_to_vec_zlib(a, level));

    let filter = Filter::FlateDecode; // PNGs always use FlateDecode

    // Write the image stream
    {
        let mut image = pdf.image_xobject(image_id, &encoded);
        image.filter(filter);
        image.width(width as i32);
        image.height(height as i32);
        image.color_space().device_rgb();
        image.bits_per_component(8);
        if mask_encoded.is_some() {
            image.s_mask(s_mask_id);
        }
        image.finish();
    }

    // Add SMask if the image has transparency
    if let Some(encoded) = &mask_encoded {
        let mut s_mask = pdf.image_xobject(s_mask_id, encoded);
        s_mask.filter(filter);
        s_mask.width(width as i32);
        s_mask.height(height as i32);
        s_mask.color_space().device_gray();
        s_mask.bits_per_component(8);
    }

    PdfImage {
        id: image_id,
        name,
        width: width as i32,
        height: height as i32,
    }
}

fn draw_image(content: &mut Content, image: &PdfImage, x: f32, y: f32, w: f32, h: f32) {
    content.save_state();
    content.transform([w, 0.0, 0.0, h, x, y]);
    content.x_object(Name(image.name));
    content.restore_state();
}

pub(crate) fn esporta_pdf_niseci(
    export_path: PathBuf,
    _riferimento_niseci: RiferimentoNISECI,
    anagrafica_niseci: AnagraficaNISECI,
    risultato_niseci: RisultatoNISECI,
) {
    let valore_niseci = match risultato_niseci.get_valore() {
        Some(v) => &format!("{}", v),
        None => "NC",
    };
    let valore_rqe_niseci = match risultato_niseci.get_rqe() {
        Some(v) => &format!("{}", v),
        None => "NC",
    };
    let stato_eco_niseci = match calculate_stato_ecologico_niseci(
        risultato_niseci.get_valore(),
        &anagrafica_niseci.area,
    ) {
        Some(v) => &format!("{}", v),
        None => "NC",
    };
    let x1 = &format!("{}", risultato_niseci.get_x1());
    let x2 = match risultato_niseci.get_x2() {
        Some(v) => &format!("{}", v),
        None => "NC",
    };
    let x3 = &format!("{}", risultato_niseci.get_x3());
    /*
    let filtered_riferimento_niseci: Vec<SpecieNISECI> = riferimento_niseci
        .elenco_specie
        .into_iter()
        .filter(|specie| specie.specie_attesa)
        .collect();
    */

    // Define an ID allocator. Every time we need a new object, we just call
    // `alloc.bump()`, which increases `alloc` by one and returns its previous
    // value.
    let mut alloc = Ref::new(1);

    // Start writing.
    let mut pdf = Pdf::new();

    // Create a secondary chunk for parallel writing. This will allows us to
    // write stuff while we're also holding a mutable reference to the main
    // writer.
    let mut secondary = Chunk::new();

    // Define some indirect reference ids we'll use.
    let page_tree_id = alloc.bump();

    let mut page_ids = vec![];

    let a4 = Rect::new(0.0, 0.0, 595.0, 842.0);
    let image_1 = prep_image(PROJECT_LOGO_DATA, &mut alloc, &mut pdf, b"I1");

    let w = (image_1.width / 8) as f32;
    let h = (image_1.height / 8) as f32;

    let x = a4.x2 * 0.1;
    let y = 762.0; //(a4.y2 - h) / 2.0;

    let image_2 = prep_image(CISBA_LOGO_DATA, &mut alloc, &mut pdf, b"I2");

    let w_2 = (image_2.width / 6) as f32;
    let h_2 = (image_2.height / 6) as f32;

    let x_2 = (a4.x2 * 0.9) - w_2;
    let y_2 = y;

    let image_3 = prep_image(PROJECT_LOGO_NAME_DATA, &mut alloc, &mut pdf, b"I3");

    let w_3 = (image_3.width / 6) as f32;
    let h_3 = (image_3.height / 6) as f32;

    let x_3 = (a4.x2 - w_3) / 2.0;
    let y_3 = y + (h - h_3) / 2.0;

    // Page 1
    let page_id = alloc.bump();
    page_ids.push(page_id);

    let font_id = alloc.bump();
    let font_name = Name(b"F1");

    // Table geometry
    let height = 842.0;
    let cell_width = 240.0;
    let cell_height = 30.0;
    let x_start = 58.0;
    let y_start = height - 168.0;

    // Write a page.
    {
        // Add page
        let mut page = pdf.page(page_id);
        // Set the size to A4 (measured in points) using `media_box` and set the
        // text object we'll write later as the page's contents.
        page.media_box(a4);
        page.parent(page_tree_id);
        // We also need to specify which resources the page needs, which in our case
        // is only a font that we name "F1" (the specific name doesn't matter).
        {
            let mut resources = page.resources();
            resources.fonts().pair(font_name, font_id);
            resources
                .x_objects()
                .pair(Name(image_1.name), image_1.id)
                .pair(Name(image_2.name), image_2.id)
                .pair(Name(image_3.name), image_3.id);
        }

        // Write a line of text, with the font specified in the resource list
        // before, at a font size of 14.0, starting at coordinates (58.0, 734.0)
        // measured from the bottom left of the page.
        //
        // Because we haven't specified any encoding when writing the Type 1 font,
        // the standard encoding is used which happens to work with most ASCII
        // characters.
        let mut content = Content::new();

        content.begin_text();
        content.set_font(font_name, 14.0);
        content.set_leading(30.0);
        content.next_line(58.0, 684.0);
        content.show(Str(&format!("{}", anagrafica_niseci.comunita).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Codice stazione: {}",
            anagrafica_niseci.codice_stazione
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Data campionamento: {}",
            anagrafica_niseci.date_string
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("{}", anagrafica_niseci.area).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Corpo idrico: {}",
            anagrafica_niseci.corpo_idrico
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Bacino: {}",
            anagrafica_niseci.bacino_appartenenza
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Idroecoregione: {}",
            anagrafica_niseci.idro_eco_regione
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Regione: {}",
            anagrafica_niseci.posizione.regione
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Provincia: {}",
            anagrafica_niseci.posizione.provincia
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Lunghezza stazione: {}",
            anagrafica_niseci.get_lunghezza_media()
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Larghezza stazione: {}",
            anagrafica_niseci.get_larghezza_media()
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Niseci: {}", valore_niseci).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(
            &format!("RQE Niseci: {}", valore_rqe_niseci).into_bytes()
        ));
        content.next_line(0.0, -30.0);
        content.show(Str(
            &format!("Stato ecologico: {}", stato_eco_niseci).into_bytes()
        ));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("X1: {}", x1).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("X2: {}", x2).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("X3: {}", x3).into_bytes()));
        content.end_text();

        let cols = 2;
        let rows = 16;

        // Horizontal lines
        for row in 0..=rows {
            let y = y_start - (row as f32 * cell_height);
            content.move_to(x_start, y);
            content.line_to(x_start + (cols as f32 * cell_width), y);
            content.stroke();
        }

        draw_image(&mut content, &image_1, x, y, w, h);
        draw_image(&mut content, &image_2, x_2, y_2, w_2, h_2);
        draw_image(&mut content, &image_3, x_3, y_3, w_3, h_3);

        content.move_to(x_start, y - 10.0);
        content.line_to(x_start + (cols as f32 * cell_width), y - 10.0);
        content.stroke();

        content.begin_text();
        content.set_font(font_name, 12.0);
        content.next_line(a4.x2 / 2.0 - 60.0, y - 30.0);
        content.show(Str(&"Applicazione NISECI".to_string().into_bytes()));

        content.next_line(20.0, -15.0);
        content.show(Str(&"DM 260/2010".to_string().into_bytes()));
        content.end_text();

        content.move_to(x_start, 30.0);
        content.line_to(x_start + (cols as f32 * cell_width), 30.0);
        content.stroke();

        content.begin_text();
        content.next_line(a4.x2 / 2.0 - 115.0, 15.0);
        content.show(Str(&format!(
            "F-value v{}, Data release: {}",
            env!("CARGO_PKG_VERSION"),
            BUILD_DATE
        )
        .into_bytes()));
        content.end_text();

        //This can be used to debug the content before streaming it
        //let content_bytes = content.finish();
        //println!("{}", String::from_utf8_lossy(&content_bytes));
        //pdf.stream(content_id, &content_bytes);

        let content_id = alloc.bump();
        secondary.stream(content_id, &content.finish());
        page.contents(content_id);
    }

    /*
    for chunk in filtered_riferimento_niseci.chunks(15) {
        // Page 2+x
        let pagex_id = alloc.bump();
        page_ids.push(pagex_id);

        // Add page 2+x
        let mut pagex = pdf.page(pagex_id);
        pagex.media_box(a4);
        pagex.parent(page_tree_id);
        {
            let mut resources = pagex.resources();
            resources.fonts().pair(font_name, font_id);
            resources.x_objects().pair(image_name, image_id);
        }

        // Content for page 2+x
        let mut contentx = Content::new();
        contentx.begin_text();
        contentx.set_font(font_name, 14.0);
        contentx.set_leading(30.0);
        contentx.next_line(58.0, 764.0);
        contentx.show(Str(
            b"Specie, cl1, cl2, cl3, cl4, adj1, adj2, adj3, adj4, dsoglia1, dsoglia2",
        ));
        contentx.next_line(0.0, -30.0);

        for specie in chunk {
            contentx.show(Str(&format!(
                "{}, {:.3}, {:.3}, {:.3}, {:.3}, {:.3}, {:.3}, {:.3}, {:.3}, {:.3}, {:.3}",
                specie.nome,
                specie.cl_soglia1,
                specie.cl_soglia2,
                specie.cl_soglia3,
                specie.cl_soglia4,
                specie.ad_juv_soglia1,
                specie.ad_juv_soglia2,
                specie.ad_juv_soglia3,
                specie.ad_juv_soglia4,
                specie.dens_soglia1,
                specie.dens_soglia2
            )
            .into_bytes()));
            contentx.next_line(0.0, -30.0);
        }
        contentx.end_text();

        let cols = 2;
        let rows = chunk.len();

        // Horizontal lines
        for row in 0..=rows {
            let y = y_start - (row as f32 * cell_height);
            contentx.move_to(x_start, y);
            contentx.line_to(x_start + (cols as f32 * cell_width), y);
            contentx.stroke();
        }

        contentx.save_state();
        contentx.transform([w, 0.0, 0.0, h, x, y]);
        contentx.x_object(image_name);
        contentx.restore_state();

        let contentx_id = alloc.bump();
        secondary.stream(contentx_id, &contentx.finish());
        pagex.contents(contentx_id);
    }
    */

    // Specify the font we want to use. Because Helvetica is one of the 14 base
    // fonts shipped with every PDF reader, we don't have to embed any font
    // data.
    pdf.type1_font(font_id).base_font(Name(b"Helvetica"));

    pdf.extend(&secondary);

    // Write the page tree with a single child page.
    pdf.pages(page_tree_id)
        .kids(page_ids.iter().copied())
        .count(page_ids.len() as i32);

    // Write the document catalog with a reference to the page tree.
    pdf.catalog(alloc.bump()).pages(page_tree_id);

    // Finish writing (this automatically creates the cross-reference table and
    // file trailer) and retrieve the resulting byte buffer.
    let buf: Vec<u8> = pdf.finish();

    // Write the thing to a file.
    match std::fs::write(&export_path, buf) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed writing pdf to {}: {}", export_path.display(), e);
        }
    }
}

pub(crate) fn esporta_pdf_hfbi(
    export_path: PathBuf,
    anagrafica_hfbi: AnagraficaHFBI,
    risultato_hfbi: RisultatoHFBI,
) {
    let valore_hfbi = match risultato_hfbi.get_valore() {
        Some(v) => &format!("{}", v),
        None => "NC",
    };
    let intermediates = risultato_hfbi.get_intermediates();
    let valore_mmi = intermediates.mmi;
    let stato_eco = match calculate_stato_ecologico_hfbi(risultato_hfbi.get_valore()) {
        Some(v) => &format!("{}", v),
        None => "NC",
    };

    // Define an ID allocator. Every time we need a new object, we just call
    // `alloc.bump()`, which increases `alloc` by one and returns its previous
    // value.
    let mut alloc = Ref::new(1);

    // Start writing.
    let mut pdf = Pdf::new();

    // Create a secondary chunk for parallel writing. This will allows us to
    // write stuff while we're also holding a mutable reference to the main
    // writer.
    let mut secondary = Chunk::new();

    // Define some indirect reference ids we'll use.
    let page_tree_id = alloc.bump();

    let mut page_ids = vec![];

    let a4 = Rect::new(0.0, 0.0, 595.0, 842.0);
    let image_1 = prep_image(PROJECT_LOGO_DATA, &mut alloc, &mut pdf, b"I1");

    let w = (image_1.width / 8) as f32;
    let h = (image_1.height / 8) as f32;

    let x = a4.x2 * 0.1;
    let y = 762.0; //(a4.y2 - h) / 2.0;

    let image_2 = prep_image(CISBA_LOGO_DATA, &mut alloc, &mut pdf, b"I2");

    let w_2 = (image_2.width / 6) as f32;
    let h_2 = (image_2.height / 6) as f32;

    let x_2 = (a4.x2 * 0.9) - w_2;
    let y_2 = y;

    let image_3 = prep_image(PROJECT_LOGO_NAME_DATA, &mut alloc, &mut pdf, b"I3");

    let w_3 = (image_3.width / 6) as f32;
    let h_3 = (image_3.height / 6) as f32;

    let x_3 = (a4.x2 - w_3) / 2.0;
    let y_3 = y + (h - h_3) / 2.0;

    let font_id = alloc.bump();
    let font_name = Name(b"F1");

    // Table geometry
    let height = 842.0;
    let cell_width = 240.0;
    let cell_height = 30.0;
    let x_start = 58.0;
    let y_start = height - 168.0;

    // Page 1
    let page_id = alloc.bump();
    page_ids.push(page_id);

    {
        // Add page
        let mut page = pdf.page(page_id);
        page.media_box(a4);
        page.parent(page_tree_id);
        {
            let mut resources = page.resources();
            resources.fonts().pair(font_name, font_id);
            resources
                .x_objects()
                .pair(Name(image_1.name), image_1.id)
                .pair(Name(image_2.name), image_2.id)
                .pair(Name(image_3.name), image_3.id);
        }

        // Content for page
        let mut content = Content::new();

        content.begin_text();
        content.set_font(font_name, 14.0);
        content.set_leading(30.0);
        content.next_line(58.0, 684.0);
        content.show(Str(&format!(
            "Codice stazione: {}",
            anagrafica_hfbi.codice_stazione
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Tipo laguna: {}",
            anagrafica_hfbi.tipo_laguna
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Data campionamento: {}",
            anagrafica_hfbi.date_string
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(
            &format!("Stagione: {}", anagrafica_hfbi.stagione).into_bytes()
        ));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Corpo idrico: {}",
            anagrafica_hfbi.corpo_idrico
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Habitat vegetato: {}",
            anagrafica_hfbi.habitat_vegetato
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Regione: {}",
            anagrafica_hfbi.posizione.regione
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Provincia: {}",
            anagrafica_hfbi.posizione.provincia
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Lunghezza stazione: {}",
            anagrafica_hfbi.get_lunghezza_media()
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Larghezza stazione: {}",
            anagrafica_hfbi.get_larghezza_media()
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Hfbi: {}", valore_hfbi).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("MMI: {}", valore_mmi).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Stato Ecologico: {}", stato_eco).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Bbent: {}", intermediates.bbent).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Bn: {}", intermediates.bn).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Dbent: {}", intermediates.dbent).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Ddom: {}", intermediates.ddom).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Dhzp: {}", intermediates.dhzp).into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Dmig: {}", intermediates.dmig).into_bytes()));
        content.end_text();

        let cols = 2;
        let rows = 18;

        // Horizontal lines
        for row in 0..=rows {
            let y = y_start - (row as f32 * cell_height);
            content.move_to(x_start, y);
            content.line_to(x_start + (cols as f32 * cell_width), y);
            content.stroke();
        }
        draw_image(&mut content, &image_1, x, y, w, h);
        draw_image(&mut content, &image_2, x_2, y_2, w_2, h_2);
        draw_image(&mut content, &image_3, x_3, y_3, w_3, h_3);

        content.move_to(x_start, y - 10.0);
        content.line_to(x_start + (cols as f32 * cell_width), y - 10.0);
        content.stroke();

        content.begin_text();
        content.next_line(a4.x2 / 2.0 - 60.0, y - 30.0);
        content.show(Str(&"Applicazione HFBI".to_string().into_bytes()));

        content.next_line(15.0, -15.0);
        content.show(Str(&"DM 260/2010".to_string().into_bytes()));
        content.end_text();

        content.move_to(x_start, 30.0);
        content.line_to(x_start + (cols as f32 * cell_width), 30.0);
        content.stroke();

        content.begin_text();
        content.next_line(a4.x2 / 2.0 - 115.0, 15.0);
        content.show(Str(&format!(
            "F-value v{}, Data release: {}",
            env!("CARGO_PKG_VERSION"),
            BUILD_DATE
        )
        .into_bytes()));
        content.end_text();

        let content_id = alloc.bump();
        secondary.stream(content_id, &content.finish());
        page.contents(content_id);
    }

    // Specify the font we want to use. Because Helvetica is one of the 14 base
    // fonts shipped with every PDF reader, we don't have to embed any font
    // data.
    pdf.type1_font(font_id).base_font(Name(b"Helvetica"));

    pdf.extend(&secondary);

    // Write the page tree with a single child page.
    pdf.pages(page_tree_id)
        .kids(page_ids.iter().copied())
        .count(page_ids.len() as i32);

    // Write the document catalog with a reference to the page tree.
    pdf.catalog(alloc.bump()).pages(page_tree_id);

    // Finish writing (this automatically creates the cross-reference table and
    // file trailer) and retrieve the resulting byte buffer.
    let buf: Vec<u8> = pdf.finish();

    // Write the thing to a file.
    match std::fs::write(&export_path, buf) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed writing pdf to {}: {}", export_path.display(), e);
        }
    }
}
