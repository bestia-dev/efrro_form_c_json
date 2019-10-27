//! **amazonmod - generic HashMap to deserialize the amazon csv**  

#![allow(clippy::needless_pass_by_value, clippy::explicit_iter_loop)]

//region use
use crate::log1;
use crate::rootrenderingmod::RootRenderingComponent;
use crate::isocountriesmod;
use crate::reqbodymod;
use crate::fetchmod;
use crate::rootrenderingmod;

use std::collections::HashMap;
//endregion

pub type Record = HashMap<String, String>;

///extract a vector of order_id to draw buttons for processing only one order
pub fn amazon_txt_create_vector_of_order_id(rrc: &mut RootRenderingComponent, amazon_txt: &str) {
    log1("amazon_txt_create_vector_of_order_id");
    rrc.amazon_txt = amazon_txt.to_string();

    //https://docs.rs/csv/1.0.7/csv/tutorial/index.html#reading-with-serde
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .double_quote(false)
        .flexible(true)
        .from_reader(rrc.amazon_txt.as_bytes());

    // The txt can have many documents. One document can have many products.
    // the order-id is important for more products for one document.
    // process can process only 1 document at a time. It means there must be some counter somewhere where
    // the user can choose which document to process.
    // maybe I can draw as many buttons as there are documents onclick the button amazon_txt process
    // if there is more than one document. If there is only one document that process it.

    //first I need a vector of Records. Then I can do all other manipulations.
    let mut vec_rec: Vec<Record> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = unwrap!(result);
        vec_rec.push(record);
    }
    rrc.vec_rec = vec_rec;

    //now I need a vector of order_id
    let mut vec_order_id: Vec<String> = Vec::new();
    let mut old_order_id = "".to_string();
    for record in &rrc.vec_rec {
        for rec_field in record {
            if rec_field.0 == "order-id" && rec_field.1.clone() != old_order_id {
                vec_order_id.push(rec_field.1.clone());
                old_order_id = rec_field.1.clone();
            }
        }
    }

    rrc.vec_order_id = vec_order_id;
}

/// process one order-id and modify request json
pub fn amazon_single_order_id_process(
    rrc: &mut RootRenderingComponent,
    order_id: &str,
    vdom_weak: dodrio::VdomWeak,
) {
    //ricevuta and ddt are created in the same time.
    //the request body must be reset to the template for replace
    reqbodymod::create_reqbody_ricevute_nuovo(rrc);

    let mut total = 0.0;
    let mut total_qty = 0.0;
    let mut line_count = 0;
    let vec_rec_clone = rrc.vec_rec.clone();
    for record in vec_rec_clone.iter() {
        for rec_fieldx in record {
            //this first loop only finds the right record
            if rec_fieldx.0 == "order-id" && rec_fieldx.1.clone() == order_id {
                //this is the right record.
                //there is at least one product, but prepare the space for the next one. This will be removed at the end.
                rrc.reqbody = rrc.reqbody.replace(
                    "articolo-template",
                    format!("{} \n articolo-template", reqbodymod::articolo_template()).as_str(),
                );

                //loop all the fields and replace. The second, third,... time there will be only articolo-template to replace.
                //All else is already replaced.
                for rec_field in record {
                    //this works only if the field names in CSV are really unique and unmistakable
                    log1("replace");
                    log1(rec_field.0.as_str());
                    log1(rec_field.1.as_str());
                    let field_name = rec_field.0;
                    //if there is a sign " escape it for json to \"
                    let mut field_value = rec_field.1.replace(r#"""#, r#"\""#);

                    //from sku get product-name from webserver, but it is asynchronous
                    //change placeholder from product-name to product-name1, 2,...
                    //so there is uniquness for async
                    if field_name == "sku" {
                        let v2 = vdom_weak.clone();
                        replace_sku_name(line_count, rrc, v2, field_value.to_string());
                    }

                    //convert date from 2019-07-04T11:51:26+00:00 to 23/07/2019
                    if field_name == "purchase-date" || field_name == "payments-date" {
                        field_value = format!(
                            "{}/{}/{}",
                            unwrap!(field_value.get(8..10)).to_string(),
                            unwrap!(field_value.get(5..7)).to_string(),
                            unwrap!(field_value.get(0..4)).to_string()
                        );

                        //the ricevuta and ddt have today-date
                        let today = js_sys::Date::new_0();

                        let dd = today.get_date();
                        #[allow(clippy::integer_arithmetic)]
                        let mm = today.get_month() + 1; //January is 0!
                        let yyyy = today.get_full_year();

                        let today_date = format!("{:02}/{:02}/{}", dd, mm, yyyy);
                        rrc.reqbody = rrc.reqbody.replace("today-date", today_date.as_str());
                    }
                    //find the country name in Italian from ISO code
                    if field_name == "ship-country" {
                        replace_ship_county_dependent(rrc, field_value.to_string(), record);
                    }
                    if field_name == "quantity-purchased" {
                        //from string to decimal
                        let f64_quantity_purchased: f64 = unwrap!(field_value.parse());
                        log1(&format!("{}", f64_quantity_purchased));
                        total_qty += f64_quantity_purchased;
                    }
                    if field_name == "item-price" {
                        //from string to decimal
                        let f64_item_price: f64 = unwrap!(field_value.parse());
                        log1(&format!("{}", f64_item_price));
                        total += f64_item_price;
                    }
                    if field_name == "shipping-price" {
                        //from string to decimal
                        let f64_shipping_price: f64 = unwrap!(field_value.parse());
                        log1(&format!("{}", f64_shipping_price));
                        //only the first time
                        if line_count == 0 {
                            total += f64_shipping_price;
                        }
                    }
                    rrc.reqbody = rrc.reqbody.replace(field_name.as_str(), &field_value);
                }
                line_count += 1;
            }
        }
    }
    log1(&format!("total {}", total));
    //the total
    rrc.reqbody = rrc
        .reqbody
        .replace("total-importo", format!("{:.2}", total).as_str());
    //total quantity
    rrc.reqbody = rrc
        .reqbody
        .replace("total-quantity", format!("{:.0}", total_qty).as_str());

    //remove the articolo-template
    rrc.reqbody = rrc.reqbody.replace("articolo-template", "");
}

///replace sku name and find product-name from webservice
pub fn replace_sku_name(
    line_count: usize,
    rrc: &mut RootRenderingComponent,
    vdom_weak: dodrio::VdomWeak,
    field_value: String,
) {
    //prepare a request and call json.
    //at the end replace the product-nameX
    let placeholder = format!("product--name{}", line_count);
    rrc.reqbody = rrc.reqbody.replace("product-name", placeholder.as_str());

    let mut reqbody = format!(
        r#"{{
                        "api_uid":"{}",
                        "api_key":"{}",
                        "#,
        rrc.api_uid, rrc.api_key
    );

    reqbody.push_str(
        format!(
            r#"
                        "filtro": "",
                        "id": "",
                        "nome": "",
                        "cod": "{}",
                        "desc": "",
                        "categoria": "",
                        "pagina": 1
                        }}"#,
            field_value
        )
        .as_str(),
    );
    log1(format!("sku {} req {}", field_value, reqbody).as_str());

    let webrequest =
        reqbodymod::create_webrequest(reqbody, "https://bestia.dev/v1/prodotti/lista".to_string());
    fetchmod::fetch_response(
        vdom_weak,
        &webrequest,
        placeholder,
        &rootrenderingmod::replace_product_name,
    );
}

