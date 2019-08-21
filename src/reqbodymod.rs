//! **reqbodymod - all request body are created in this module**  
//region description
//! ## JSON
//! fattureInCloud.it receives all requests as POST and all the data is
//! in the body as a simple json text
//endregion

#![allow(clippy::needless_pass_by_value)]

//region use
use crate::log1;
use crate::rootrenderingmod::RootRenderingComponent;

use web_sys::{Request, RequestInit, RequestMode};
use wasm_bindgen::JsValue;
//endregion

///create web request from rrc
pub fn create_webrequest_from_rrc(rrc: &RootRenderingComponent) -> web_sys::Request {
  let mut opts = RequestInit::new();
  opts.method("POST");

  opts.mode(RequestMode::Cors);
  opts.body(Some(&JsValue::from_str(&rrc.reqbody)));

  let w_webrequest = unwrap!(Request::new_with_str_and_init(&rrc.url, &opts));
  //unwrap!(request.headers().set("Content-Type", "text/plain"));

  log1("before fetch_with_webrequest");
  //return
  w_webrequest
}

///create web request from string
pub fn create_webrequest(reqbody: String, url: String) -> web_sys::Request {
  let mut opts = RequestInit::new();
  opts.method("POST");

  opts.mode(RequestMode::Cors);
  opts.body(Some(&JsValue::from_str(&reqbody)));

  let w_webrequest = unwrap!(Request::new_with_str_and_init(url.as_str(), &opts));
  //unwrap!(request.headers().set("Content-Type", "text/plain"));

  log1("before fetch_with_webrequest");
  //return
  w_webrequest
}

///create reqbody for info
pub fn create_reqbody_info(rrc: &mut RootRenderingComponent) {
  rrc.url = "http://34.87.17.103/v1/richiesta/info".to_string();
  rrc.tipo_doc = "".to_string();
  rrc.amazon_txt_disabled = "w3-disabled".to_string();
  rrc.reqbody = format!(
    r#"{{
"api_uid":"{}", 
"api_key":"{}"
}}"#,
    rrc.api_uid, rrc.api_key
  );

  rrc.respbody = "".to_string();
}
/*
///create reqbody for ricevute lista
pub fn create_reqbody_ricevute_lista(rrc: &mut RootRenderingComponent) {
  rrc.url = "http://34.87.17.103/v1/ricevute/lista".to_string();
  rrc.tipo_doc = "ricevute".to_string();
  rrc.amazon_txt_disabled = "w3-disabled".to_string();
  rrc.reqbody = format!(
    r#"{{
"api_uid":"{}",
"api_key":"{}",
"#,
    rrc.api_uid, rrc.api_key
  );

  rrc.reqbody.push_str(
    r#"
"anno": 2019,
"data_inizio": "01/01/2019",
"data_fine": "31/12/2019",
"cliente": "",
"fornitore": "",
"id_cliente": "",
"id_fornitore": "",
"saldato": "",
"oggetto": "order-id",
"ogni_ddt": "",
"PA": false,
"PA_tipo_cliente": "",
"pagina": 1
}"#,
  );
  rrc.respbody = "".to_string();
}
*/
/*
///create reqbody for prodotti lista
pub fn create_reqbody_prodotti_lista(rrc: &mut RootRenderingComponent) {
  rrc.url = "http://34.87.17.103/v1/prodotti/lista".to_string();
  rrc.tipo_doc = "prodotti".to_string();
  rrc.amazon_txt_disabled = "w3-disabled".to_string();
  rrc.reqbody = format!(
    r#"{{
"api_uid":"{}",
"api_key":"{}",
"#,
    rrc.api_uid, rrc.api_key
  );

  rrc.reqbody.push_str(
    r#"
  "filtro": "",
  "id": "",
  "nome": "",
  "cod": "DOC133",
  "desc": "",
  "categoria": "",
  "pagina": 1
}"#,
  );
  rrc.respbody = "".to_string();
}
*/

