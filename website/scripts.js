'use strict';

let protocol = "ws";
let conn = null;
let action = "sell";

let sells = [];
let buys = [];

let prevSells = {};
let prevBuys = {};

$(document).ready(function () {
    connect();

    $("#submit_form").click(function () {
        let data = {
            size: parseInt(parseFloat($("#size").val())),
            price: parseInt(parseFloat($("#price").val())) * 10e9,
            sell: action === "sell" ? true : false,
            type: "Transaction",
        }
        data = JSON.stringify(data);

        if (protocol == "http") {
            $.ajax({
                url: 'http://127.0.0.1:8080/',
                type: 'post',
                data: data,
                headers: { "Content-Type": "application/json" },
                dataType: 'json',
                success: function (data) {
                    console.info(data);
                }
            });
        } else {
            conn.send(data);
        }
    });

    $("#select_sell").click(function () {
        $(this).attr("class", "btn btn-danger");
        $("#select_buy").attr("class", "btn btn-outline-success");

        action = "sell";
    })

    $("#select_buy").click(function () {
        $(this).attr("class", "btn btn-success");
        $("#select_sell").attr("class", "btn btn-outline-danger");

        action = "buy";
    })

    $("#ws").click(function () {
        connect();
        $(this).css("color", "blue");
        $("#http").css("color", "black");
        protocol = "ws"
    })

    $("#http").click(function () {
        disconnect();
        $(this).css("color", "blue");
        $("#ws").css("color", "black");
        protocol = "http"
    })
})

function connect() {
    disconnect();
    var url = "ws://127.0.0.1:8080/ws/"

    conn = new WebSocket(url);
    console.log("Connecting...");

    conn.onopen = function () {
        console.log("Connected");
    }
    conn.onmessage = function (e) {
        let data = JSON.parse(e.data);

        switch (data.type) {
            case "Transaction":
                RenderTransaction(data);
                break;

            case "OrderBook":
                HandleOrderBookUpdate(data);
                UpdateUI();
                break;

            case "OrderBookUpdate":
                HandleOrderBookUpdate(data);
                UpdateUI();
                break;

            default:
                console.log("Error occured, false type")
        }
    }
    conn.onclose = function () {
        console.warn("Disconnected from server")
        conn = null;
    }
}

function RenderTransaction(transaction) {
    let el = $("#list-transactions");
    let color = transaction.sell ? "red" : "#47d600"

    let html = `<div class="row">
    <div class="col"><p>${transaction.size}</p></div>
    <div class="col"><p style="color: ${color}">${transaction.price / 10e9}</p></div>
    <div class="col"><p class="date">${(new Date()).toLocaleTimeString().slice(0, -3)}</p></div>

    </div>`
    el.prepend(html);
}

function HandleOrderBookUpdate(data) {
    let newBuys = data["buy"];
    let newSells = data["sell"];

    console.log("newBuys", newBuys, "newSells", newSells);

    updateBook(newSells, newBuys);

    prevBuys = buys;
    prevSells = sells;
}

class Order extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            order: props.value,
        }
    }

    render() {
        return (
            <div className="row">
                <div className="col"><p>{this.state.order.size}</p></div>
                <div className="col price-col"><p>{this.state.order.price / 10e9}</p></div>
            </div>
        );
    }
}

function UpdateUI() {
    sells.sort(compareSellOrder);
    buys.sort(compareSellOrder);

    // Sells
    const sellOrders = sells.map((order) =>
        <Order key={(order.timestamp / 10e6 + order.size + order.price / 10e9).toString()} value={order} />
    );
    let toRender = <div id="sub-sell-orders">{sellOrders}</div>

    ReactDOM.render(
        toRender,
        document.getElementById('sell-orders')
    );


    // Buys
    const buyOrders = buys.map((order) =>
        <Order key={(order.timestamp / 10e6 + order.size + order.price / 10e9).toString()} value={order} />
    );
    toRender = <div id="buy-orders">{buyOrders}</div>

    ReactDOM.render(
        toRender,
        document.getElementById('outer-buy-orders')
    );
}

function updateBook(newSells, newBuys) {
    // Sells
    if (Object.keys(newSells).length > 0) {
        if (Object.keys(sells).length > 0) {
            if ("delete" in newSells) {

                for (let i = 0; i < newSells["delete"].length; i++) {
                    for (let x = 0; x < sells.length; x++) {
                        if (newSells["delete"][i].timestamp === sells[x].timestamp) {
                            sells.splice(x, 1);
                        }
                    }
                }
            }

            if ("add" in newSells) {
                sells = sells.concat(newSells["add"]);
            }

        } else {
            newSells["add"].sort(compareSellOrder);
            sells = newSells["add"];
        }
    }

    // Buys
    if (Object.keys(newBuys).length > 0) {
        if (Object.keys(buys).length > 0) {
            if ("delete" in newBuys) {

                for (let i = 0; i < newBuys["delete"].length; i++) {
                    for (let x = 0; x < buys.length; x++) {
                        if (newBuys["delete"][i].timestamp === buys[x].timestamp) {
                            buys.splice(x, 1);
                        }
                    }
                }
            }

            if ("add" in newBuys) {
                buys = buys.concat(newBuys["add"]);
            }

        } else {
            newBuys["add"].sort(compareSellOrder);
            buys = newBuys["add"];
            console.log("here")
        }
    }
}

function compareSellOrder(a, b) {
    if (a.price < b.price) {
        return 1;
    }
    else if (a.price > b.price) {
        return -1;
    } else {
        return 0;
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