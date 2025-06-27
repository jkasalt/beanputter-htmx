use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::Currency;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct UbsTransactionRecord {
    #[serde(rename = "Date de transaction")]
    pub date: NaiveDate,

    #[serde(rename = "Débit")]
    pub debit: Decimal,

    #[serde(rename = "Crédit")]
    pub credit: Decimal,

    #[serde(rename = "Monnaie")]
    pub currency: Currency,

    #[serde(rename = "Description1")]
    pub payee: String,

    #[serde(rename = "Description2")]
    pub description: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_read_csv() {
        let data = r#"
Date de transaction;Heure de transaction;Date de comptabilisation;Date de valeur;Monnaie;Débit;Crédit;Sous-montant;Solde;N° de transaction;Description1;Description2;Description3;Notes de bas de page;
2025-03-14;;2025-03-14;2025-03-14;CHF;2.40;;;39942.6;1234567890123456;"Bing bong ullabong";;"Haha";;
2025-03-13;;2025-03-13;2025-03-13;CHF;;60.00;;39940.2;0000067890123456;"Dun dun dun";;"Hoho";;
            "#;

        let mut reader = csv::Reader::from_reader(data.as_bytes());
        let result: Vec<UbsTransactionRecord> = reader
            .deserialize()
            .collect::<Result<Vec<UbsTransactionRecord>, _>>()
            .unwrap();
        let expected = vec![
            UbsTransactionRecord {
                date: NaiveDate::from_ymd_opt(2025, 3, 14).unwrap(),
                credit: Decimal::from_str_radix("2.4", 10).unwrap(),
                debit: Decimal::ZERO,
                currency: Currency::Chf,
                payee: "Bing bong ullabong".to_string(),
                description: "Haha".to_string(),
            },
            UbsTransactionRecord {
                date: NaiveDate::from_ymd_opt(2025, 3, 13).unwrap(),
                debit: Decimal::from_str_radix("-60", 10).unwrap(),
                credit: Decimal::ZERO,
                currency: Currency::Chf,
                payee: "Dun dun dun".to_string(),
                description: "Hoho".to_string(),
            },
        ];
        assert_eq!(expected, result);
    }
}
