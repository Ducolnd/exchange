let protocol = "ws";
let conn = null;
let action = "sell";

$(document).ready(function() {
    connect();

    $("#submit_form").click(function() {
        let data = {
            size: parseInt(parseFloat($("#size").val())),
            price: parseInt(parseFloat($("#price").val())),
            sell: action === "sell" ? true : false,
        }      
        data = JSON.stringify(data);

        if (protocol == "http") {
            $.ajax({
                url: 'http://127.0.0.1:8080/',
                type: 'post',
                data: data,
                headers: {"Content-Type": "application/json"},
                dataType: 'json',
                success: function (data) {
                    console.info(data);
                }
            });
        } else {
            conn.send(data);
        }
    });

    $("#select_sell").click(function() {
        $(this).attr("class", "btn btn-danger");
        $("#select_buy").attr("class", "btn btn-outline-success");

        action = "sell";
    })

    $("#select_buy").click(function() {
        $(this).attr("class", "btn btn-success");
        $("#select_sell").attr("class", "btn btn-outline-danger");

        action = "buy";
    })

    $("#ws").click(function() {
        connect();
        $(this).css("color", "blue");
        $("#http").css("color", "black");
        protocol = "ws"
    })

    $("#http").click(function() {
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

    conn.onopen = function() {
        console.log("Connected");
    }
    conn.onmessage = function(e) {
        // console.info("Received a message ");
        let data = JSON.parse(e.data);

        let buys = JSON.parse(data[0]);
        let sells = JSON.parse(data[1]);
        // console.log(sells, buys);
        updateUI(sells, buys);
    }
    conn.onclose = function() {
        console.warn("Disconnected from server")
        conn = null;
    }
}

function updateUI(sells, buys) {
    $("#buy-orders").empty();
    for (let buy of buys.reverse()) {
        let element = $(`<div class="row">
                            <div class="col"><p>${buy.size}</p></div>
                            <div class="col"><p>${buy.price}</p></div>
                        </div>`);

        $("#buy-orders").append(element);
    }
    
    $("#sub-sell-orders").empty();
    for (let sell of sells) {
        let element = $(`<div class="row">
                            <div class="col"><p>${sell.size}</p></div>
                            <div class="col"><p>${sell.price}</p></div>
                        </div>`);

        $("#sub-sell-orders").append(element);
    }
}

function disconnect() {
    if (conn != null) {
        console.log('Disconnecting...');
        conn.close();
        conn = null;
    }
}