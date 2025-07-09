use anyhow::Result;
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
    use csv::{Reader, ReaderBuilder};

    use super::*;

    #[test]
    fn should_read_csv() {
        let data = r#"
Date de transaction;Heure de transaction;Date de comptabilisation;Date de valeur;Monnaie;Débit;Crédit;Sous-montant;Solde;N° de transaction;Description1;Description2;Description3;Notes de bas de page;
2025-03-14;;2025-03-14;2025-03-14;CHF;2.40;;;39942.6;1234567890123456;"Bing bong ullabong";"Haha";;;
2025-03-13;;2025-03-13;2025-03-13;CHF;;60.00;;39940.2;0000067890123456;"Dun dun dun";"Hoho";;;"#;
        let mut reader: Reader<_> = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(data.as_bytes());
        let result = reader.deserialize().collect::<Result<Vec<_>, _>>().unwrap();
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
