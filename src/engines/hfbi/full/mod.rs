use crate::{domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI, CondizioniRiferimentoHFBI, ValoriIntermediHFBI}, engines::hfbi::{bbent::calc_bbent, bn::calc_bn, dbent::calc_dbent, ddom::calc_ddom, dhzp::calc_dhzp, dmig::calc_dmig}};




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



pub(crate) fn calculate_mmi(campionamento: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> Result<(f32, ValoriIntermediHFBI), String> {
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


  let weighted_rqe_sum = weighted_rqe_ddom + weighted_rqe_bn + weighted_rqe_dmig + weighted_rqe_bbent + weighted_rqe_dbent + weighted_rqe_dhzp;
  let weight_sum = W_DDOM + W_BN + W_DMIG + W_BBENT + W_DBENT + W_DHZP;

  let mmi = weighted_rqe_sum / weight_sum;

  let intermediates = ValoriIntermediHFBI {
    bbent,
    bn,
    dbent,
    ddom,
    dhzp,
    dmig
  };
  Ok((mmi, intermediates))
}


pub(crate) fn calculate_hfbi(campionamento: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> Result<(f32, ValoriIntermediHFBI), String> {
  match calculate_mmi(campionamento, anagrafica) {
    Ok((mmi, intermediates)) => Ok(((mmi + HFBI_T) / HFBI_S, intermediates)),
    Err(error) => Err(error)
  }
}