///create reqbody for ricevute nuovo
pub fn create_reqbody_ricevute_nuovo(rrc: &mut RootRenderingComponent) {
  rrc.url = "http://34.87.17.103/v1/ricevute/nuovo".to_string();
  rrc.tipo_doc = "ricevute".to_string();
  rrc.amazon_txt_disabled = "".to_string();
  rrc.reqbody = format!(
    r#"{{
"api_uid":"{}", 
"api_key":"{}",
"#,
    rrc.api_uid, rrc.api_key
  );

  rrc.reqbody.push_str(
    r#"
  "id_cliente": "0",
  "id_fornitore": "0",
  "nome": "recipient-name",
  "indirizzo_via": "ship-address-1 ship-address-2 ship-address-3",
  "indirizzo_cap": "ship-postal-code",
  "indirizzo_citta": "ship-city",
  "indirizzo_provincia": "ship-state",
  "indirizzo_extra": "",
  "paese": "ship-country-name",
  "paese_iso": "ship-country",
  "lingua": "market-lang",
  "piva": "",
  "cf": "",
  "autocompila_anagrafica": false,
  "salva_anagrafica": false,
  "numero": "",
  "data": "today-date",
  "valuta": "currency",
  "valuta_cambio": 1,
  "prezzi_ivati": true,
  "rivalsa": 0,
  "cassa": 0,
  "rit_acconto": 0,
  "imponibile_ritenuta": 0,
  "rit_altra": 0,
  "marca_bollo": 0,
  "oggetto_visibile": "order-id",
  "oggetto_interno": "",
  "centro_ricavo": "sales-channel",
  "centro_costo": "",
  "note": "",
  "nascondi_scadenza": false,
  "ddt": true,
  "ftacc": false,
  "id_template": "0",
  "ddt_id_template": "0",
  "ftacc_id_template": "0",
  "mostra_info_pagamento": false,
  "metodo_pagamento": "Bonifico",
  "metodo_titoloN": "IBAN",
  "metodo_descN": "",
  "mostra_totali": "tutti",
  "mostra_bottone_paypal": false,
  "mostra_bottone_bonifico": false,
  "mostra_bottone_notifica": false,
  "lista_articoli": [
    articolo-template
    {
      "id": "0",
      "codice": "cod-spedizione",
      "nome": "name-spedizione",
      "um": "",
      "quantita": 1,
      "descrizione": "",
      "categoria": "Spedizioni",
      "prezzo_netto": 0,
      "prezzo_lordo": shipping-price,
      "cod_iva": 0,
      "tassabile": true,
      "sconto": 0,
      "applica_ra_contributi": true,
      "ordine": null,
      "sconto_rosso": 0,
      "in_ddt": true,
      "magazzino": true
    }
  ],
  "lista_pagamenti": [
    {
      "data_scadenza": "payments-date",
      "importo": total-importo,
      "metodo": "not",
      "data_saldo": "payments-date"
    }
  ],
  "ddt_numero": null,
  "ddt_data": "today-date",
  "ddt_colli": "1",
  "ddt_peso": "total-quantity kg",
  "ddt_causale": "vendita-or-sale",
  "ddt_luogo": "ship-address-1 ship-address-2 ship-address-3 ship-postal-code ship-city ship-state ship-country-name",
  "ddt_trasportatore": "GLS-or-SPRING-or-DHL",
  "ddt_annotazioni": "ship-phone-number",
  "PA": false,
  "PA_tipo_cliente": "PA",
  "PA_tipo": "nessuno",
  "PA_numero": "",
  "PA_data": "",
  "PA_cup": "",
  "PA_cig": "",
  "PA_codice": "",
  "PA_pec": "",
  "PA_esigibilita": "N",
  "PA_modalita_pagamento": "MP01",
  "PA_istituto_credito": "",
  "PA_iban": "",
  "PA_beneficiario": "",
  "extra_anagrafica": {
    "mail": "buyer-email",
    "tel": "ship-phone-number",
    "fax": ""
  },
  "split_payment": true
}"#,
  );

  rrc.respbody = "".to_string();
}

///get articolo template
pub fn articolo_template() -> String {
  r#"
    {
      "id": "0",
      "codice": "sku",
      "nome": "product-name",
      "um": "",
      "quantita": quantity-purchased,
      "descrizione": "",
      "categoria": "",
      "prezzo_netto": 0,
      "prezzo_lordo": item-price,
      "cod_iva": 0,
      "tassabile": true,
      "sconto": 0,
      "applica_ra_contributi": true,
      "ordine": null,
      "sconto_rosso": 0,
      "in_ddt": true,
      "magazzino": true
    },
    "#
  .to_string()
}
