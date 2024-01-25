<h3 style="color:yellow">Here are some definitions of terms this description uses:</h3>

- **Bids**: The line of buyers as referenced in the article.
  - Array of records. Each record contains quantity and price. Sorted by highest price first.
- **Offers**: The line of sellers as referenced in the article.
  - Array of records. Each record contains quantity and price. Sorted by lowest price first.
- **Order Book**: This is an object that contains the bids and offers data arrays.

This is a Rust program that runs from the terminal, that fetches the order books from CoinBase Pro and Gemini exchanges and prints out the price to buy 10 bitcoin and the price to sell 10 bitcoin.

The exchanges all have JSON REST APIs for fetching order books.

1. **CoinBase Pro**:
   - Docs: <https://docs.pro.coinbase.com/#get-product-order-book>
   - Endpoint for BTC-USD: <https://api.pro.coinbase.com/products/BTC-USD/book?level=2>
2. **Gemini Exchange**:
   - Docs: <https://docs.gemini.com/rest-api/#current-order-book>
   - Endpoint for BTC-USD: <https://api.gemini.com/v1/book/BTCUSD>
3. **Kraken Exchange(Bonus)**:
   - Docs: <https://www.kraken.com/en-us/features/api#get-order-book>
   - Endpoint for BTC-USD: <https://api.kraken.com/0/public/Depth?pair=XBTUSD>

<h3 style="color:yellow">Before to start, install rust</h3>

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If the installation is successful, the following line will appear:

```
Rust is installed now. Great!
```

<h3 style="color:yellow">Then let's execute our program using cargo!</h3>

```bash
git clone https://<your_git_token_here>@github.com/lakefishingman522/RustOrderBook.git
cd RustOrderBook
cargo run
```

This will display follow selection console, then please select at least one market to fetch data from.

![Oops!](./refer/selection.png)

The result is like follow, Thanks for your time!
![Alt Text](./refer/result.png)

## Great!
