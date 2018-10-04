# google-analytics-v4-report-flattener

Converts Google Analytics API v4 reports to flat/delimited data.

This is a wrapper around [this Rust package](https://crates.io/crates/ga-v4-flattener). Credit to [Neon](https://github.com/neon-bindings/neon) for doing the hard work for the Rust-Node bindings.

### `toDelimited(data: string, delimiter: string): Array<string>`
```ts
const { toDelimited } = require('google-analytics-v4-report-flattener')
const data = require('./test.json')

toDelimited(JSON.stringify(data), ",")
[ '"ga:deviceCategory","ga:sessions","ga:bounces"\n"desktop",25,17\n"mobile",2,2\n',
  '"ga:country","ga:sessions","ga:bounces"\n"Azerbaijan",1,0\n"France",18,11\n"Japan",4,4\n"Switzerland",1,1\n"United States",3,3\n' ]
```

### `toFlatJsonString(data: string)`
```ts
const { toDelimited } = require('google-analytics-v4-report-flattener')
const data = require('./test.json')

JSON.parse(toFlatJsonString(JSON.stringify(data)))
[
  [{
      'ga:bounces': 17,
      'ga:deviceCategory': 'desktop',
      'ga:sessions': 25
    },
    {
      'ga:bounces': 2,
      'ga:deviceCategory': 'mobile',
      'ga:sessions': 2
    }
  ],
  [{
      'ga:bounces': 0,
      'ga:country': 'Azerbaijan',
      'ga:sessions': 1
    },
    {
      'ga:bounces': 11,
      'ga:country': 'France',
      'ga:sessions': 18
    },
    {
      'ga:bounces': 4,
      'ga:country': 'Japan',
      'ga:sessions': 4
    },
    {
      'ga:bounces': 1,
      'ga:country': 'Switzerland',
      'ga:sessions': 1
    },
    {
      'ga:bounces': 3,
      'ga:country': 'United States',
      'ga:sessions': 3
    }
  ]
]
```
