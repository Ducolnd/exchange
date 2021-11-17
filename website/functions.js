// These variables are used to keep track of the Order Book, later used for rending
let sells = [];
let buys = [];


// Renders sells and buys to the client
function UpdateUI() {
    sells.sort(compareSellOrder);
    buys.sort(compareSellOrder);

    // Sells
    const sellOrders = sells.map((order) =>
        <Order key={(order.timestamp / 10e9 + order.size + order.price / 10e9).toString()} value={order} />
    );
    let toRender = <div id="sub-sell-orders">{sellOrders}</div>

    ReactDOM.render(
        toRender,
        document.getElementById('sell-orders')
    );


    // Buys
    const buyOrders = buys.map((order) =>
        <Order key={(order.timestamp / 10e9 + order.size + order.price / 10e9).toString()} value={order} />
    );
    toRender = <div id="buy-orders">{buyOrders}</div>

    ReactDOM.render(
        toRender,
        document.getElementById('outer-buy-orders')
    );
}


function RenderTransaction(transaction) {
    let el = $("#list-transactions");
    let color = transaction.sell ? "red" : "#47d600"

    let html = `
    <div class="row">
        <div class="col"><p>${transaction.size}</p></div>
        <div class="col"><p style="color: ${color}">${transaction.price / 10e9}</p></div>
        <div class="col"><p class="date">${(new Date()).toLocaleTimeString().slice(0, -3)}</p></div>
    </div>
    `
    el.prepend(html);
}

/// Updates the Order Book data, does not change UI
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

// 
function HandleOrderBookUpdate(data) {
    let newBuys = data["buy"];
    let newSells = data["sell"];

    updateBook(newSells, newBuys);
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