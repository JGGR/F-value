// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2025 jgabaut, gioninjo

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

use crate::domain::hfbi::{AnagraficaHFBI, RisultatoHFBI};
use crate::domain::niseci::{AnagraficaNISECI, RiferimentoNISECI, RisultatoNISECI};
use crate::engines::niseci::full::calculate_stato_ecologico;
use crate::app::core::{CISBA_LOGO_DATA, ISPRA_LOGO_DATA};
use image::{ColorType, GenericImageView, ImageFormat};
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::{Chunk, Content, Filter, Finish, Name, Pdf, Rect, Ref, Str};
use std::path::PathBuf;

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
    let stato_eco_niseci =
        match calculate_stato_ecologico(risultato_niseci.get_valore(), &anagrafica_niseci.area) {
            Some(v) => &format!("{}", v),
            None => "NC",
        };

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

    let image_id = alloc.bump();
    let image_name = Name(b"I1");

    let s_mask_id = alloc.bump();

    // Decode the image.
    let format = image::guess_format(&ISPRA_LOGO_DATA).unwrap();
    let dynamic = image::load_from_memory(&ISPRA_LOGO_DATA).unwrap();

    let (filter, encoded, mask) = match format {
        // A JPEG is already valid DCT-encoded data.
        ImageFormat::Jpeg => {
            assert!(dynamic.color() == ColorType::Rgb8);
            (Filter::DctDecode, ISPRA_LOGO_DATA.to_vec(), None)
        }

        // While PNGs uses deflate internally, we need to re-encode to get just
        // the raw coded samples without metadata. Also, we need to encode the
        // RGB and alpha data separately.
        ImageFormat::Png => {
            let level = CompressionLevel::DefaultLevel as u8;
            let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

            // If there's an alpha channel, extract the pixel alpha values.
            let mask = dynamic.color().has_alpha().then(|| {
                let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
                compress_to_vec_zlib(&alphas, level)
            });

            (Filter::FlateDecode, encoded, mask)
        }

        // You could handle other image formats similarly or just recode them to
        // JPEG or PNG, whatever best fits your use case.
        _ => panic!("unsupported image format"),
    };

    // Write the stream for the image we want to embed.
    {
        let mut image = pdf.image_xobject(image_id, &encoded);
        image.filter(filter);
        image.width(dynamic.width() as i32);
        image.height(dynamic.height() as i32);
        image.color_space().device_rgb();
        image.bits_per_component(8);
        if mask.is_some() {
            image.s_mask(s_mask_id);
        }
        image.finish();
    }

    {
        // Add SMask if the image has transparency.
        if let Some(encoded) = &mask {
            let mut s_mask = pdf.image_xobject(s_mask_id, encoded);
            s_mask.filter(filter);
            s_mask.width(dynamic.width() as i32);
            s_mask.height(dynamic.height() as i32);
            s_mask.color_space().device_gray();
            s_mask.bits_per_component(8);
        }
    }

    let a4 = Rect::new(0.0, 0.0, 595.0, 842.0);

    // Size the image at 1pt per pixel.
    let w = (dynamic.width() / 8) as f32;
    let h = (dynamic.height() / 8) as f32;

    // Center the image on the page.
    let x = 205.0; //(a4.x2 - w) / 2.0;
    let y = 742.0; //(a4.y2 - h) / 2.0;

    let image_id_2 = alloc.bump();
    let image_name_2 = Name(b"I2");

    let s_mask_id_2 = alloc.bump();

    // Decode the image.
    let format_2 = image::guess_format(&CISBA_LOGO_DATA).unwrap();
    let dynamic_2 = image::load_from_memory(&CISBA_LOGO_DATA).unwrap();

    let (filter_2, encoded_2, mask_2) = match format_2 {
        // A JPEG is already valid DCT-encoded data.
        ImageFormat::Jpeg => {
            assert!(dynamic_2.color() == ColorType::Rgb8);
            (Filter::DctDecode, CISBA_LOGO_DATA.to_vec(), None)
        }

        // While PNGs uses deflate internally, we need to re-encode to get just
        // the raw coded samples without metadata. Also, we need to encode the
        // RGB and alpha data separately.
        ImageFormat::Png => {
            let level = CompressionLevel::DefaultLevel as u8;
            let encoded = compress_to_vec_zlib(dynamic_2.to_rgb8().as_raw(), level);

            // If there's an alpha channel, extract the pixel alpha values.
            let mask = dynamic_2.color().has_alpha().then(|| {
                let alphas: Vec<_> = dynamic_2.pixels().map(|p| (p.2).0[3]).collect();
                compress_to_vec_zlib(&alphas, level)
            });

            (Filter::FlateDecode, encoded, mask)
        }

        // You could handle other image formats similarly or just recode them to
        // JPEG or PNG, whatever best fits your use case.
        _ => panic!("unsupported image format"),
    };

    // Write the stream for the image we want to embed.
    {
        let mut image = pdf.image_xobject(image_id_2, &encoded_2);
        image.filter(filter_2);
        image.width(dynamic_2.width() as i32);
        image.height(dynamic_2.height() as i32);
        image.color_space().device_rgb();
        image.bits_per_component(8);
        if mask.is_some() {
            image.s_mask(s_mask_id_2);
        }
        image.finish();
    }

    {
        // Add SMask if the image has transparency.
        if let Some(encoded) = &mask_2 {
            let mut s_mask = pdf.image_xobject(s_mask_id_2, encoded);
            s_mask.filter(filter_2);
            s_mask.width(dynamic_2.width() as i32);
            s_mask.height(dynamic_2.height() as i32);
            s_mask.color_space().device_gray();
            s_mask.bits_per_component(8);
        }
    }

    // Size the image at 1pt per pixel.
    let w_2 = (dynamic_2.width() / 8) as f32;
    let h_2 = (dynamic_2.height() / 8) as f32;

    // Center the image on the page.
    let x_2 = x + w + w_2; //(a4.x2 - w) / 2.0;
    let y_2 = y; //(a4.y2 - h) / 2.0;

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
            resources.x_objects().pair(image_name, image_id).pair(image_name_2, image_id_2);
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
        content.show(Str(
            &format!("Data: {}", anagrafica_niseci.date_string).into_bytes()
        ));
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
            "Lunghezza media stazione: {}",
            anagrafica_niseci.lunghezza_media_stazione
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Larghezza media stazione: {}",
            anagrafica_niseci.larghezza_media_stazione
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
        content.end_text();

        let cols = 2;
        let rows = 13;

        // Horizontal lines
        for row in 0..=rows {
            let y = y_start - (row as f32 * cell_height);
            content.move_to(x_start, y);
            content.line_to(x_start + (cols as f32 * cell_width), y);
            content.stroke();
        }

        content.save_state();
        content.transform([w, 0.0, 0.0, h, x, y]);
        content.x_object(image_name);
        content.restore_state();

        content.save_state();
        content.transform([w_2, 0.0, 0.0, h_2, x_2, y_2]);
        content.x_object(image_name_2);
        content.restore_state();

        content.move_to(x_start, y);
        content.line_to(x_start + (cols as f32 * cell_width), y);
        content.stroke();

        content.begin_text();
        content.next_line(a4.x2 / 2.0 - 30.0, y - 15.0);
        content.show(Str(&format!("Applicazione NISECI").into_bytes()));

        content.next_line(0.0, -15.0);
        content.show(Str(&format!("DM 260/2010").into_bytes()));
        content.end_text();

        content.move_to(x_start, 30.0);
        content.line_to(x_start + (cols as f32 * cell_width), 30.0);
        content.stroke();

        content.begin_text();
        content.next_line(a4.x2 / 2.0 - 45.0, 15.0);
        content.show(Str(&format!("F-value v{}", env!("CARGO_PKG_VERSION")).into_bytes()));
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

    let image_id = alloc.bump();
    let image_name = Name(b"I1");

    let s_mask_id = alloc.bump();

    // Decode the image.
    let format = image::guess_format(&ISPRA_LOGO_DATA).unwrap();
    let dynamic = image::load_from_memory(&ISPRA_LOGO_DATA).unwrap();

    let (filter, encoded, mask) = match format {
        // A JPEG is already valid DCT-encoded data.
        ImageFormat::Jpeg => {
            assert!(dynamic.color() == ColorType::Rgb8);
            (Filter::DctDecode, ISPRA_LOGO_DATA.to_vec(), None)
        }

        // While PNGs uses deflate internally, we need to re-encode to get just
        // the raw coded samples without metadata. Also, we need to encode the
        // RGB and alpha data separately.
        ImageFormat::Png => {
            let level = CompressionLevel::DefaultLevel as u8;
            let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

            // If there's an alpha channel, extract the pixel alpha values.
            let mask = dynamic.color().has_alpha().then(|| {
                let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
                compress_to_vec_zlib(&alphas, level)
            });

            (Filter::FlateDecode, encoded, mask)
        }

        // You could handle other image formats similarly or just recode them to
        // JPEG or PNG, whatever best fits your use case.
        _ => panic!("unsupported image format"),
    };

    // Write the stream for the image we want to embed.
    {
        let mut image = pdf.image_xobject(image_id, &encoded);
        image.filter(filter);
        image.width(dynamic.width() as i32);
        image.height(dynamic.height() as i32);
        image.color_space().device_rgb();
        image.bits_per_component(8);
        if mask.is_some() {
            image.s_mask(s_mask_id);
        }
        image.finish();
    }

    {
        // Add SMask if the image has transparency.
        if let Some(encoded) = &mask {
            let mut s_mask = pdf.image_xobject(s_mask_id, encoded);
            s_mask.filter(filter);
            s_mask.width(dynamic.width() as i32);
            s_mask.height(dynamic.height() as i32);
            s_mask.color_space().device_gray();
            s_mask.bits_per_component(8);
        }
    }

    let a4 = Rect::new(0.0, 0.0, 595.0, 842.0);

    // Size the image at 1pt per pixel.
    let w = (dynamic.width() / 8) as f32;
    let h = (dynamic.height() / 8) as f32;

    // Center the image on the page.
    let x = 205.0; //(a4.x2 - w) / 2.0;
    let y = 742.0; //(a4.y2 - h) / 2.0;

    let image_id_2 = alloc.bump();
    let image_name_2 = Name(b"I2");

    let s_mask_id_2 = alloc.bump();

    // Decode the image.
    let format_2 = image::guess_format(&CISBA_LOGO_DATA).unwrap();
    let dynamic_2 = image::load_from_memory(&CISBA_LOGO_DATA).unwrap();

    let (filter_2, encoded_2, mask_2) = match format_2 {
        // A JPEG is already valid DCT-encoded data.
        ImageFormat::Jpeg => {
            assert!(dynamic_2.color() == ColorType::Rgb8);
            (Filter::DctDecode, CISBA_LOGO_DATA.to_vec(), None)
        }

        // While PNGs uses deflate internally, we need to re-encode to get just
        // the raw coded samples without metadata. Also, we need to encode the
        // RGB and alpha data separately.
        ImageFormat::Png => {
            let level = CompressionLevel::DefaultLevel as u8;
            let encoded = compress_to_vec_zlib(dynamic_2.to_rgb8().as_raw(), level);

            // If there's an alpha channel, extract the pixel alpha values.
            let mask = dynamic_2.color().has_alpha().then(|| {
                let alphas: Vec<_> = dynamic_2.pixels().map(|p| (p.2).0[3]).collect();
                compress_to_vec_zlib(&alphas, level)
            });

            (Filter::FlateDecode, encoded, mask)
        }

        // You could handle other image formats similarly or just recode them to
        // JPEG or PNG, whatever best fits your use case.
        _ => panic!("unsupported image format"),
    };

    // Write the stream for the image we want to embed.
    {
        let mut image = pdf.image_xobject(image_id_2, &encoded_2);
        image.filter(filter_2);
        image.width(dynamic_2.width() as i32);
        image.height(dynamic_2.height() as i32);
        image.color_space().device_rgb();
        image.bits_per_component(8);
        if mask.is_some() {
            image.s_mask(s_mask_id_2);
        }
        image.finish();
    }

    {
        // Add SMask if the image has transparency.
        if let Some(encoded) = &mask_2 {
            let mut s_mask = pdf.image_xobject(s_mask_id_2, encoded);
            s_mask.filter(filter_2);
            s_mask.width(dynamic_2.width() as i32);
            s_mask.height(dynamic_2.height() as i32);
            s_mask.color_space().device_gray();
            s_mask.bits_per_component(8);
        }
    }

    // Size the image at 1pt per pixel.
    let w_2 = (dynamic_2.width() / 8) as f32;
    let h_2 = (dynamic_2.height() / 8) as f32;

    // Center the image on the page.
    let x_2 = x + w + w_2; //(a4.x2 - w) / 2.0;
    let y_2 = y; //(a4.y2 - h) / 2.0;

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
            resources.x_objects().pair(image_name, image_id).pair(image_name_2, image_id_2);
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
        content.show(Str(
            &format!("Data: {}", anagrafica_hfbi.date_string).into_bytes()
        ));
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
            "Lunghezza media stazione: {}",
            anagrafica_hfbi.lunghezza_media_transetto
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!(
            "Larghezza media stazione: {}",
            anagrafica_hfbi.larghezza_media_transetto
        )
        .into_bytes()));
        content.next_line(0.0, -30.0);
        content.show(Str(&format!("Hfbi: {}", valore_hfbi).into_bytes()));
        content.end_text();

        let cols = 2;
        let rows = 10;

        // Horizontal lines
        for row in 0..=rows {
            let y = y_start - (row as f32 * cell_height);
            content.move_to(x_start, y);
            content.line_to(x_start + (cols as f32 * cell_width), y);
            content.stroke();
        }

        content.save_state();
        content.transform([w, 0.0, 0.0, h, x, y]);
        content.x_object(image_name);
        content.restore_state();

        content.save_state();
        content.transform([w_2, 0.0, 0.0, h_2, x_2, y_2]);
        content.x_object(image_name_2);
        content.restore_state();

        content.move_to(x_start, y);
        content.line_to(x_start + (cols as f32 * cell_width), y);
        content.stroke();

        content.begin_text();
        content.next_line(a4.x2 / 2.0 - 30.0, y - 15.0);
        content.show(Str(&format!("Applicazione HFBI").into_bytes()));

        content.next_line(0.0, -15.0);
        content.show(Str(&format!("DM 260/2010").into_bytes()));
        content.end_text();

        content.move_to(x_start, 30.0);
        content.line_to(x_start + (cols as f32 * cell_width), 30.0);
        content.stroke();

        content.begin_text();
        content.next_line(a4.x2 / 2.0 - 45.0, 15.0);
        content.show(Str(&format!("F-value v{}", env!("CARGO_PKG_VERSION")).into_bytes()));
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
