use std::collections::HashMap;

let alp_num_enc = HashMap.from([
    ('0',0),  ('1',1),  ('2',2),  ('3',3),  ('4',4),
    ('5',5),  ('6',6),  ('7',7),  ('8',8),  ('9',9),
    ('A',10), ('B',11), ('C',12), ('D',13), ('E',14),
    ('F',15), ('G',16), ('H',17), ('I',18), ('J',19),
    ('K',20), ('L',21), ('M',22), ('N',23), ('O',24),
    ('P',25), ('Q',26), ('R',27), ('S',28), ('T',29),
    ('U',30), ('V',31), ('W',32), ('X',33), ('Y',34),
    ('Z',35), (' ',36), ('$',37), ('%',38), ('*',39),
    ('+',40), ('-',41), ('.',42), ('/',43), (':',44),
]);

let num_enc = HashMap.from([
    ('0',0),  ('1',1),  ('2',2),  ('3',3),  ('4',4),
    ('5',5),  ('6',6),  ('7',7),  ('8',8),  ('9',9),
])

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum EncodingMode {
    Numeric,
    Alphanumeric,
    Byte,
    ECI,
    Kanji,
}

impl EncodingMode {
    pub fn get_mode_indicator(&self) -> u8 {
        match self {
            EncodingMode::Numeric      => 1,
            EncodingMode::Alphanumeric => 2,
            EncodingMode::Byte         => 4,
            EncodingMode::ECI          => 4,
            EncodingMode::Kanji        => 8,
        }
    }
}
