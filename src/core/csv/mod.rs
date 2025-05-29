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

pub(crate) mod deser;
pub(crate) mod parser;

// This must be kept aligned with RecordCsvRiferimentoNISECI definition.
// TODO: get this stuff with some macro?
pub(crate) const RIFERIMENTO_NISECI_HEADER_FIELDS: [&str; 17] = [ "nomeComune", "nomeLatino", "codiceSpecie", "origine", "tipoAutoctono", "alloNocivita", "specieAttesa", "clSoglia1", "clSoglia2", "clSoglia3", "clSoglia4", "adJuvSoglia1", "adJuvSoglia2", "adJuvSoglia3", "adJuvSoglia4", "densSoglia1", "densSoglia2" ];
pub(crate) const RIFERIMENTO_NISECI_HEADER_FIELD_TYPES: [&str; 17] = [ "String", "String", "String", "String", "u32", "u32", "u32", "u32", "u32", "u32", "u32", "f32", "f32", "f32", "f32", "f32", "f32" ];
pub(crate) const RIFERIMENTO_NISECI_HEADER: &str = "\
nomeComune;nomeLatino;codiceSpecie;origine;tipoAutoctono;alloNocivita;specieAttesa;clSoglia1;clSoglia2;clSoglia3;clSoglia4;adJuvSoglia1;adJuvSoglia2;adJuvSoglia3;adJuvSoglia4;densSoglia1;densSoglia2";

// This must be kept aligned with RecordCsvCampionamentoNISECI definition.
// TODO: get this stuff with some macro?
pub(crate) const CAMPIONAMENTO_NISECI_HEADER_FIELDS: [&str; 6] = [ "data", "stazione", "numPassaggio", "codiceSpecie", "lunghezza", "peso" ];
pub(crate) const CAMPIONAMENTO_NISECI_HEADER_FIELD_TYPES: [&str; 6] = [ "String", "String", "u32", "String", "u32", "u32" ];
pub(crate) const CAMPIONAMENTO_NISECI_HEADER: &str = "\
data;stazione;numPassaggio;codiceSpecie;lunghezza;peso";

// This must be kept aligned with RecordCsvAnagraficaNISECI definition.
// TODO: get this stuff with some macro?
pub(crate) const ANAGRAFICA_NISECI_HEADER_FIELDS: [&str; 13] = [
"codiceStazione", "corpoIdrico", "regione", "provincia", "data", "lunghezzaStazione", "larghezzaStazione", "tipoComunita", "fonte", "numeroProtocollo", "idroEcoRegione", "areaAlpina", "nomeBacino" ];
pub(crate) const ANAGRAFICA_NISECI_HEADER_FIELD_TYPES: [&str; 13] = [ "String", "String", "String", "String", "String", "f32", "f32", "u32", "String", "String", "u32", "u32", "String"];
pub(crate) const ANAGRAFICA_NISECI_HEADER: &str = "\
codiceStazione;corpoIdrico;regione;provincia;data;lunghezzaStazione;larghezzaStazione;tipoComunita;fonte;numeroProtocollo;idroEcoRegione;areaAlpina;nomeBacino";

#[derive(Copy,Clone)]
pub(crate) enum TipoRecordCsv {
    RiferimentoNISECI,
    CampionamentoNISECI,
    AnagraficaNISECI,
}

pub(crate) trait RecordCsvRiferimentoNISECI: serde::de::DeserializeOwned {
    #[allow(dead_code)]
    fn nome_comune(&self) -> String;
    fn nome_latino(&self) -> String;
    fn codice_specie(&self) -> String;
    fn origine(&self) -> String;
    fn tipo_autoctono(&self) -> u32;
    fn allo_nocivita(&self) -> u32;
    fn specie_attesa(&self) -> u32;
    fn cl_soglia1(&self) -> u32;
    fn cl_soglia2(&self) -> u32;
    fn cl_soglia3(&self) -> u32;
    fn cl_soglia4(&self) -> u32;
    fn ad_juv_soglia1(&self) -> f32;
    fn ad_juv_soglia2(&self) -> f32;
    fn ad_juv_soglia3(&self) -> f32;
    fn ad_juv_soglia4(&self) -> f32;
    fn dens_soglia1(&self) -> f32;
    fn dens_soglia2(&self) -> f32;
}

pub(crate) trait RecordCsvCampionamentoNISECI: serde::de::DeserializeOwned {
    #[allow(dead_code)]
    fn data(&self) -> String;
    #[allow(dead_code)]
    fn stazione(&self) -> String;
    fn num_passaggio(&self) -> u32;
    fn codice_specie(&self) -> String;
    fn lunghezza(&self) -> u32;
    fn peso(&self) -> u32;
}

pub(crate) trait RecordCsvAnagraficaNISECI: serde::de::DeserializeOwned {
    fn codice_stazione(&self) -> String;
    fn corpo_idrico(&self) -> String;
    fn regione(&self) -> String;
    fn provincia(&self) -> String;
    fn data(&self) -> String;
    fn lunghezza_stazione(&self) -> f32;
    fn larghezza_stazione(&self) -> f32;
    fn tipo_comunita(&self) -> u32;
    fn fonte(&self) -> String;
    fn numero_protocollo(&self) -> String;
    fn idro_eco_regione(&self) -> u32;
    fn area_alpina(&self) -> u32;
    fn nome_bacino(&self) -> String;
}