///some data are dependent of the ship_country
pub fn replace_ship_county_dependent(
    rrc: &mut RootRenderingComponent,
    field_value: String,
    record: &Record,
) {
    let country_name = if field_value == "IT" {
        "Italia".to_string()
    } else {
        isocountriesmod::get_country_name(&field_value)
    };
    rrc.reqbody = rrc.reqbody.replace("ship-country-name", &country_name);

    let market_lang = if field_value == "IT" {
        "it".to_string()
    } else {
        "en".to_string()
    };
    rrc.reqbody = rrc.reqbody.replace("market-lang", &market_lang);

    let vendita_or_sale = if field_value == "IT" {
        "vendita".to_string()
    } else {
        "sale".to_string()
    };
    rrc.reqbody = rrc.reqbody.replace("vendita-or-sale", &vendita_or_sale);

    let trasportatore = if field_value == "IT" {
        "GLS".to_string()
    } else {
        //SPRING se la spedizione costa 11 euro. DHL se costa 16€
        let mut xx = "".to_string();
        for rf2 in record {
            if rf2.0 == "shipping-price" {
                //from string to decimal
                let f64_shipping_price: f64 = unwrap!(rf2.1.parse());
                //floats are difficult to be equal. It is better a comparison.
                if f64_shipping_price < 14.0 {
                    xx = "SPRING".to_string();
                } else {
                    xx = "DHL".to_string();
                }
            }
        }
        //return from block
        xx
    };
    rrc.reqbody = rrc.reqbody.replace("GLS-or-SPRING-or-DHL", &trasportatore);
    //cod-spedizione
    let cod_spedizione = if trasportatore == "GLS" {
        "SHIP AMA"
    } else if trasportatore == "SPRING" {
        "SHIP SPR"
    } else if trasportatore == "DHL" {
        "SHIP DHL"
    } else {
        ""
    };
    rrc.reqbody = rrc.reqbody.replace("cod-spedizione", cod_spedizione);
    //in the future, the name-spedizioni could be fetch from webservice, but now it is not a priority.
    //name-spedizione
    let name_spedizione = if trasportatore == "GLS" {
        "Spedizione Italia - AMAZON"
    } else if trasportatore == "SPRING" {
        "Spedizione Int. / Int. Shipping - SPRING"
    } else if trasportatore == "DHL" {
        "Spedizione Int. / Int. Shipping - DHL"
    } else {
        ""
    };
    rrc.reqbody = rrc.reqbody.replace("name-spedizione", name_spedizione);
}
