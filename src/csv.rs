use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct UbsTransactionRecord {
    #[serde(rename = "Date de transaction")]
    pub date: NaiveDate,

    #[serde(rename = "Débit", with = "rust_decimal::serde::float_option")]
    pub debit: Option<Decimal>,

    #[serde(rename = "Crédit", with = "rust_decimal::serde::float_option")]
    pub credit: Option<Decimal>,

    #[serde(rename = "Monnaie")]
    pub currency: String,

    #[serde(rename = "Description1")]
    pub payee: String,

    #[serde(rename = "Description2")]
    pub description: String,
}

pub fn read_ubs_csv(data: &str) -> Result<Vec<UbsTransactionRecord>> {
    let data = data
        .split_once("\n\n")
        .context("error: expected UBS CSV data to have a split (`\\n\\n`) between metadata and transactions, but it was not found.")?
        .1;
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(data.as_bytes());
    reader
        .deserialize()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_read_csv_with_metadata() {
        let data = r#"Numéro de compte:;0243 00517049.40;
IBAN:;CH18 0024 3243 5170 4940 F;
Du:;2025-03-01;
Au:;2025-03-14;
Solde initial:;86124.47;
Solde final:;84298.77;
Évaluation en:;CHF;
Nombre de transactions dans cette période:;27;

Date de transaction;Heure de transaction;Date de comptabilisation;Date de valeur;Monnaie;Débit;Crédit;Sous-montant;Solde;N° de transaction;Description1;Description2;Description3;Notes de bas de page;
2025-03-14;;2025-03-14;2025-03-14;CHF;2.40;;;39942.6;1234567890123456;"Bing bong ullabong";"Haha";;;
2025-03-13;;2025-03-13;2025-03-13;CHF;;60.00;;39940.2;0000067890123456;"Dun dun dun";"Hoho";;;"#;

        let result = read_ubs_csv(data).unwrap();

        let expected = vec![
            UbsTransactionRecord {
                date: NaiveDate::from_ymd_opt(2025, 3, 14).unwrap(),
                debit: Some(Decimal::from_str_radix("2.4", 10).unwrap()),
                credit: None,
                currency: "CHF".to_string(),
                payee: "Bing bong ullabong".to_string(),
                description: "Haha".to_string(),
            },
            UbsTransactionRecord {
                date: NaiveDate::from_ymd_opt(2025, 3, 13).unwrap(),
                debit: None,
                credit: Some(Decimal::from_str_radix("60", 10).unwrap()),
                currency: "CHF".to_string(),
                payee: "Dun dun dun".to_string(),
                description: "Hoho".to_string(),
            },
        ];
        assert_eq!(expected, result);
    }
}
