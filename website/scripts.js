let protocol = "ws";
let conn = null;

$(document).ready(function() {
    connect();

    $("#transaction").submit(function(e) {
        let data = {
            size: parseInt(parseFloat($("#size").val())),
            price: parseInt(parseFloat($("#price").val())),
            sell: $("#sell:checked").val() === "on" ? false : true,
        }      
        data = JSON.stringify(data);
        e.preventDefault();

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
        updateUI(buys, sells);
    }
    conn.onclose = function() {
        console.warn("Disconnected from server")
        conn = null;
    }
}

function updateUI(sells, buys) {
    $("#buys").empty();
    for (let buy of buys.reverse()) {
        let element = $(`<li style="background-color: lightgreen;" class='list-group-item'>$${buy.price} --- size: ${buy.size}</li>`);
        $("#buys").append(element);
    }
    
    $("#sells").empty();
    for (let sell of sells) {
        let element = $(`<li style="background-color: #ffcccb;" class='list-group-item'>$${sell.price} --- size: ${sell.size}</li>`);
        $("#sells").append(element);
    }
}

function disconnect() {
    if (conn != null) {
        console.log('Disconnecting...');
        conn.close();
        conn = null;
    }
}