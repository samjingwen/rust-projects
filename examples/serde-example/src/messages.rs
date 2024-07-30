pub const SNAPSHOT: &str = r#"
{
    "channel": "book",
    "type": "snapshot",
    "data": [
        {
            "symbol": "BTC/USD",
            "bids": [
                {
                    "price": 69706.5,
                    "qty": 0.32100000
                },
                {
                    "price": 69706.0,
                    "qty": 0.00040001
                },
                {
                    "price": 69700.7,
                    "qty": 1.14776302
                },
                {
                    "price": 69699.5,
                    "qty": 0.33750000
                },
                {
                    "price": 69699.0,
                    "qty": 1.14779227
                },
                {
                    "price": 69697.1,
                    "qty": 0.00014341
                },
                {
                    "price": 69696.2,
                    "qty": 0.98131002
                },
                {
                    "price": 69696.1,
                    "qty": 0.03585049
                },
                {
                    "price": 69696.0,
                    "qty": 0.08321800
                },
                {
                    "price": 69695.5,
                    "qty": 1.14784874
                }
            ],
            "asks": [
                {
                    "price": 69706.6,
                    "qty": 3.01727247
                },
                {
                    "price": 69707.9,
                    "qty": 0.03585769
                },
                {
                    "price": 69708.6,
                    "qty": 0.08322100
                },
                {
                    "price": 69709.0,
                    "qty": 1.14762866
                },
                {
                    "price": 69710.2,
                    "qty": 0.00010000
                },
                {
                    "price": 69710.4,
                    "qty": 0.10330000
                },
                {
                    "price": 69714.2,
                    "qty": 1.14754286
                },
                {
                    "price": 69718.6,
                    "qty": 0.07946318
                },
                {
                    "price": 69718.7,
                    "qty": 0.33750000
                },
                {
                    "price": 69721.4,
                    "qty": 0.00014328
                }
            ],
            "checksum": 3965253375
        }
    ]
}
    "#;

pub const UPDATE: &str = r#"
{
    "channel": "book",
    "type": "update",
    "data": [
        {
            "symbol": "BTC/USD",
            "bids": [],
            "asks": [
                {
                    "price": 69415.1,
                    "qty": 1.15248846
                }
            ],
            "checksum": 3587791954,
            "timestamp": "2024-07-29T10:19:35.613094Z"
        }
    ]
}
"#;

