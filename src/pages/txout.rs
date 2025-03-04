use bitcoin::OutPoint;
use maud::{html, Markup};

use crate::{network, req::ParsedRequest, rpc::txout::TxOutJson, NetworkExt};

use super::html_page;

pub fn page(tx: &TxOutJson, outpoint: OutPoint, parsed: &ParsedRequest) -> Markup {
    let txid = format!("{:x}", outpoint.txid);
    let outpoint = html! { code { u { (&txid) } ":" (outpoint.vout) } };
    let link = format!("{}t/{}", network().as_url_path(), &txid);
    let is_spent = if tx.utxos.is_empty() {
        "SPENT"
    } else {
        "UNSPENT"
    };

    let content = html! {
        section {
            hgroup {
                h1 { "Transaction output " }
                p { a href=(link) { (outpoint) } }
            }

            h2 { (is_spent) }

        }
    };

    html_page("Txout", content, parsed)
}
