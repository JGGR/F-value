use crate::{
    engines::hfbi::{
        bbent::calc_bbent, bn::calc_bn, dbent::calc_dbent, ddom::calc_ddom, dhzp::calc_dhzp,
        dmig::calc_dmig,
    },
};
use crate::domain::hfbi::{
        AnagraficaHFBI, CampionamentoHFBI, CondizioniRiferimentoHFBI, StatoEcologicoHFBI,
        ValoriIntermediHFBI,
};

// WEIGHTS
const W_DDOM: f32 = 1.0;
const W_BN: f32 = 0.7;
const W_DMIG: f32 = 0.05;
const W_BBENT: f32 = 0.82;
const W_DBENT: f32 = 0.37;
const W_DHZP: f32 = 0.84;

// MAGIC CONST
const HFBI_T: f32 = -0.167;
const HFBI_S: f32 = 0.150;

const STATO_ECOLOGICO_HFBI_SOGLIA_ECCELLENTE: f32 = 0.94;
const STATO_ECOLOGICO_HFBI_SOGLIA_BUONO: f32 = 0.55;
const STATO_ECOLOGICO_HFBI_SOGLIA_SUFFICIENTE: f32 = 0.33;
const STATO_ECOLOGICO_HFBI_SOGLIA_SCARSO: f32 = 0.11;

pub fn calculate_mmi(
    campionamento: &CampionamentoHFBI,
    anagrafica: &AnagraficaHFBI,
) -> Result<ValoriIntermediHFBI, String> {
    let condizioni_riferimento = match CondizioniRiferimentoHFBI::get_cond_riferimento(anagrafica) {
        Some(cond) => cond,
        None => return Err(String::from("Errore condizioni di riferimento non trovate")),
    };

    let bbent: f32 = calc_bbent(campionamento, anagrafica);
    let bn: f32 = calc_bn(campionamento);
    let dbent: f32 = calc_dbent(campionamento, anagrafica);
    let ddom: f32 = calc_ddom(campionamento, anagrafica);
    let dhzp: f32 = calc_dhzp(campionamento, anagrafica);
    let dmig: f32 = calc_dmig(campionamento, anagrafica);
    let rqe_bbent = bbent / condizioni_riferimento.bbent;
    let rqe_bn = bn / condizioni_riferimento.bn;
    let rqe_dbent = dbent / condizioni_riferimento.dbent;
    let rqe_ddom = ddom / condizioni_riferimento.ddom;
    let rqe_dhzp = dhzp / condizioni_riferimento.dhzp;
    let rqe_dmig = dmig / condizioni_riferimento.dmig;
    let weighted_rqe_bbent = W_BBENT * rqe_bbent;
    let weighted_rqe_bn = W_BN * rqe_bn;
    let weighted_rqe_dbent = W_DBENT * rqe_dbent;
    let weighted_rqe_ddom = W_DDOM * rqe_ddom;
    let weighted_rqe_dhzp = W_DHZP * rqe_dhzp;
    let weighted_rqe_dmig = W_DMIG * rqe_dmig;

    let weighted_rqe_sum = weighted_rqe_ddom
        + weighted_rqe_bn
        + weighted_rqe_dmig
        + weighted_rqe_bbent
        + weighted_rqe_dbent
        + weighted_rqe_dhzp;
    let weight_sum = W_DDOM + W_BN + W_DMIG + W_BBENT + W_DBENT + W_DHZP;

    let mmi = weighted_rqe_sum / weight_sum;
    let rounded_mmi = (1000.0 * mmi).round() / 1000.0;

    let intermediates = ValoriIntermediHFBI {
        bbent,
        bn,
        dbent,
        ddom,
        dhzp,
        dmig,
        mmi: rounded_mmi,
    };
    Ok(intermediates)
}

pub fn calculate_hfbi(
    campionamento: &CampionamentoHFBI,
    anagrafica: &AnagraficaHFBI,
) -> Result<(f32, ValoriIntermediHFBI), String> {
    match calculate_mmi(campionamento, anagrafica) {
        Ok(intermediates) => {
            let hfbi = (intermediates.mmi + HFBI_T) / HFBI_S;
            let rounded_hfbi = (1000.0 * hfbi).round() / 1000.0;
            Ok((rounded_hfbi, intermediates))
        }
        Err(error) => Err(error),
    }
}

