#[derive(Debug)]
pub enum TransactionType { // тип транзакции: `DEPOSIT`, `TRANSFER`, или `WITHDRAWAL`
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
}

#[derive(Debug)]
pub enum TransactionStatus { // состояние транзакции: `SUCCESS`, `FAILURE`, или `PENDING`
    SUCCESS,
    FAILURE,
    PENDING,
}

#[derive(Debug)]
pub struct Transaction {
    tx_id: u64,                 // неотрицательное целое число, идентифицирующее транзакцию
    tx_type: TransactionType,   // тип транзакции: `DEPOSIT`, `TRANSFER`, или `WITHDRAWAL`
    from_user_id: u64,          // неотрицательное целое число, идентифицирующее отправитель счета
    to_user_id: u64,            // неотрицательное целое число, идентифицирующее получателя счета
    amount: u64,                // неотрицательное целое число, представляющее сумму в наименьшей единице валюты
    timestamp: u64,             // Unix epoch timestamp в миллисекундах
    status: TransactionStatus,  // состояние транзакции: `SUCCESS`, `FAILURE`, или `PENDING`
    description: String,        // произвольное текстовое описание, UTF-8
}

pub struct Storage {
   pub transactions: Vec<Transaction>,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
