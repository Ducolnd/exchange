const WEBSOCKET_SERVER_URL = "ws://127.0.0.1:8080/ws/"

let conn = null;

function connect() {
    if (!(conn === null)) {
        return
    }

    console.log("Connecting to websocket...");
    conn = new WebSocket(WEBSOCKET_SERVER_URL);

    conn.onerror = function (e) {
        console.error("WebSocket error observed:", e);
    }

    conn.onopen = function () {
        console.log("Connected");
        conn.send(JSON.stringify({type: "RequestEntireBook"}))
    }
    conn.onmessage = function (e) {
        let data = JSON.parse(e.data);

        switch (data.type) {
            case "Transaction":
                RenderTransaction(data);
                break;

            case "OrderBook":
                HandleInitialOrderBook(data);
                AggregateTransactions();
                UpdateUI();
                break;
                
            case "OrderBookUpdate":
                HandleOrderBookUpdate(data);
                AggregateTransactions();
                UpdateUI();
                break;

            default:
                console.log("Error occured, false type")
        }
    }
    conn.onclose = function () {
        console.log("Disconnected from server")
        conn = null;
    }
}


function disconnect() {
    if (conn != null) {
        console.log('Disconnecting...');
        conn.close();
        conn = null;

        sells = [];
        buys = [];
    }
}