pub fn calculate_stato_ecologico_hfbi(hfbi: Option<f32>) -> Option<StatoEcologicoHFBI> {
    match hfbi {
        Some(val) => {
            if val >= STATO_ECOLOGICO_HFBI_SOGLIA_ECCELLENTE {
                return Some(StatoEcologicoHFBI::Eccellente);
            }
            if val >= STATO_ECOLOGICO_HFBI_SOGLIA_BUONO {
                return Some(StatoEcologicoHFBI::Buono);
            }
            if val >= STATO_ECOLOGICO_HFBI_SOGLIA_SUFFICIENTE {
                return Some(StatoEcologicoHFBI::Sufficiente);
            }
            if val >= STATO_ECOLOGICO_HFBI_SOGLIA_SCARSO {
                return Some(StatoEcologicoHFBI::Scarso);
            }
            Some(StatoEcologicoHFBI::Cattivo)
        }
        None => None,
    }
}

#[cfg(test)]
mod full_hfbi_private_tests {
    // We use the structs from the domain, but NOT the functions from the parent module.
    // By defining our own mock functions with the same names, they will be used instead.
    use crate::domain::hfbi::{
        AnagraficaHFBI, CampionamentoHFBI, GruppoEcoHFBI, GruppoTrofHFBI, HabitatHFBI, RecordHFBI,
        SpecieHFBI, StagioneHFBI, TipoLagunaCostieraHFBI,
    };

    const EPSILON: f32 = 1e-6;

    fn create_specie_record(
        codice_specie: &'static str,
        gruppo_eco: GruppoEcoHFBI,
        peso: f32,
    ) -> RecordHFBI {
        RecordHFBI {
            specie: SpecieHFBI {
                nome_comune: "Test Specie",
                codice_specie: codice_specie,
                autoctono: true,
                gruppo_eco,
                gruppo_trofico: GruppoTrofHFBI {
                    microbentivori: 0.0,
                    macrobentivori: 0.0,
                    iperbentivori: 0.0,
                    erbivori: 0.0,
                    detritivori: 0.0,
                    planctivori: 0.0,
                    onnivori: 0.0,
                },
            },
            numero_individui: 1,
            peso,
        }
    }

    // Bring in the functions we actually want to test from the parent module.
    use super::{calculate_hfbi, calculate_mmi};
    use crate::domain::location::Location; // Assuming path is correct

    // Test helper to create a minimal Anagrafica struct.
    fn create_test_anagrafica(codice_stazione: &str) -> AnagraficaHFBI {
        AnagraficaHFBI {
            codice_stazione: codice_stazione.to_string(),
            corpo_idrico: "TestCorpoIdrico".to_string(),
            posizione: Location {
                regione: "Test".to_string(),
                provincia: "Test".to_string(),
            },
            date_string: "01/01/2025".to_string(),
            tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
            stagione: StagioneHFBI::Primavera,
            habitat_vegetato: HabitatHFBI::NonVegetato,
            lunghezza_media_transetto: 100.0,
            larghezza_media_transetto: 100.0,
        }
    }

    // ===================================================================
    // ===== 2. Tests
    // ===================================================================

    #[test]
    fn test_mmi_and_hfbi() {
        let anagrafica = create_test_anagrafica("OK_STATION_INTEGRATION");
        // We will create a non-empty campione with specific data.
        let campione = CampionamentoHFBI {
            campionamento: vec![
                // Species 1: Migratory, dominant, contributes to all metrics
                create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 500.0),
                // Species 2: Migratory, contributes to most metrics
                create_specie_record("SP2", GruppoEcoHFBI::MigratoriMarini, 200.0),
                // Species 3: Resident, not dominant
                create_specie_record("SP3", GruppoEcoHFBI::ResidentiDiEstuario, 100.0),
            ],
        };

        // We can't know the exact intermediate values without re-implementing all the
        // functions here. But we can ensure the logic flows and produces a valid, finite number.
        let mmi_result = calculate_mmi(&campione, &anagrafica);
        assert!(
            mmi_result.is_ok(),
            "calculate_mmi should succeed with non-empty data"
        );

        let intermediates = mmi_result.unwrap();

        let tested: f32 = 0.552; // ho testato a mano il risultato

        assert!(
            intermediates.mmi.is_finite(),
            "MMI should be a finite number, but was {}",
            intermediates.mmi
        );
        assert!(intermediates.bbent.is_finite(), "bbent should be finite");
        assert!(intermediates.bn.is_finite(), "bn should be finite");
        assert!(intermediates.ddom.is_finite(), "ddom should be finite");
        assert!((intermediates.mmi - tested).abs() < EPSILON);

        // Now, we test the final step with the calculated MMI
        let hfbi_result = calculate_hfbi(&campione, &anagrafica);
        assert!(hfbi_result.is_ok(), "calculate_hfbi should also succeed");

        let (hfbi, _) = hfbi_result.unwrap();
        assert!(
            hfbi.is_finite(),
            "HFBI should be a finite number, but was {}",
            hfbi
        );
    }
}
